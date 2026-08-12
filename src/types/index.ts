// Shared types mirroring the Rust models in src-tauri/src/models/*.

export type ClipKind = "video" | "audio" | "image" | "text";
export type TrackKind = "video" | "audio";

export interface ClipTransform {
  x: number;
  y: number;
  scale: number;
  rotation: number;
  opacity: number;
  anchor_x: number;
  anchor_y: number;
}

export interface Clip {
  id: string;
  media_id: string;
  kind: ClipKind;
  name: string;
  source_in: number;
  source_out: number;
  timeline_start: number;
  duration: number;
  transform: ClipTransform;
  volume: number;
  effects: string[];
}

export interface Track {
  id: string;
  kind: TrackKind;
  name: string;
  locked: boolean;
  muted: boolean;
  hidden: boolean;
  clips: Clip[];
}

export interface MediaAsset {
  id: string;
  name: string;
  path: string;
  kind: string;
  duration_seconds: number;
  width: number;
  height: number;
  fps: number;
  thumbnail_path: string | null;
}

export interface Project {
  id: string;
  name: string;
  created_at: string;
  modified_at: string;
  fps: number;
  width: number;
  height: number;
  sample_rate: number;
  media_assets: MediaAsset[];
  tracks: Track[];
  duration_seconds: number;
}

export interface EffectDescriptor {
  id: string;
  name: string;
  category: string;
  description: string;
}

export interface ExportPreset {
  id: string;
  name: string;
  container: string;
  video_codec: string;
  audio_codec: string;
  resolution: string;
  fps: number;
  bitrate_mbps: number;
}

export interface ExportRequest {
  output_path: string;
  preset_id: string;
  start: number | null;
  end: number | null;
}

export interface ExportResult {
  path: string;
  duration_seconds: number;
  bytes: number;
}

export interface AppInfo {
  name: string;
  version: string;
  rust_version: string;
}

export interface PlatformInfo {
  os: string;
  arch: string;
  family: string;
}

export type Tool = "select" | "razor" | "slip" | "ripple" | "hand";
