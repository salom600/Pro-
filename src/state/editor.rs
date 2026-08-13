use serde::{Deserialize, Serialize};

/// Editor UI state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorState {
    pub active_tool: Tool,
    pub selected_clip_id: Option<String>,
    pub source_media_id: Option<String>,
    pub playhead: f64,
    pub is_playing: bool,
    pub zoom: f64,
    pub show_media_bin: bool,
    pub show_properties: bool,
    pub export_open: bool,
    pub settings_open: bool,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            active_tool: Tool::default(),
            selected_clip_id: None,
            source_media_id: None,
            playhead: 0.0,
            is_playing: false,
            zoom: 50.0,
            show_media_bin: true,
            show_properties: true,
            export_open: false,
            settings_open: false,
        }
    }
}

/// Tools — matches standard NLE shortcuts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tool {
    #[default]
    Select,       // V
    TrackSelect,  // A
    Ripple,       // B
    Razor,        // C
    Slip,         // Y
    Slide,        // U
    Pen,          // P
    Hand,         // H
    Zoom,         // Z
    Type,         // T
}

impl Tool {
    pub fn label(&self) -> &'static str {
        match self {
            Tool::Select => "Selection",
            Tool::TrackSelect => "Track Select",
            Tool::Ripple => "Ripple Edit",
            Tool::Razor => "Razor",
            Tool::Slip => "Slip",
            Tool::Slide => "Slide",
            Tool::Pen => "Pen",
            Tool::Hand => "Hand",
            Tool::Zoom => "Zoom",
            Tool::Type => "Type",
        }
    }

    pub fn shortcut(&self) -> &'static str {
        match self {
            Tool::Select => "V",
            Tool::TrackSelect => "A",
            Tool::Ripple => "B",
            Tool::Razor => "C",
            Tool::Slip => "Y",
            Tool::Slide => "U",
            Tool::Pen => "P",
            Tool::Hand => "H",
            Tool::Zoom => "Z",
            Tool::Type => "T",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "v" | "V" => Some(Tool::Select),
            "a" | "A" => Some(Tool::TrackSelect),
            "b" | "B" => Some(Tool::Ripple),
            "c" | "C" => Some(Tool::Razor),
            "y" | "Y" => Some(Tool::Slip),
            "u" | "U" => Some(Tool::Slide),
            "p" | "P" => Some(Tool::Pen),
            "h" | "H" => Some(Tool::Hand),
            "z" | "Z" => Some(Tool::Zoom),
            "t" | "T" => Some(Tool::Type),
            _ => None,
        }
    }

    pub fn all() -> &'static [Tool] {
        &[
            Tool::Select,
            Tool::TrackSelect,
            Tool::Ripple,
            Tool::Razor,
            Tool::Slip,
            Tool::Slide,
            Tool::Pen,
            Tool::Hand,
            Tool::Zoom,
            Tool::Type,
        ]
    }
}
