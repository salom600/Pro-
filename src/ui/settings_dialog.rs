//! Settings dialog — hardware resources, playback, UI preferences.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

// Thread-local state for settings (avoids borrow checker issues).
thread_local! {
    static SETTINGS: std::cell::RefCell<Settings> = std::cell::RefCell::new(Settings::default());
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub cpu_cores: u32,
    pub ram_limit_gb: u32,
    pub gpu_acceleration: bool,
    pub playback_quality: PlaybackQuality,
    pub cache_size_gb: u32,
    pub auto_save_interval: u32,
    pub language: String,
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        let cores = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(4);
        Self {
            cpu_cores: cores,
            ram_limit_gb: 8,
            gpu_acceleration: true,
            playback_quality: PlaybackQuality::Full,
            cache_size_gb: 5,
            auto_save_interval: 5,
            language: "English".to_string(),
            theme: "Dark".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackQuality {
    Draft,
    Half,
    Full,
    Quarter,
}

impl PlaybackQuality {
    pub fn label(&self) -> &'static str {
        match self {
            PlaybackQuality::Draft => "Draft (1/4)",
            PlaybackQuality::Half => "Half (1/2)",
            PlaybackQuality::Full => "Full (1:1)",
            PlaybackQuality::Quarter => "Low (1/8)",
        }
    }
}

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    let mut open = app.editor.read().settings_open;
    if !open {
        return;
    }

    let mut should_close = false;

    egui::Window::new("Settings")
        .open(&mut open)
        .resizable(true)
        .collapsible(false)
        .default_width(480.0)
        .default_height(520.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            SETTINGS.with(|s| {
                let mut settings = s.borrow_mut();

                ui.add_space(8.0);

                // ── Hardware Resources ──
                ui.heading(
                    egui::RichText::new("Hardware Resources")
                        .color(theme::TEXT_PRIMARY)
                        .size(14.0),
                );
                ui.separator();
                ui.add_space(4.0);

                egui::Grid::new("hw_grid")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("CPU Cores:");
                        ui.add_sized(
                            egui::Vec2::new(120.0, 20.0),
                            egui::DragValue::new(&mut settings.cpu_cores)
                                .range(1..=64)
                                .suffix(" cores"),
                        );
                        ui.end_row();

                        ui.label("RAM Limit:");
                        ui.add_sized(
                            egui::Vec2::new(120.0, 20.0),
                            egui::DragValue::new(&mut settings.ram_limit_gb)
                                .range(1..=128)
                                .suffix(" GB"),
                        );
                        ui.end_row();

                        ui.label("Cache Size:");
                        ui.add_sized(
                            egui::Vec2::new(120.0, 20.0),
                            egui::DragValue::new(&mut settings.cache_size_gb)
                                .range(1..=50)
                                .suffix(" GB"),
                        );
                        ui.end_row();

                        ui.label("GPU Acceleration:");
                        ui.checkbox(&mut settings.gpu_acceleration, "Enable");
                        ui.end_row();
                    });

                ui.add_space(12.0);

                // ── Playback ──
                ui.heading(
                    egui::RichText::new("Playback")
                        .color(theme::TEXT_PRIMARY)
                        .size(14.0),
                );
                ui.separator();
                ui.add_space(4.0);

                egui::Grid::new("playback_grid")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Playback Quality:");
                        let qualities = [
                            PlaybackQuality::Draft,
                            PlaybackQuality::Half,
                            PlaybackQuality::Full,
                            PlaybackQuality::Quarter,
                        ];
                        egui::ComboBox::from_id_source("quality_combo")
                            .selected_text(settings.playback_quality.label())
                            .show_ui(ui, |ui| {
                                for q in qualities {
                                    ui.selectable_value(
                                        &mut settings.playback_quality,
                                        q,
                                        q.label(),
                                    );
                                }
                            });
                        ui.end_row();

                        ui.label("Auto-save (min):");
                        ui.add_sized(
                            egui::Vec2::new(120.0, 20.0),
                            egui::DragValue::new(&mut settings.auto_save_interval)
                                .range(0..=60),
                        );
                        ui.end_row();
                    });

                ui.add_space(12.0);

                // ── Interface ──
                ui.heading(
                    egui::RichText::new("Interface")
                        .color(theme::TEXT_PRIMARY)
                        .size(14.0),
                );
                ui.separator();
                ui.add_space(4.0);

                egui::Grid::new("ui_grid")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Language:");
                        let languages = ["English", "Arabic", "Chinese", "French", "Spanish", "German", "Japanese"];
                        egui::ComboBox::from_id_source("lang_combo")
                            .selected_text(&settings.language)
                            .show_ui(ui, |ui| {
                                for lang in languages {
                                    ui.selectable_value(
                                        &mut settings.language,
                                        lang.to_string(),
                                        lang,
                                    );
                                }
                            });
                        ui.end_row();

                        ui.label("Theme:");
                        let themes = ["Dark", "Light", "Cinema"];
                        egui::ComboBox::from_id_source("theme_combo")
                            .selected_text(&settings.theme)
                            .show_ui(ui, |ui| {
                                for t in themes {
                                    ui.selectable_value(
                                        &mut settings.theme,
                                        t.to_string(),
                                        t,
                                    );
                                }
                            });
                        ui.end_row();
                    });

                ui.add_space(16.0);

                // ── Buttons ──
                ui.horizontal(|ui| {
                    if ui.button("Reset to Defaults").clicked() {
                        *settings = Settings::default();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(egui::Button::new("Apply & Close").fill(theme::ACCENT)).clicked() {
                            should_close = true;
                        }
                        if ui.button("Cancel").clicked() {
                            should_close = true;
                        }
                    });
                });
            });
        });

    if should_close {
        open = false;
    }
    app.editor.write().settings_open = open;
}
