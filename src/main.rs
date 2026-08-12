//! Pro Video Editor — native entry point.
//!
//! Bootstraps logging, loads the window icon, configures the native
//! viewport, and hands control to the egui app.

use std::sync::Arc;

use eframe::egui;
use parking_lot::RwLock;

use pro_video_editor::app::ProApp;
use pro_video_editor::state::editor::EditorState;
use pro_video_editor::state::project::Project;
use pro_video_editor::theme;

fn main() -> eframe::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("Pro Video Editor starting up (native, egui + wgpu)");
    log::info!("Platform: {} {}", std::env::consts::OS, std::env::consts::ARCH);

    let project = Arc::new(RwLock::new(Project::default()));
    let editor = Arc::new(RwLock::new(EditorState::default()));

    let icon = load_icon();

    let viewport = egui::ViewportBuilder::default()
        .with_title("Pro — Video Editor")
        .with_inner_size([1440.0, 900.0])
        .with_min_inner_size([1024.0, 640.0])
        .with_icon(icon)
        .with_active(true)
        .with_visible(true)
        .with_window_level(egui::WindowLevel::Normal)
        .with_fullscreen(false)
        .with_decorations(true)
        .with_transparent(false)
        .with_resizable(true);

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_simple_native("Pro Video Editor", options, move |ctx, _frame| {
        theme::apply(ctx);
        let mut app = ProApp::new(project.clone(), editor.clone());
        app.update(ctx, _frame);
    })
}

fn load_icon() -> egui::IconData {
    // Embed the icon at compile time so the binary is self-contained.
    let bytes = include_bytes!("../assets/icon.png");
    match image::load_from_memory(bytes) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            egui::IconData {
                rgba: rgba.into_raw(),
                width: w,
                height: h,
            }
        }
        Err(_) => {
            log::warn!("failed to load embedded icon; using 1x1 transparent");
            egui::IconData {
                rgba: vec![0, 0, 0, 0],
                width: 1,
                height: 1,
            }
        }
    }
}
