use crate::helpers::with_focus;
use crate::Storage;
use anyhow::Context;
use dygma_focus::prelude::*;
use tauri::{Result, State};

#[tauri::command]
pub(crate) fn find_all_devices() -> Result<Vec<Device>> {
    Ok(Focus::find_all_devices()
        .context("Could not find any Dygma devices")?
        .into_iter()
        // For macOS as it will return 2 serial devices per dygma device
        .filter(|device| !device.serial_port.starts_with("/dev/cu."))
        .collect())
}

#[tauri::command]
pub(crate) fn connect(port: &str, storage: State<Storage>) -> Result<()> {
    *storage.focus.lock().unwrap() =
        Some(Focus::new_via_port(port).context("Failed to create Focus via port")?);
    Ok(())
}

#[tauri::command]
pub(crate) fn disconnect(storage: State<Storage>) {
    *storage.focus.lock().unwrap() = None;
}

#[tauri::command]
pub(crate) fn version(storage: State<Storage>) -> Result<String> {
    with_focus(storage, |focus| focus.version())
}

#[tauri::command]
pub(crate) fn settings_get(storage: State<Storage>) -> Result<Settings> {
    with_focus(storage, |focus| focus.settings_get())
}

#[tauri::command]
pub(crate) fn settings_set(data: Settings, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.settings_set(&data))
}

#[tauri::command]
pub(crate) fn palette_rgb_get(storage: State<Storage>) -> Result<Vec<RGB>> {
    with_focus(storage, |focus| focus.palette_rgb_get())
}

#[tauri::command]
pub(crate) fn palette_rgb_set(data: Vec<RGB>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.palette_rgb_set(&data))
}

#[tauri::command]
pub(crate) fn palette_rgbw_get(storage: State<Storage>) -> Result<Vec<RGBW>> {
    with_focus(storage, |focus| focus.palette_rgbw_get())
}

#[tauri::command]
pub(crate) fn palette_rgbw_set(data: Vec<RGBW>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.palette_rgbw_set(&data))
}

#[tauri::command]
pub(crate) fn color_map_get(storage: State<Storage>) -> Result<Vec<u8>> {
    with_focus(storage, |focus| focus.color_map_get())
}

#[tauri::command]
pub(crate) fn color_map_set(data: Vec<u8>, storage: State<Storage>) -> Result<()> {
    with_focus(storage, |focus| focus.color_map_set(&data))
}
