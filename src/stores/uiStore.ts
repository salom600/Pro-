import { create } from "zustand";
import type { Tool } from "../types";

interface UIStore {
  activeTool: Tool;
  playhead: number;
  isPlaying: boolean;
  zoom: number;
  showEffects: boolean;
  showInspector: boolean;
  exportDialogOpen: boolean;
  sourceMediaId: string | null;

  setActiveTool: (t: Tool) => void;
  setPlayhead: (t: number) => void;
  togglePlay: () => void;
  setPlaying: (p: boolean) => void;
  setZoom: (z: number) => void;
  toggleEffects: () => void;
  toggleInspector: () => void;
  setExportDialogOpen: (v: boolean) => void;
  setSourceMediaId: (id: string | null) => void;
}

export const useUIStore = create<UIStore>((set) => ({
  activeTool: "select",
  playhead: 0,
  isPlaying: false,
  zoom: 50, // pixels per second
  showEffects: true,
  showInspector: true,
  exportDialogOpen: false,
  sourceMediaId: null,

  setActiveTool: (t) => set({ activeTool: t }),
  setPlayhead: (t) => set({ playhead: Math.max(0, t) }),
  togglePlay: () => set((s) => ({ isPlaying: !s.isPlaying })),
  setPlaying: (p) => set({ isPlaying: p }),
  setZoom: (z) => set({ zoom: Math.max(5, Math.min(500, z)) }),
  toggleEffects: () => set((s) => ({ showEffects: !s.showEffects })),
  toggleInspector: () => set((s) => ({ showInspector: !s.showInspector })),
  setExportDialogOpen: (v) => set({ exportDialogOpen: v }),
  setSourceMediaId: (id) => set({ sourceMediaId: id }),
}));
