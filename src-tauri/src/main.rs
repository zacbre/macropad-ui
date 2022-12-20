#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use sysinfo::Signal::Sys;
use sysinfo::{ProcessExt, System, SystemExt};
use crate::audio::AudioEndpoint;

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

struct State {
    pub proc_list: Arc<RwLock<HashMap<u16, String>>>,
    pub connected: Arc<RwLock<bool>>,
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
fn get_apps(state: tauri::State<State>) -> HashMap<u16, String> {
    state.proc_list.read().unwrap().clone()
}

#[tauri::command]
fn set_mapping(state: tauri::State<State>, mapping: Mapping) {
    let mut proc_list = state.proc_list.write().unwrap();
    if proc_list.contains_key(&mapping.key) {
        if let Some(x) = proc_list.get_mut(&mapping.key) {
            *x = mapping.value;
        }
    }
}

fn main() {
    let items: HashMap<u16, String> = collection! {
        0x00C0 => "spotify".to_string(),
        0x00C1 => "vivaldi".to_string(),
        0x00C2 => "cod".to_string(),
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
    let proc_list: Arc<RwLock<HashMap<u16, String>>> = Arc::new(RwLock::new(items));
    let state = State { 
        proc_list: proc_list.clone(),
        connected: Arc::new(RwLock::new(false)),
    };

    // start a separate thread to listen for HID stuff.
    let cloned_proc_list = state.proc_list.clone();
    let cloned_connected = state.connected.clone();
    std::thread::spawn(move || hid::start_hid_thread(cloned_proc_list, cloned_connected));
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_apps, set_mapping, get_connected_state, get_process_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
