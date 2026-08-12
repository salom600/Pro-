//! Pro Video Editor — application root.
//!
//! Wires up Tauri, registers all commands, and prepares the plugin set.
//! All heavy media work lives in `services` and is exposed to the
//! frontend via `commands`.

pub mod commands;
pub mod models;
pub mod services;

use commands::{
    effects::*, export::*, media::*, project::*, timeline::*, system::*,
};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("Pro Video Editor starting up...");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(models::project::ProjectState::default())
        .manage(models::timeline::TimelineState::default())
        .invoke_handler(tauri::generate_handler![
            // system
            get_app_info,
            get_platform_info,
            // project
            create_project,
            open_project,
            save_project,
            get_recent_projects,
            // media bin
            import_media,
            list_media,
            remove_media,
            generate_thumbnail,
            probe_media,
            // timeline
            add_clip_to_timeline,
            remove_clip,
            move_clip,
            split_clip,
            get_timeline,
            // effects
            list_effects,
            list_transitions,
            apply_effect,
            // export
            export_project,
            get_export_presets,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let main_window = app.get_webview_window("main").expect("main window missing");
                main_window.open_devtools();
            }
            log::info!("Pro Video Editor ready.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Pro Video Editor");
}
