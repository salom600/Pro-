import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

import type { Project, MediaAsset, Clip, Track } from "../types";

interface ProjectStore {
  project: Project;
  selectedClipId: string | null;
  loading: boolean;
  error: string | null;

  newProject: () => Promise<void>;
  openProject: (path: string) => Promise<void>;
  saveProject: (path: string) => Promise<void>;
  refreshFromBackend: () => Promise<void>;
  importMedia: (path: string) => Promise<MediaAsset | null>;
  removeMedia: (id: string) => Promise<void>;
  addClipToTimeline: (params: {
    mediaId: string;
    trackId: string;
    name: string;
    kind: string;
    duration: number;
    timelineStart: number;
  }) => Promise<Clip | null>;
  removeClip: (clipId: string) => Promise<void>;
  moveClip: (clipId: string, newTrackId: string | null, newStart: number) => Promise<void>;
  splitClip: (atTime: number) => Promise<void>;
  selectClip: (id: string | null) => void;
  applyEffect: (clipId: string, effectId: string) => Promise<void>;
}

const defaultProject: Project = {
  id: "",
  name: "Untitled Project",
  created_at: "",
  modified_at: "",
  fps: 30,
  width: 1920,
  height: 1080,
  sample_rate: 48000,
  media_assets: [],
  tracks: [
    { id: "v1", kind: "video", name: "V1", locked: false, muted: false, hidden: false, clips: [] },
    { id: "v2", kind: "video", name: "V2", locked: false, muted: false, hidden: false, clips: [] },
    { id: "a1", kind: "audio", name: "A1", locked: false, muted: false, hidden: false, clips: [] },
    { id: "a2", kind: "audio", name: "A2", locked: false, muted: false, hidden: false, clips: [] },
  ],
  duration_seconds: 0,
};

export const useProjectStore = create<ProjectStore>((set, get) => ({
  project: defaultProject,
  selectedClipId: null,
  loading: false,
  error: null,

  newProject: async () => {
    try {
      const project = await invoke<Project>("create_project");
      set({ project, selectedClipId: null, error: null });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  openProject: async (path) => {
    set({ loading: true, error: null });
    try {
      const project = await invoke<Project>("open_project", { path });
      set({ project, selectedClipId: null, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  saveProject: async (path) => {
    try {
      await invoke("save_project", { path });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  refreshFromBackend: async () => {
    try {
      const project = await invoke<Project>("get_timeline");
      set({ project });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  importMedia: async (path) => {
    try {
      const asset = await invoke<MediaAsset>("import_media", { path });
      await get().refreshFromBackend();
      return asset;
    } catch (e) {
      set({ error: String(e) });
      return null;
    }
  },

  removeMedia: async (id) => {
    try {
      await invoke("remove_media", { id });
      await get().refreshFromBackend();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  addClipToTimeline: async ({ mediaId, trackId, name, kind, duration, timelineStart }) => {
    try {
      const clip = await invoke<Clip>("add_clip_to_timeline", {
        mediaId,
        trackId,
        name,
        kind,
        duration,
        timelineStart,
      });
      await get().refreshFromBackend();
      return clip;
    } catch (e) {
      set({ error: String(e) });
      return null;
    }
  },

  removeClip: async (clipId) => {
    try {
      await invoke("remove_clip", { clipId });
      if (get().selectedClipId === clipId) set({ selectedClipId: null });
      await get().refreshFromBackend();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  moveClip: async (clipId, newTrackId, newStart) => {
    try {
      await invoke("move_clip", { clipId, newTrackId, newStart });
      await get().refreshFromBackend();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  splitClip: async (atTime) => {
    try {
      await invoke("split_clip", { atTime });
      await get().refreshFromBackend();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  selectClip: (id) => set({ selectedClipId: id }),

  applyEffect: async (clipId, effectId) => {
    try {
      await invoke("apply_effect", { clipId, effectId });
      await get().refreshFromBackend();
    } catch (e) {
      set({ error: String(e) });
    }
  },
}));

export const selectSelectedClip = (s: ProjectStore): Clip | null => {
  const id = s.selectedClipId;
  if (!id) return null;
  for (const t of s.project.tracks) {
    const c = t.clips.find((c) => c.id === id);
    if (c) return c;
  }
  return null;
};

export const selectAllTracks = (s: ProjectStore): Track[] => s.project.tracks;

export const selectTimelineDuration = (s: ProjectStore): number => {
  let max = 0;
  for (const t of s.project.tracks) {
    for (const c of t.clips) {
      const end = c.timeline_start + c.duration;
      if (end > max) max = end;
    }
  }
  return max;
};
