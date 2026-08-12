import { useUIStore } from "../../stores/uiStore";
import { useProjectStore } from "../../stores/projectStore";
import type { Tool } from "../../types";

import "./Toolbar.css";

const TOOLS: { id: Tool; label: string; icon: string; key: string }[] = [
  { id: "select", label: "Select", icon: "➤", key: "V" },
  { id: "razor", label: "Razor", icon: "✂", key: "C" },
  { id: "slip", label: "Slip", icon: "⇄", key: "Y" },
  { id: "ripple", label: "Ripple", icon: "↔", key: "B" },
  { id: "hand", label: "Hand", icon: "✋", key: "H" },
];

export default function Toolbar() {
  const activeTool = useUIStore((s) => s.activeTool);
  const setActiveTool = useUIStore((s) => s.setActiveTool);
  const playhead = useUIStore((s) => s.playhead);
  const setPlayhead = useUIStore((s) => s.setPlayhead);
  const isPlaying = useUIStore((s) => s.isPlaying);
  const togglePlay = useUIStore((s) => s.togglePlay);
  const zoom = useUIStore((s) => s.zoom);
  const setZoom = useUIStore((s) => s.setZoom);
  const splitClip = useProjectStore((s) => s.splitClip);

  const handleSplit = () => {
    splitClip(playhead);
  };

  const skip = (delta: number) => setPlayhead(playhead + delta);

  return (
    <div className="toolbar">
      <div className="toolbar-group">
        {TOOLS.map((t) => (
          <button
            key={t.id}
            className={`tool-btn ${activeTool === t.id ? "active" : ""}`}
            onClick={() => setActiveTool(t.id)}
            title={`${t.label} (${t.key})`}
          >
            <span className="tool-icon">{t.icon}</span>
          </button>
        ))}
      </div>

      <div className="divider-v" />

      <div className="toolbar-group">
        <button className="tool-btn" onClick={() => skip(-5)} title="Back 5s">
          <span className="tool-icon">⏮</span>
        </button>
        <button className="tool-btn play-btn" onClick={togglePlay} title="Play/Pause (Space)">
          <span className="tool-icon">{isPlaying ? "⏸" : "▶"}</span>
        </button>
        <button className="tool-btn" onClick={() => skip(5)} title="Forward 5s">
          <span className="tool-icon">⏭</span>
        </button>
        <button className="tool-btn" onClick={handleSplit} title="Split at playhead (S)">
          <span className="tool-icon">✂</span>
        </button>
      </div>

      <div className="divider-v" />

      <div className="toolbar-group timecode">
        <span className="timecode-label">TC</span>
        <span className="timecode-value">{formatTimecode(playhead)}</span>
      </div>

      <div className="toolbar-spacer" />

      <div className="toolbar-group zoom">
        <button className="tool-btn" onClick={() => setZoom(zoom - 10)} title="Zoom out">−</button>
        <input
          type="range"
          min="10"
          max="200"
          value={zoom}
          onChange={(e) => setZoom(Number(e.target.value))}
          className="zoom-slider"
        />
        <button className="tool-btn" onClick={() => setZoom(zoom + 10)} title="Zoom in">+</button>
        <span className="zoom-value">{zoom}px/s</span>
      </div>
    </div>
  );
}

function formatTimecode(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);
  const f = Math.floor((seconds % 1) * 30);
  return `${pad(h)}:${pad(m)}:${pad(s)}:${pad(f)}`;
}

function pad(n: number): string {
  return n.toString().padStart(2, "0");
}
