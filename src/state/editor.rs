use serde::{Deserialize, Serialize};

/// Timeline-focused state — what the editor is currently doing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineState {
    pub playhead: f64,
    pub is_playing: bool,
    pub zoom: f64,
    pub thumbnail_size: f32,
    pub snap_enabled: bool,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            playhead: 0.0,
            is_playing: false,
            zoom: 50.0,
            thumbnail_size: 120.0,
            snap_enabled: true,
        }
    }
}

/// Editor UI state.
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
    pub media_search_query: String,
    pub media_view_mode: MediaViewMode,
    pub active_media_tab: MediaTab,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaViewMode {
    #[default]
    Grid,
    List,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaTab {
    #[default]
    Project,
    Browser,
    Prints,
    Offices,
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

    pub fn skip_frame(&mut self, fps: f64, forward: bool) {
        let frame = 1.0 / fps.max(1.0);
        self.set_playhead(self.timeline.playhead + if forward { frame } else { -frame });
    }

    pub fn set_zoom(&mut self, z: f64) {
        self.timeline.zoom = z.clamp(5.0, 500.0);
    }

    pub fn set_thumbnail_size(&mut self, s: f32) {
        self.timeline.thumbnail_size = s.clamp(60.0, 200.0);
    }
}

/// Full professional tool set matching Premiere Pro / DaVinci Resolve.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
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

/// Track header controls state (mute/solo/lock/visibility).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrackHeaderState {
    pub locked: bool,
    pub muted: bool,
    pub solo: bool,
    pub hidden: bool,
    pub target_sync: bool,
}
