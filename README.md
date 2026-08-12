# Pro — Video Editor

A **native, GPU-accelerated** video editor built in **pure Rust**. No browser, no Electron, no WebView — just a fast, memory-safe binary that runs directly on the OS.

> **Status:** Foundation release (v0.2.0) — native rewrite. The full UI shell, timeline, media bin, dual monitors, toolbar, inspector, effects, and export dialog are all in place. FFmpeg render pipeline ships in the next iteration.

---

## Why Native (not Tauri/Electron)?

Browser-based desktop apps (Tauri, Electron, WebView2) carry a browser engine as overhead: a multi-process JS runtime, a CSS layout engine, IPC bridges between the UI and the backend, and JS garbage-collection pauses during video playback.

For a video editor that handles large files, real-time playback, and GPU effects, that overhead is the wrong architecture. **Pro** is built differently:

| | Browser-based | Pro (native) |
|---|---|---|
| Runtime | Chromium/WebKit + JS engine | Pure Rust binary |
| Memory | 100-300 MB baseline | ~20-40 MB |
| IPC latency | Cross-process JSON | Direct function calls |
| Playback pauses | JS GC jitter | None |
| Binary size | 20-100 MB | 5-10 MB |
| GPU access | Indirect (canvas/WebGL) | Direct (wgpu) |

---

## Tech Stack

- **Rust** — memory safety with zero-cost abstractions, no GC, fearless concurrency
- **egui** — pure-Rust immediate-mode GUI (used by Rerun, emulators, dev tools)
- **wgpu** — cross-platform GPU API (Vulkan/Metal/DX12)
- **rfd** — native OS file dialogs
- **ffmpeg-next** — media decode/encode (optional, feature-gated)
- **rayon** — parallel processing for render workers
- **parking_lot** — fast synchronization primitives

---

## ✨ Features (Foundation v0.2.0)

- **Media Bin** — import video/audio/image files with probing and thumbnails
- **Timeline** — 4 tracks (2 video + 2 audio), drag clips, razor split at playhead, custom painter rendering
- **Dual Monitors** — Source + Program with fit-to-frame image display and audio waveform decoration
- **Toolbar** — Select/Razor/Slip/Ripple/Hand tools, transport (play/skip), zoom slider, timecode
- **Inspector** — Source info, timing, transform (position/scale/rotation/opacity), audio volume, applied effects
- **Effects & Transitions** — 8 effects + 5 transitions (color grade, vignette, blur, EQ, fade, dissolve, wipe, …)
- **Export** — 5 presets (YouTube 1080p/4K, Web 720p, Social 1080p, ProRes), manifest writer
- **Project** — Save/Open `.prov` project files (JSON)
- **Native UX** — proper OS file dialogs, keyboard shortcuts (V/C/Y/B/H, Space, S, Delete, arrows)
- **Cross-platform** — Windows, macOS (Intel + ARM), Linux

---

## 🏗 Architecture

```
pro-video-editor/
├── Cargo.toml
├── assets/
│   └── icon.png              # App icon (embedded at compile time)
├── scripts/
│   └── generate_icon.py      # Icon generator
├── .github/workflows/
│   └── build.yml             # Multi-platform CI + release
└── src/
    ├── main.rs               # Binary entry, window setup, icon load
    ├── lib.rs                # Crate root
    ├── app.rs                # ProApp — state + edit operations + frame update
    ├── theme.rs              # Dark indigo/violet palette + egui style
    ├── state/
    │   ├── clip.rs           # Clip, ClipKind, ClipTransform
    │   ├── track.rs          # Track, TrackKind
    │   ├── project.rs        # Project, MediaAsset (document model)
    │   ├── editor.rs         # EditorState, Tool, TimelineState
    │   └── mod.rs
    ├── ui/
    │   ├── titlebar.rs       # Top menu (File / View / Export)
    │   ├── toolbar.rs        # Tools + transport + zoom
    │   ├── media_bin.rs      # Import / organize / thumbnail view
    │   ├── timeline.rs       # Custom-painted multi-track timeline
    │   ├── monitors.rs       # Source + Program preview
    │   ├── inspector.rs      # Clip property editor
    │   ├── effects.rs        # Effects & transitions catalogue
    │   ├── export_dialog.rs  # Export workflow
    │   ├── about.rs          # About dialog
    │   ├── statusbar.rs      # Bottom status bar
    │   └── mod.rs
    ├── media/
    │   ├── probe.rs          # FFmpeg probe (feature-gated) + extension fallback
    │   ├── thumbnail.rs      # Frame extraction (feature-gated) + placeholder
    │   ├── export_presets.rs # Curated export presets + effect catalogue
    │   └── mod.rs
    └── render/
        └── mod.rs            # Placeholder for future wgpu rendering
```

