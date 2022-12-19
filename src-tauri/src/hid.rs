use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::{Duration, Instant};
use hidapi::{DeviceInfo, HidApi, HidDevice};
use nvml_wrapper::{Device, Nvml};
use sysinfo::{CpuExt, System, SystemExt};
use windows::Win32::System::Com::CoInitialize;
use crate::audio;
use crate::audio::AudioEndpoint;
use crate::error::HidError;
use crate::packet::{Packet, PacketHeader};

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum Volume {
    Up = 0x01,
    Down = 0x00,
}

fn get_volume(application: String) -> Result<f32, anyhow::Error> {
    println!("Getting volume for: {:?}", application);
    let applications = audio::enumerate_applications()?;
    let process = applications.iter().filter(|p| p.name.to_lowercase() == application).nth(0);
    if let Some(process) = process {
        let existing_volume = unsafe{process.volume.GetMasterVolume()?};
        println!("Got volume: {}", existing_volume);
        return Ok(existing_volume);
    }
    Err(anyhow::Error::msg("Cannot find running application!"))
}

fn change_volume(application: String, volume: Volume, force: bool) -> Result<(), anyhow::Error> {
    let applications = audio::enumerate_applications()?;
    let processes = applications.iter().filter(|p| p.name.to_lowercase() == application).collect::<Vec<&AudioEndpoint>>();
    for process in processes {
        let mut existing_volume = unsafe{process.volume.GetMasterVolume()?};
        println!("Volume of {}: {}", application, existing_volume);
        existing_volume = match &volume {
            x if x == &Volume::Up && force => 1.0,
            x if x == &Volume::Down && force => 0.0,
            Volume::Up => existing_volume + 0.05,
            Volume::Down => existing_volume - 0.05,
            _ => 0.00
        };
        if existing_volume > 1.0 {
            existing_volume = 1.0;
        } else if existing_volume < 0.05 {
            existing_volume = 0.0;
        }
        println!("Setting volume of {} to {}", application, existing_volume);
        unsafe { process.volume.SetMasterVolume(existing_volume, std::ptr::null()); }
    }

    Ok(())
}

pub fn start_hid_thread(items: Arc<RwLock<HashMap<u16, String>>>) -> Result<(), anyhow::Error> {
    unsafe {
        CoInitialize(None).unwrap();
    }
    let nvml = Nvml::init()?;
    let gpu = nvml.device_by_index(0)?;
    println!("Printing all available hid devices:");

    loop {
        let api = HidApi::new().expect("Cannot create HidAPI");
        let mut p_device: Option<&DeviceInfo> = None;
        for device in api.device_list() {
            //println!("vendor: {:#?}, product: {:#?}, interface: {}", device.vendor_id(), device.product_id(), device.interface_number());
            if device.vendor_id() == 0xdeaf && device.product_id() == 0x0913 && device.interface_number() == 1 {
                p_device = Some(device);
            }
        }
        if p_device.is_none() {
            println!("Cannot find device!");
            sleep(Duration::from_secs(2));
            continue;
        }

        let device = p_device.unwrap().open_device(&api);
        let device = match device {
            Ok(m) => m,
            Err(_e) => {
                continue;
            }
        };
        //device.set_blocking_mode(false)?;
        match communicate_with_device(&device, &gpu, &items) {
            Err(_e) => continue,
            _ => {}
        }
    }
}

