import { useRef, useCallback } from "react";

import { useProjectStore } from "../../stores/projectStore";
import { useUIStore } from "../../stores/uiStore";
import type { Track, Clip } from "../../types";

import "./Timeline.css";

export default function Timeline() {
  const project = useProjectStore((s) => s.project);
  const zoom = useUIStore((s) => s.zoom);
  const playhead = useUIStore((s) => s.playhead);
  const setPlayhead = useUIStore((s) => s.setPlayhead);
  const activeTool = useUIStore((s) => s.activeTool);
  const selectedClipId = useProjectStore((s) => s.selectedClipId);
  const selectClip = useProjectStore((s) => s.selectClip);
  const moveClip = useProjectStore((s) => s.moveClip);
  const splitClip = useProjectStore((s) => s.splitClip);
  const removeClip = useProjectStore((s) => s.removeClip);

  const rulerRef = useRef<HTMLDivElement>(null);

  const totalDuration = Math.max(
    60,
    ...project.tracks.map((t) =>
      t.clips.reduce((m, c) => Math.max(m, c.timeline_start + c.duration), 0)
    )
  );

  const handleRulerClick = useCallback(
    (e: React.MouseEvent) => {
      if (!rulerRef.current) return;
      const rect = rulerRef.current.getBoundingClientRect();
      const x = e.clientX - rect.left + rulerRef.current.scrollLeft;
      setPlayhead(x / zoom);
    },
    [zoom, setPlayhead]
  );

  const handleClipClick = (clipId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    selectClip(clipId);
  };

  const handleClipDoubleClick = (clipId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    if (activeTool === "razor") {
      // Razor uses playhead position.
      splitClip(playhead);
    }
  };

  const handleClipDragStart = (e: React.DragEvent, clip: Clip, trackId: string) => {
    e.dataTransfer.setData("application/x-clip-id", clip.id);
    e.dataTransfer.setData("application/x-track-id", trackId);
    e.dataTransfer.effectAllowed = "move";
  };

  const handleTrackDrop = (e: React.DragEvent, trackId: string) => {
    e.preventDefault();
    const clipId = e.dataTransfer.getData("application/x-clip-id");
    if (!clipId) return;
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    const newStart = (e.clientX - rect.left) / zoom;
    moveClip(clipId, trackId, newStart);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Delete" || e.key === "Backspace") {
      if (selectedClipId) removeClip(selectedClipId);
    } else if (e.key === "s" || e.key === "S") {
      splitClip(playhead);
    }
  };

  return (
    <div className="panel timeline" tabIndex={0} onKeyDown={handleKeyDown}>
      <div className="panel-header">
        <span>Timeline</span>
        <div className="timeline-info">
          <span>{project.tracks.length} tracks</span>
          <span>·</span>
          <span>{totalDuration.toFixed(1)}s</span>
        </div>
      </div>

      <div className="timeline-body">
        <div className="track-headers">
          <div className="ruler-spacer" />
          {project.tracks.map((t) => (
            <TrackHeader key={t.id} track={t} />
          ))}
        </div>

        <div className="timeline-scroll">
          <div
            className="ruler"
            ref={rulerRef}
            onClick={handleRulerClick}
            style={{ width: `${totalDuration * zoom}px` }}
          >
            {renderRulerTicks(totalDuration, zoom)}
          </div>

          <div className="tracks-area">
            {project.tracks.map((t) => (
              <TrackLane
                key={t.id}
                track={t}
                zoom={zoom}
                totalDuration={totalDuration}
                selectedClipId={selectedClipId}
                onClipClick={handleClipClick}
                onClipDoubleClick={handleClipDoubleClick}
                onClipDragStart={handleClipDragStart}
                onTrackDrop={handleTrackDrop}
                activeTool={activeTool}
              />
            ))}
            <div
              className="playhead"
              style={{ left: `${playhead * zoom}px` }}
            >
              <div className="playhead-handle" />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function TrackHeader({ track }: { track: Track }) {
  return (
    <div className={`track-header ${track.kind}`}>
      <span className="track-name">{track.name}</span>
      <div className="track-controls">
        <button className="track-btn" title={track.muted ? "Unmute" : "Mute"} disabled>
          M
        </button>
        <button className="track-btn" title={track.hidden ? "Show" : "Hide"} disabled>
          S
        </button>
        <button className="track-btn" title={track.locked ? "Unlock" : "Lock"} disabled>
          L
        </button>
      </div>
    </div>
  );
}

interface TrackLaneProps {
  track: Track;
  zoom: number;
  totalDuration: number;
  selectedClipId: string | null;
  onClipClick: (id: string, e: React.MouseEvent) => void;
  onClipDoubleClick: (id: string, e: React.MouseEvent) => void;
  onClipDragStart: (e: React.DragEvent, clip: Clip, trackId: string) => void;
  onTrackDrop: (e: React.DragEvent, trackId: string) => void;
  activeTool: string;
}

function TrackLane({
  track,
  zoom,
  totalDuration,
  selectedClipId,
  onClipClick,
  onClipDoubleClick,
  onClipDragStart,
  onTrackDrop,
  activeTool,
}: TrackLaneProps) {
  return (
    <div
      className={`track-lane ${track.kind} ${activeTool === "razor" ? "razor-mode" : ""}`}
      style={{ width: `${totalDuration * zoom}px` }}
      onDragOver={(e) => e.preventDefault()}
      onDrop={(e) => onTrackDrop(e, track.id)}
    >
      {track.clips.map((clip) => (
        <div
          key={clip.id}
          className={`clip ${clip.kind} ${selectedClipId === clip.id ? "selected" : ""}`}
          style={{
            left: `${clip.timeline_start * zoom}px`,
            width: `${Math.max(8, clip.duration * zoom)}px`,
          }}
          onClick={(e) => onClipClick(clip.id, e)}
          onDoubleClick={(e) => onClipDoubleClick(clip.id, e)}
          draggable={activeTool === "select" || activeTool === "hand"}
          onDragStart={(e) => onClipDragStart(e, clip, track.id)}
          title={`${clip.name} — ${clip.duration.toFixed(2)}s`}
        >
          <div className="clip-label">{clip.name}</div>
          {clip.kind === "audio" && <AudioWaveform />}
          <div className="clip-effects">
            {clip.effects.slice(0, 3).map((fx) => (
              <span key={fx} className="clip-fx-dot" title={fx} />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}

function AudioWaveform() {
  // Decorative waveform rendered as CSS bars.
  return (
    <div className="clip-waveform">
      {Array.from({ length: 30 }).map((_, i) => (
        <div
          key={i}
          className="wave-bar"
          style={{ height: `${30 + Math.abs(Math.sin(i * 1.3)) * 70}%` }}
        />
      ))}
    </div>
  );
}

function renderRulerTicks(totalDuration: number, zoom: number) {
  // Adaptive tick interval based on zoom level.
  let interval = 1;
  if (zoom < 20) interval = 30;
  else if (zoom < 40) interval = 10;
  else if (zoom < 80) interval = 5;
  else if (zoom < 150) interval = 2;
  else interval = 1;

  const ticks: React.ReactNode[] = [];
  for (let s = 0; s <= totalDuration; s += interval) {
    ticks.push(
      <div key={s} className="ruler-tick" style={{ left: `${s * zoom}px` }}>
        <span className="ruler-label">{formatRuler(s)}</span>
      </div>
    );
  }
  return ticks;
}

function formatRuler(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${m}:${s.toString().padStart(2, "0")}`;
}