### State model

State is shared via `Arc<RwLock<Project>>` and `Arc<RwLock<EditorState>>`. The UI thread takes a read snapshot each frame; background workers (future render pipeline, media probing) can mutate state without blocking. No message passing, no IPC — just direct memory access guarded by a lock.

---

## 🚀 Getting started

### Prerequisites

- [Rust](https://rustup.rs/) 1.77+
- Platform-specific GUI deps:
  - **Linux:** `sudo apt install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev libgl1-mesa-dev libegl1-mesa-dev`
  - **macOS:** Xcode Command Line Tools (`xcode-select --install`)
  - **Windows:** MSVC build tools (Visual Studio Build Tools)

### Run

```bash
cargo run --release
```

### Build for production

```bash
cargo build --release
# Binary: target/release/pro-video-editor (or .exe on Windows)
```

### Enable real FFmpeg probing (optional)

```bash
# Linux
sudo apt install libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libswscale-dev libswresample-dev

# Build with the feature
cargo build --release --features ffmpeg
```

---

## ⌨️ Keyboard shortcuts

| Key | Action |
|---|---|
| `V` | Select tool |
| `C` | Razor tool |
| `Y` | Slip tool |
| `B` | Ripple tool |
| `H` | Hand tool |
| `Space` | Play / Pause |
| `←` / `→` | Skip 1 second |
| `S` | Split at playhead |
| `Delete` / `Backspace` | Remove selected clip |

---

## 🤖 CI / CD

GitHub Actions (`.github/workflows/build.yml`) builds Pro on **4 targets** in parallel:

| Runner | Target | Artifact |
|---|---|---|
| `ubuntu-22.04` | `x86_64-unknown-linux-gnu` | `.tar.gz` |
| `windows-latest` | `x86_64-pc-windows-msvc` | `.zip` |
| `macos-14` | `aarch64-apple-darwin` | `.tar.gz` (Apple Silicon) |
| `macos-13` | `x86_64-apple-darwin` | `.tar.gz` (Intel) |

- **On every push / PR:** builds in release mode, uploads artifacts (30-day retention)
- **On tag `v*`:** creates a GitHub Release with all binaries attached

### Triggering a release

```bash
git tag v0.2.0
git push origin v0.2.0
```

---

## 📋 Roadmap

### v0.2 (this release) — Native foundation
- [x] Pure-Rust native app (egui + wgpu)
- [x] All UI panels: media bin, timeline, monitors, toolbar, inspector, effects, export
- [x] Multi-platform CI + release pipeline

### v0.3 — Rendering
- [ ] FFmpeg-backed export pipeline (real video encoding)
- [ ] GPU video texture upload via wgpu
- [ ] Real-time playback in Program monitor
- [ ] Frame-accurate scrubbing

### v0.4 — Editing depth
- [ ] Ripple / slip / slide edits (full implementation)
- [ ] Keyframe animation (transform, opacity, volume)
- [ ] Snap to playhead / clip edges
- [ ] JKL shuttle control

### v0.5 — Effects rendering
- [ ] Real-time GPU effects via wgpu shaders (WGSL)
- [ ] Custom effect plugins (WASM)
- [ ] Color scopes (waveform, vectorscope, histogram)

### v0.6 — Pro features
- [ ] Multi-cam editing
- [ ] Proxy workflows for 4K/8K
- [ ] Audio mixing console
- [ ] Collaboration (CRDT-based)

---

## 📝 License

MIT — see [LICENSE](./LICENSE).

---

Built with pure Rust. No browsers were harmed in the making of this editor. © 2026 salom600.
