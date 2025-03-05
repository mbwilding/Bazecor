use anyhow::anyhow;
use dygma_focus::prelude::*;
use log::info;
use std::sync::Mutex;
use tauri::{Result, State};
use tauri_plugin_store::StoreExt;

struct Storage {
    focus: Mutex<Option<Focus>>,
}

#[tauri::command]
fn find_all_devices() -> Result<Vec<Device>> {
    Ok(Focus::find_all_devices()?
        .into_iter()
        // For macOS as it will return 2 serial devices per dygma device
        .filter(|device| !device.serial_port.starts_with("/dev/cu."))
        .collect())
}

#[tauri::command]
fn connect(port: &str, storage: State<Storage>) -> Result<()> {
    *storage.focus.lock().unwrap() = Some(Focus::new_via_port(port)?);
    Ok(())
}

#[tauri::command]
fn disconnect(storage: State<Storage>) {
    *storage.focus.lock().unwrap() = None;
}

fn with_focus<T, F>(storage: State<Storage>, func: F) -> Result<T>
where
    F: FnOnce(&mut Focus) -> Result<T>,
{
    let mut focus_guard = storage.focus.lock().unwrap();
    let focus = focus_guard
        .as_mut()
        .ok_or_else(|| anyhow!("Not connected"))?;
    match func(focus) {
        Ok(result) => Ok(result),
        Err(err) => {
            *focus_guard = None;
            Err(err)
        }
    }
}

#[tauri::command]
fn version(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| Ok(focus.version()?))
}

#[tauri::command]
fn settings_get(storage: State<Storage>) -> Result<Settings> {
    with_focus(storage, |focus| Ok(focus.settings_get()?))
}

#[tauri::command]
fn settings_set(data: Settings, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| Ok(focus.settings_set(&data)?))
}

#[tauri::command]
fn palette_rgb_get(storage: State<Storage>) -> Result<Vec<RGB>> {
    with_focus(storage, |focus| Ok(focus.palette_rgb_get()?))
}

#[tauri::command]
fn palette_rgb_set(data: Vec<RGB>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| Ok(focus.palette_rgb_set(&data)?))
}

#[tauri::command]
fn palette_rgbw_get(storage: State<Storage>) -> Result<Vec<RGBW>> {
    with_focus(storage, |focus| Ok(focus.palette_rgbw_get()?))
}

#[tauri::command]
fn palette_rgbw_set(data: Vec<RGBW>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| Ok(focus.palette_rgbw_set(&data)?))
}

#[tauri::command]
fn color_map_get(storage: State<Storage>) -> Result<Vec<u8>> {
    with_focus(storage, |focus| Ok(focus.color_map_get()?))
}

#[tauri::command]
fn color_map_set(data: Vec<u8>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| Ok(focus.color_map_set(&data)?))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
    }

    #[cfg(not(debug_assertions))]
    {
        use tauri_plugin_log::fern::colors::ColoredLevelConfig;
        use tauri_plugin_log::{Builder, Target, TargetKind};

        let log_plugin = Builder::default()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: None }),
                Target::new(TargetKind::Webview),
            ])
            .with_colors(ColoredLevelConfig::default())
            .build();

        builder = builder.plugin(log_plugin);
    }

    builder
        .manage(Storage {
            focus: Default::default(),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        // .plugin(
        //     tauri_plugin_log::Builder::new()
        //         .format(|out, message, record| {
        //             let tech = if record.target() == "bazecor_lib" {
        //                 "B"
        //             } else {
        //                 "F"
        //             };
        //             out.finish(format_args!("[{}] {}: {}", record.level(), tech, message))
        //         })
        //         // .level(log::LevelFilter::Debug)
        //         .build(),
        // )
        .setup(|app| {
            // Create a new store or load the existing one
            // this also put the store in the app's resource table
            // so your following calls `store` calls (from both rust and js)
            // will reuse the same store
            let store = app.store("settings.json")?;

            // Note that values must be serde_json::Value instances,
            // otherwise, they will not be compatible with the JavaScript bindings.
            // store.set("settings.backupFolder", json!(""));
            // store.set("settings.backupFrequency", json!(0));
            // store.set("settings.language", json!("english"));
            // store.set("settings.darkMode", json!("system"));
            // store.set("settings.hideBluetoothExperimental", json!(false));
            // store.set("settings.showDefaults", json!(false));
            // store.set("settings.autoUpdate", json!(null));
            // store.set("settings.verbose", json!(false));
            // store.set("settings.version", json!(null));

            // Get a value from the store.
            let settings_keys = vec![
                "settings.backupFolder",
                "settings.backupFrequency",
                "settings.language",
                "settings.darkMode",
                "settings.hideBluetoothExperimental",
                "settings.showDefaults",
                "settings.autoUpdate",
                "settings.verbose",
                "settings.version",
                // "neurons",
            ];

            info!("Begin settings dump");

            for key in settings_keys {
                let value = store
                    .get(key)
                    .expect(&format!("Failed to get value for key {}", key));
                info!("{}: {}", key, value);
            }

            info!("End settings dump");

            // Remove the store from the resource table
            // store.close_resource();

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
