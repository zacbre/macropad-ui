#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use sysinfo::Signal::Sys;
use sysinfo::{ProcessExt, System, SystemExt};
use crate::audio::AudioEndpoint;
use crate::error::HidError;

mod audio;
mod error;
mod packet;
mod hid;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

pub struct State {
    pub settings: Arc<RwLock<Settings>>,
    pub connected: Arc<RwLock<bool>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub proc_list: HashMap<u16, String>,
    pub setting_item_1: bool,
}

impl Settings {
    pub fn import_json() -> Result<Settings, HidError> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "zacbre", "macropad-ui") {
            let config_dir = proj_dirs.config_dir();
            std::fs::create_dir_all(config_dir)?;
            let contents = std::fs::read_to_string(config_dir.with_file_name("config.json"))?;
            let settings = serde_json::from_str::<Settings>(contents.as_str())?;
            return Ok(settings);
        }
        Err(HidError::new("Cannot import json.".to_string()))
    }

    fn default() -> Self {
        let items: HashMap<u16, String> = collection! {
            0x00C0 => "spotify".to_string(),
            0x00C1 => String::default(),
            0x00C2 => String::default(),
            0x00C3 => String::default(),
            0x00C4 => String::default(),
            0x00C5 => String::default(),
            0x00C6 => String::default(),
            0x00C7 => String::default(),
            0x00C8 => String::default(),
            0x00C9 => String::default(),
            0x00CA => String::default(),
            0x00CB => String::default(),
            0x00CC => String::default(),
            0x00CD => String::default(),
            0x00CE => String::default(),
            0x00CF => String::default(),
            0x00D0 => String::default(),
            0x00D1 => String::default(),
            0x00D2 => String::default(),
            0x00D3 => String::default(),
            0x00D4 => String::default(),
            0x00D5 => String::default(),
            0x00D6 => String::default(),
            0x00D7 => String::default(),
            0x00D8 => String::default(),
            0x00D9 => String::default(),
            0x00DA => String::default(),
            0x00DB => String::default(),
            0x00DC => String::default(),
            0x00DD => String::default(),
            0x00DE => String::default(),
            0x00DF => String::default(),
        };

        Settings {
            proc_list: items,
            setting_item_1: true,
        }
    }

    fn save_json(&self) -> Result<(), HidError> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "zacbre", "macropad-ui") {
            let config_dir = proj_dirs.config_dir();
            std::fs::create_dir_all(config_dir)?;
            let serialized = serde_json::to_string(&self)?;
            std::fs::write(config_dir.with_file_name("config.json"), serialized.as_bytes())?;
            return Ok(());
        }
        Err(HidError::new("Cannot save json!".to_string()))
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Mapping {
    pub key: u16,
    pub value: String
}

#[tauri::command]
fn get_process_list() -> Vec<String> {
    if let Ok(audio_endpoints) = audio::enumerate_applications() {
        audio_endpoints.into_iter().map(|p| p.name).collect::<Vec<String>>()
    } else {
        vec![]
    }
}

#[tauri::command]
fn get_connected_state(state: tauri::State<State>) -> bool {
    state.connected.read().unwrap().clone()
}

#[tauri::command]
fn get_apps(state: tauri::State<State>) -> Settings {
    state.settings.read().unwrap().clone()
}

#[tauri::command]
fn set_mapping(state: tauri::State<State>, mapping: Mapping) {
    {
        let mut settings = state.settings.write().unwrap();
        if settings.proc_list.contains_key(&mapping.key) {
            if let Some(x) = settings.proc_list.get_mut(&mapping.key) {
                *x = mapping.value;
            }
        }
    }
    {
        let settings = state.settings.read().unwrap();
        settings.save_json();
    }
    // save
}

fn main() {
    // import the hashmap via json file.
    let settings = match Settings::import_json() {
        Ok(r) => r,
        Err(_) => {
            println!("Could not read json...creating new default file.");
            let default = Settings::default();
            default.save_json();
            default
        }
    };

    let settings: Arc<RwLock<Settings>> = Arc::new(RwLock::new(settings));
    let state = State { 
        settings,
        connected: Arc::new(RwLock::new(false)),
    };

    // start a separate thread to listen for HID stuff.
    let cloned_settings = state.settings.clone();
    let cloned_connected = state.connected.clone();
    std::thread::spawn(move || hid::start_hid_thread(cloned_settings, cloned_connected));
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_apps, set_mapping, get_connected_state, get_process_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
