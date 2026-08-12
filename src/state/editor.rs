use serde::{Deserialize, Serialize};

/// Timeline-focused state — what the editor is currently doing.
/// Kept separate from the project document model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineState {
    pub playhead: f64,
    pub is_playing: bool,
    pub zoom: f64,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            playhead: 0.0,
            is_playing: false,
            zoom: 50.0, // pixels per second
        }
    }
}

/// Editor UI state — tool selection, panels, modals.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditorState {
    pub active_tool: Tool,
    pub selected_clip_id: Option<String>,
    pub source_media_id: Option<String>,
    pub timeline: TimelineState,
    pub show_media_bin: bool,
    pub show_inspector: bool,
    pub show_effects: bool,
    pub export_dialog_open: bool,
    pub about_open: bool,
}

impl EditorState {
    pub fn toggle_play(&mut self) {
        self.timeline.is_playing = !self.timeline.is_playing;
    }

    pub fn set_playhead(&mut self, t: f64) {
        self.timeline.playhead = t.max(0.0);
    }

    pub fn skip(&mut self, delta: f64) {
        self.set_playhead(self.timeline.playhead + delta);
    }

    pub fn set_zoom(&mut self, z: f64) {
        self.timeline.zoom = z.clamp(5.0, 500.0);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tool {
    #[default]
    Select,
    Razor,
    Slip,
    Ripple,
    Hand,
}

impl Tool {
    pub fn label(&self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Razor => "Razor",
            Tool::Slip => "Slip",
            Tool::Ripple => "Ripple",
            Tool::Hand => "Hand",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Tool::Select => "➤",
            Tool::Razor => "✂",
            Tool::Slip => "⇄",
            Tool::Ripple => "↔",
            Tool::Hand => "✋",
        }
    }

    pub fn shortcut(&self) -> &'static str {
        match self {
            Tool::Select => "V",
            Tool::Razor => "C",
            Tool::Slip => "Y",
            Tool::Ripple => "B",
            Tool::Hand => "H",
        }
    }
}
