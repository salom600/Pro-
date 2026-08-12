use parking_lot::RwLock;
use std::sync::Arc;

use super::track::Track;

/// Timeline-focused state. Kept separate from project state so the UI can
/// read timeline-only data cheaply without serializing the whole project.
#[derive(Default)]
pub struct TimelineState {
    pub tracks: Arc<RwLock<Vec<Track>>>,
    pub playhead: Arc<RwLock<f64>>,
}
