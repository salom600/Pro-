//! Editor state — in-memory model shared across all UI panels.

pub mod clip;
pub mod editor;
pub mod project;
pub mod timeline;
pub mod track;

pub use clip::{Clip, ClipKind, ClipTransform};
pub use editor::{EditorState, Tool};
pub use project::{MediaAsset, Project};
pub use timeline::TimelineState;
pub use track::{Track, TrackKind};
