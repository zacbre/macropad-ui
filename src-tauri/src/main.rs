#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use auto_launch::AutoLaunchBuilder;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use sysinfo::Signal::Sys;
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::{CustomMenuItem, GlobalWindowEvent, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Window, WindowEvent, WindowUrl, Wry};
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
    pub show_stats: bool,
    pub increment: i32,
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
            show_stats: true,
            increment: 5,
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
fn set_increment(state: tauri::State<State>, vol: i32) {
    {
        let mut sett = state.settings.write().unwrap();
        sett.increment = vol;
    }
    {
        let settings = state.settings.read().unwrap();
        settings.save_json();
    }
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
}

#[tauri::command]
async fn open_window(handle: tauri::AppHandle, url: String) {
    tauri::WindowBuilder::new(
        &handle,
            "via",
            WindowUrl::App(url.into())
        )
        .title("Via")
        .center()
        .inner_size(900.0, 900.0)
        .build()
        .unwrap();
}

fn main() {
    let auto = AutoLaunchBuilder::new()
        .set_app_name("Macropad-UI")
        .set_app_path(std::env::current_exe().unwrap().to_str().unwrap())
        .build()
        .unwrap();

    if let Ok(enabled) = auto.is_enabled() {
        if !enabled {
            auto.enable().unwrap();
        }
    }

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

    let tray_settings = state.settings.clone();
    let mut show_stats = CustomMenuItem::new("show_stats".to_string(), "Send Stats to Macropad");
    {
        show_stats.selected = tray_settings.write().unwrap().show_stats;
    }
    let tray_menu = SystemTrayMenu::new()
        .add_item(show_stats)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let cloned_window = window.clone();
            window.on_window_event(move |event| match event {
                WindowEvent::CloseRequested {
                    api,
                    ..
                } => {
                    cloned_window.hide();
                    api.prevent_close();
                }
                _ => {}
            });
            Ok(())
        })
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                // shown/hidden
                let window = app.get_window("main");
                if let Some(w) = window {
                    if !w.is_visible().unwrap() {
                        w.show();
                    }
                }
            },
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show_stats" => {
                        let mut settings = tray_settings.write().unwrap();
                        settings.show_stats = !settings.show_stats;
                        app.tray_handle().get_item("show_stats").set_selected(settings.show_stats);
                        settings.save_json();
                    }
                    _ => {}
                }
            }
            _ => ()
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .invoke_handler(tauri::generate_handler![get_apps, set_mapping, get_connected_state, get_process_list, open_window, set_increment])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
