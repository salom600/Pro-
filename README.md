# Pro — Video Editor

A modern, cross-platform video editor built with **Rust + Tauri**. Sleek, fast, and stable on Windows, macOS, and Linux.

> **Status:** Foundation release (v0.1.0). The full UI shell, timeline, media bin, dual monitors, toolbar, inspector, effects, and export dialog are all in place. The real FFmpeg render pipeline ships in the next iteration.

---

## ✨ Features (Foundation)

| Area | Status |
|---|---|
| **Media Bin** | Import / organize video, audio, image files. Thumbnails (placeholder in CI builds). |
| **Timeline** | Multi-track (2 video + 2 audio), drag-and-drop clips, razor split, snap to playhead. |
| **Monitors** | Dual Source / Program monitors with timecode. |
| **Toolbar** | Select, Razor, Slip, Ripple, Hand tools. Play/pause, zoom, split. |
| **Inspector** | Transform (position, scale, rotation, opacity), timing, audio volume, applied effects. |
| **Effects & Transitions** | Built-in catalogue: color grade, vignette, blur, grain, EQ, compressor, fade, dissolve, wipe, slide, zoom. |
| **Export** | 5 presets (YouTube 1080p/4K, Web 720p, Social 1080p, ProRes). Manifest writer stub. |
| **Project** | Save / open `.prov` project files (JSON). |
| **Cross-platform** | Windows (MSI/NSIS), macOS (DMG, Intel + Apple Silicon), Linux (deb/AppImage). |

---

## 🏗 Architecture

```
pro-video-editor/
├── src-tauri/              # Rust backend (Tauri 2)
│   ├── src/
│   │   ├── main.rs         # Binary entry
│   │   ├── lib.rs          # App wiring, plugin + command registration
│   │   ├── commands/       # Tauri commands exposed to frontend
│   │   │   ├── system.rs   # App/platform info
│   │   │   ├── project.rs  # Create / open / save projects
│   │   │   ├── media.rs    # Import / list / probe / thumbnail
│   │   │   ├── timeline.rs # Add / remove / move / split clips
│   │   │   ├── effects.rs  # Effect catalogue + apply
│   │   │   └── export.rs   # Export presets + pipeline stub
│   │   ├── models/         # Clip / Track / Project / Timeline state
│   │   └── services/
│   │       └── ffmpeg_service.rs   # FFmpeg probe + thumbnail (feature-gated)
│   ├── capabilities/       # Tauri permissions
│   ├── icons/              # Generated app icons
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                    # React + TypeScript frontend
│   ├── components/
│   │   ├── TitleBar/       # Window chrome + menu
│   │   ├── Toolbar/        # Editing tools + transport
│   │   ├── MediaBin/       # Import & organize raw media
│   │   ├── Monitors/       # Source + Program preview
│   │   ├── Timeline/       # Multi-track timeline
│   │   ├── Inspector/      # Clip property editor
│   │   ├── EffectsPanel/   # Effects & transitions library
│   │   └── ExportDialog/   # Export workflow
│   ├── stores/             # Zustand state (project + UI)
│   ├── types/              # Shared TS types mirroring Rust models
│   └── styles/             # Theme + global CSS
├── .github/workflows/
│   └── build.yml           # Multi-platform CI + release
├── scripts/
│   └── generate_icons.py   # Icon set generator
└── package.json
```

### Tech stack

- **Backend:** Rust 1.77+ / Tauri 2 / tokio / serde / parking_lot
- **Frontend:** React 18 / TypeScript 5 / Vite 5 / Zustand
- **Media (optional):** `ffmpeg-next` (feature-gated; falls back to extension heuristics when FFmpeg dev libs are absent)
- **Packaging:** Tauri's native bundlers (MSI/NSIS, DMG, deb/AppImage)

---

## 🚀 Getting started (local dev)

### Prerequisites

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- Platform-specific Tauri dependencies:
  - **Linux:** `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libgtk-3-dev libayatana-appindicator3-dev`
  - **macOS:** Xcode Command Line Tools (`xcode-select --install`)
  - **Windows:** WebView2 runtime (preinstalled on Win 11) + MSVC build tools

### Run

```bash
npm install
npm run tauri:dev
```

### Build for production

```bash
npm run tauri:build
# Output: src-tauri/target/release/bundle/
```

### Enable real FFmpeg probing (optional)

```bash
# Linux
sudo apt install libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libswscale-dev libswresample-dev

# Then build with the feature:
npm run tauri:build -- -- --features ffmpeg
```

---

## 🤖 CI / CD

GitHub Actions (`.github/workflows/build.yml`) builds Pro on **4 targets** in parallel:

| Runner | Target | Output |
|---|---|---|
| `ubuntu-22.04` | `x86_64-unknown-linux-gnu` | `.deb` + `.AppImage` |
| `windows-latest` | `x86_64-pc-windows-msvc` | `.msi` + `.exe` (NSIS) |
| `macos-14` | `aarch64-apple-darwin` | `.dmg` (Apple Silicon) |
| `macos-13` | `x86_64-apple-darwin` | `.dmg` (Intel) |

- **On every push / PR:** builds in debug mode, uploads artifacts (30-day retention).
- **On tag `v*`:** builds in release mode, creates a GitHub Release with all installers attached.
- **On failure:** the workflow summary prints the build log location and artifact list for fast diagnosis.

### Triggering a release

```bash
git tag v0.1.0
git push origin v0.1.0
```

The Release job will collect all artifacts and publish them to the GitHub Releases page automatically.

---

## 📋 Roadmap

### v0.1 (this release) — Foundation
- [x] Project structure & build system
- [x] Rust backend command surface
- [x] React UI: media bin, timeline, monitors, toolbar, inspector, effects, export dialog
- [x] Multi-platform CI + release pipeline

### v0.2 — Rendering
- [ ] FFmpeg-backed export pipeline (real video encoding)
- [ ] Real thumbnail extraction
- [ ] Audio waveform rendering from PCM

### v0.3 — Playback
- [ ] In-app preview playback (program monitor)
- [ ] Frame-accurate scrubbing
- [ ] JKL shuttle control

### v0.4 — Editing depth
- [ ] Ripple / slip / slide edits
- [ ] Keyframe animation (transform, opacity, volume)
- [ ] Multi-cam support

### v0.5 — Effects rendering
- [ ] Real-time GPU effects via wgpu
- [ ] Custom effect plugins (WASM)
- [ ] Color scopes (waveform, vectorscope)

---

## 📝 License

MIT — see [LICENSE](./LICENSE).

---

Built with Rust, Tauri, and care. © 2026 salom600.
