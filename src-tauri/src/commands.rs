use crate::Storage;
use anyhow::anyhow;
use dygma_focus::prelude::*;
use tauri::{Result, State};

#[tauri::command]
pub fn find_all_devices() -> Result<Vec<Device>> {
    Ok(Focus::find_all_devices()?
        .into_iter()
        // For macOS as it will return 2 serial devices per dygma device
        .filter(|device| !device.serial_port.starts_with("/dev/cu."))
        .collect())
}

#[tauri::command]
pub fn connect(port: &str, storage: State<Storage>) -> Result<()> {
    *storage.focus.lock().unwrap() = Some(Focus::new_via_port(port)?);
    Ok(())
}

#[tauri::command]
pub fn disconnect(storage: State<Storage>) {
    *storage.focus.lock().unwrap() = None;
}

pub fn with_focus<T, F>(storage: State<Storage>, func: F) -> Result<T>
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
pub fn version(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| Ok(focus.version()?))
}

#[tauri::command]
pub fn settings_get(storage: State<Storage>) -> Result<Settings> {
    with_focus(storage, |focus| Ok(focus.settings_get()?))
}

#[tauri::command]
pub fn settings_set(data: Settings, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| Ok(focus.settings_set(&data)?))
}

#[tauri::command]
pub fn palette_rgb_get(storage: State<Storage>) -> Result<Vec<RGB>> {
    with_focus(storage, |focus| Ok(focus.palette_rgb_get()?))
}

#[tauri::command]
pub fn palette_rgb_set(data: Vec<RGB>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| Ok(focus.palette_rgb_set(&data)?))
}

#[tauri::command]
pub fn palette_rgbw_get(storage: State<Storage>) -> Result<Vec<RGBW>> {
    with_focus(storage, |focus| Ok(focus.palette_rgbw_get()?))
}

#[tauri::command]
pub fn palette_rgbw_set(data: Vec<RGBW>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| Ok(focus.palette_rgbw_set(&data)?))
}

#[tauri::command]
pub fn color_map_get(storage: State<Storage>) -> Result<Vec<u8>> {
    with_focus(storage, |focus| Ok(focus.color_map_get()?))
}

#[tauri::command]
pub fn color_map_set(data: Vec<u8>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| Ok(focus.color_map_set(&data)?))
}