fn communicate_with_device(device: &HidDevice, gpu: &Device, items: &Arc<RwLock<HashMap<u16, String>>>) -> Result<(), HidError> {
    let mut sys = System::new_all();
    let mut now = Instant::now();
    loop {
        let packet = recv_packet(&device);
        match packet {
            Ok(p) => {
                println!("Packet: {:?}", p);
                match p.header {
                    PacketHeader::GetVolume => {
                        println!("Get volume!");
                        let raw = p.raw();
                        let application: u16 = u16::from(raw[1]) << 8 | u16::from(raw[0]);
                        let apps = items.read().unwrap();
                        if apps.contains_key(&application) {
                            let application_title = apps[&application].clone();
                            let volume = get_volume(format!("{}.exe", application_title));
                            let mut app = application_title.as_bytes().to_vec();
                            if volume.is_ok() {
                                let volume = volume.unwrap();
                                // volume packet, append name of string
                                let mut v = ((volume * 100.0).round() as u8).to_be_bytes().to_vec();
                                v.append(&mut app);
                                let volume_packet = Packet::new(PacketHeader::GetVolume, v);
                                send_packet(&device, volume_packet)?;
                            } else {
                                println!("Cannot get volume!");
                                let mut v = (255 as u8).to_be_bytes().to_vec();
                                v.append(&mut app);
                                let volume_packet = Packet::new(PacketHeader::GetVolume, v);
                                send_packet(&device, volume_packet)?;
                            }
                        }

                    }
                    PacketHeader::ForceVolume | PacketHeader::ChangeVolume => {
                        println!("Changing volume!");
                        let raw = p.raw();
                        let application: u16 = u16::from(raw[1]) << 8 | u16::from(raw[0]);
                        let apps = items.read().unwrap();
                        let volume_up_or_down = if raw[2] == Volume::Up as u8 {
                            Volume::Up
                        } else {
                            Volume::Down
                        };
                        if apps.contains_key(&application) {
                            let application_title = format!("{}.exe", apps[&application]);
                            println!("Changing volume {:?} on {}!", volume_up_or_down, application_title);
                            let force = p.header == PacketHeader::ForceVolume;
                            change_volume(application_title, volume_up_or_down, force)?;
                        }
                    },
                    _ => ()
                };
            }
            Err(_) => ()
        }
        if now.elapsed() >= std::time::Duration::from_secs(1) {
            send_stats(&mut sys, &device, &gpu)?;
            println!("Send stats!");
            now = Instant::now();
        }
    }
}

fn send_stats(sys: &mut System, device: &HidDevice, gpu: &Device) -> Result<(), HidError> {
    sys.refresh_cpu();
    sys.refresh_memory();
    sys.refresh_processes();
    let total_mem_percentage = ((sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0).round();
    //println!("Mem Usage: {}%", total_mem_percentage);
    // send packet with mem usage.
    let mem_packet = Packet::new(PacketHeader::MemUsage, (total_mem_percentage as u8).to_be_bytes().to_vec());
    send_packet(device, mem_packet)?;

    let mut total_cpu_usage = 0.0;
    for cpu in sys.cpus() {
        total_cpu_usage += cpu.cpu_usage();
    }
    total_cpu_usage = (total_cpu_usage / sys.cpus().len() as f32).round();
    //println!("CPU Usage: {}%", total_cpu_usage);
    let cpu_packet = Packet::new(PacketHeader::CpuUsage, (total_cpu_usage as u8).to_be_bytes().to_vec());
    send_packet(device, cpu_packet)?;

    let process_len = sys.processes().len();
    //println!("Process Count: {}", process_len);
    let process_len_packet = Packet::new(PacketHeader::ProcessCount, (process_len as u16).to_be_bytes().to_vec());
    send_packet(device, process_len_packet)?;

    let gpu_usage = gpu.utilization_rates()?;
    let gpu_usage_packet = Packet::new(PacketHeader::GpuUtilization, (gpu_usage.gpu as u8).to_be_bytes().to_vec());
    send_packet(device, gpu_usage_packet)?;

    Ok(())
}

fn send_packet(device: &HidDevice, packet: Packet) -> Result<(), HidError> {
    /*for item in &packet.raw()[..32] {
        print!("{:#02x} ", item);
    }
    println!();*/
    match device.write(&packet.to_packet_bytes()[..32]) {
        Ok(_) => Ok(()),
        Err(e) => Err(HidError::new(e.to_string()))
    }
}

fn recv_packet(device: &HidDevice) -> Result<Packet, HidError> {
    let mut buf = [0u8; 32];
    let res = device.read_timeout(&mut buf[..], 500)?;
    if res > 0 {
        for item in &buf {
            print!("{:#02x} ", item);
        }
        Packet::from(res, &mut buf[..])
    } else {
        Err(HidError::new("Packet not > 0 len!".to_string()))
    }
}