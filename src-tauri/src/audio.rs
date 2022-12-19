
use sysinfo::{ProcessExt, System, SystemExt, Process, PidExt};


use windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator, eRender, eMultimedia, IAudioSessionManager2, IAudioSessionControl2, ISimpleAudioVolume};
use windows::Win32::System::Com::{self};

use windows::core::Interface;
pub struct AudioEndpoint {
    pub name: String,
    pub volume: ISimpleAudioVolume
}

pub fn enumerate_applications() -> Result<Vec<AudioEndpoint>, anyhow::Error> {
    let mut applications: Vec<AudioEndpoint> = Vec::new();
    let s = System::new_all();

    unsafe {
        let enumerator = Com::CoCreateInstance::<_, IMMDeviceEnumerator>(
                &MMDeviceEnumerator,
        None,
        Com::CLSCTX_ALL,
        )
        .unwrap();

        let speakers = enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
        let manager: IAudioSessionManager2 = speakers.Activate(Com::CLSCTX_ALL, None)?;
        let sessions = manager.GetSessionEnumerator()?;

        for n in 0..sessions.GetCount()? {
            let session_control = sessions.GetSession(n)?;
            let ctl: IAudioSessionControl2 = session_control.cast()?;
            let process_id = ctl.GetProcessId()?;

            let process: Option<&Process> = s.processes().iter().filter(|(pid, _)| pid.as_u32() == process_id).map(|(_, process)| process).nth(0);

            let simple_audio_ctl: ISimpleAudioVolume = ctl.cast()?;
            let volume = simple_audio_ctl.GetMasterVolume()?;
            if let Some(proc) = process {
                println!("Display Name: {}, Volume: {}%", proc.name(), volume * 100_f32);
                applications.push(AudioEndpoint{
                    name: proc.name().to_string(),
                    volume: simple_audio_ctl,
                });
                //simple_audio_ctl.SetMasterVolume(0.75_f32, ptr::null())?;
            }
        }
    }

    Ok(applications)
}