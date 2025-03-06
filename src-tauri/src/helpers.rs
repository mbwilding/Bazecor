use log::debug;
use serde_json::json;
use std::collections::HashMap;
use tauri::{App, Builder, Wry};
use tauri_plugin_store::StoreExt;

pub fn observability(mut builder: Builder<Wry>) -> Builder<Wry> {
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

pub fn settings(app: &mut App) -> anyhow::Result<()> {
    // Create a new store or load the existing one
    // this also put the store in the app's resource table
    // so your following calls `store` calls (from both rust and js)
    // will reuse the same store
    //
    // Directory:
    //   macOS: ~/Library/Application Support/com.bazecore.app/settings.json
    //   windows: ~\AppData\Roaming\com.bazecor.app\settings.json
    let store = app.store("settings.json")?;

    let settings_defaults = HashMap::from([
        ("settings.backupFolder", json!("")),
        ("settings.backupFrequency", json!(0)),
        ("settings.language", json!("english")),
        ("settings.darkMode", json!("system")),
        ("settings.hideBluetoothExperimental", json!(false)),
        ("settings.showDefaults", json!(false)),
        ("settings.autoUpdate", json!(null)),
        ("settings.verbose", json!(false)),
        ("settings.version", json!(null)),
    ]);

    for (key, default_value) in &settings_defaults {
        if !store.has(key) {
            store.set(key.to_string(), default_value.clone());
        }
    }

    let settings_keys: Vec<&str> = settings_defaults.keys().cloned().collect();

    #[cfg(debug_assertions)]
    {
        debug!("Begin settings dump");
        for key in settings_keys {
            let value = store
                .get(key)
                .expect(&format!("Failed to get value for key {}", key));
            debug!("{}: {}", key, value);
        }
        debug!("End settings dump");
    }

    // Remove the store from the resource table
    // store.close_resource();

    Ok(())
}
