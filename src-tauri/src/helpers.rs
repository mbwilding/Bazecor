use crate::Storage;
use anyhow::anyhow;
use dygma_focus::{errors::FocusError, Focus};
use tauri::{Builder, State, Wry};

pub(crate) fn observability(mut builder: Builder<Wry>) -> Builder<Wry> {
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
}

pub(crate) fn with_focus<T, F>(storage: State<Storage>, func: F) -> tauri::Result<T>
where
    F: FnOnce(&mut Focus) -> Result<T, FocusError>,
{
    let mut focus_guard = storage.focus.lock().unwrap();
    let focus = focus_guard
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Not connected"))?;
    match func(focus) {
        Ok(result) => Ok(result),
        Err(err) => {
            *focus_guard = None;
            Err(anyhow!(err).into())
        }
    }
}
