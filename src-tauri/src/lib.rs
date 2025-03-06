mod commands;
mod helpers;
mod settings;

use crate::commands::*;
use crate::helpers::*;
use crate::settings::*;
use dygma_focus::prelude::*;
use std::sync::Mutex;

struct Storage {
    focus: Mutex<Option<Focus>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    builder = observability(builder);

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .manage(Storage {
            focus: Default::default(),
        })
        .setup(|app| {
            settings(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            find_all_devices,
            connect,
            disconnect,
            version,
            settings_get,
            settings_set,
            palette_rgb_get,
            palette_rgb_set,
            palette_rgbw_get,
            palette_rgbw_set,
            color_map_get,
            color_map_set,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
