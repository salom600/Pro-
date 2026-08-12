//! GPU rendering helpers.
//!
//! Future home of video texture upload, effect shaders (WGSL), and
//! composition pipelines. The foundation release keeps this minimal —
//! the egui painter handles all current rendering needs.

pub mod placeholder {
    /// Reserved for future wgpu integration. Keeps the module tree stable
    /// as we add GPU-accelerated rendering in later iterations.
    pub fn version() -> &'static str {
        "render-module-v0.1-placeholder"
    }
}
