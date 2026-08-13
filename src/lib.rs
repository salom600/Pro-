//! Pro Video Editor — Rust engine library.
//!
//! Compiled as a cdylib (.so/.dll/.dylib) and called from the Qt/C++ frontend
//! via the C ABI defined in `ffi.rs`.

pub mod ffi;
pub mod media;
pub mod render;
pub mod state;
