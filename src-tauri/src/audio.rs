use pulse::context::{Context, FlagSet};
use sysinfo::{ProcessExt, System, SystemExt, Process, PidExt};
#[cfg(windows)]
use windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator, eRender, eMultimedia, IAudioSessionManager2, IAudioSessionControl2, ISimpleAudioVolume};
#[cfg(windows)]
use windows::Win32::System::Com::{self};
#[cfg(windows)]
use windows::core::Interface;
pub struct AudioEndpoint {
    pub name: String,
    #[cfg(windows)]
    pub volume: ISimpleAudioVolume
}

#[cfg(unix)]
pub fn enumerate_applications() -> Result<Vec<AudioEndpoint>, anyhow::Error> {
    let main_loop = pulse_glib::Mainloop::new(None);
    if let Some(main) = main_loop {
        let api = main.get_api();
        let context_new = Context::new(&main, "Macropad-UI");
        if let Some(mut context) = context_new {
            let result = context.connect(None, FlagSet::empty(), api.into());
            let closure = MultiUseCallback::
            context.set_state_callback();
        }
    }

    Ok(vec![])
}

#[cfg(windows)]
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