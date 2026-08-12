import { useProjectStore, selectSelectedClip } from "../../stores/projectStore";
import { useUIStore } from "../../stores/uiStore";
import { convertFileSrc } from "@tauri-apps/api/core";

import "./Monitors.css";

export default function Monitors() {
  const sourceMediaId = useUIStore((s) => s.sourceMediaId);
  const project = useProjectStore((s) => s.project);
  const selectedClip = useProjectStore(selectSelectedClip);
  const playhead = useUIStore((s) => s.playhead);

  const sourceAsset = project.media_assets.find((a) => a.id === sourceMediaId);

  // Program monitor shows the clip under the playhead (best effort).
  const programClip = findClipAt(project.tracks, playhead) || selectedClip;
  const programAsset = programClip
    ? project.media_assets.find((a) => a.id === programClip.media_id)
    : null;

  return (
    <div className="monitors">
      <Monitor
        label="Source"
        asset={sourceAsset}
        clip={null}
        emptyHint="Select media from the bin"
      />
      <Monitor
        label="Program"
        asset={programAsset}
        clip={programClip}
        emptyHint="Timeline is empty"
        showTimecode={playhead}
      />
    </div>
  );
}

interface MonitorProps {
  label: string;
  asset: { path: string; name: string; kind: string } | null | undefined;
  clip: { name: string; kind: string; timeline_start: number; duration: number } | null | undefined;
  emptyHint: string;
  showTimecode?: number;
}

function Monitor({ label, asset, clip, emptyHint, showTimecode }: MonitorProps) {
  return (
    <div className="monitor panel">
      <div className="panel-header">
        <span>{label}</span>
        {clip && <span className="monitor-clip-name">{clip.name}</span>}
        {typeof showTimecode === "number" && (
          <span className="monitor-tc">{formatTc(showTimecode)}</span>
        )}
      </div>
      <div className="monitor-body">
        {asset ? (
          asset.kind === "audio" ? (
            <div className="monitor-audio">
              <div className="audio-bars">
                {Array.from({ length: 40 }).map((_, i) => (
                  <div
                    key={i}
                    className="audio-bar"
                    style={{
                      height: `${20 + Math.abs(Math.sin(i * 0.5)) * 60}%`,
                    }}
                  />
                ))}
              </div>
            </div>
          ) : asset.kind === "image" || asset.kind === "video" ? (
            <img
              src={convertFileSrc(asset.path)}
              alt={asset.name}
              className="monitor-frame"
            />
          ) : (
            <div className="monitor-empty">{emptyHint}</div>
          )
        ) : (
          <div className="monitor-empty">
            <div className="empty-icon">📺</div>
            <div>{emptyHint}</div>
          </div>
        )}
      </div>
    </div>
  );
}

function findClipAt(
  tracks: { clips: { id: string; media_id: string; timeline_start: number; duration: number; name: string; kind: string }[] }[],
  time: number
) {
  for (const t of tracks) {
    for (const c of t.clips) {
      if (time >= c.timeline_start && time < c.timeline_start + c.duration) {
        return c;
      }
    }
  }
  return null;
}

function formatTc(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  const f = Math.floor((seconds % 1) * 30);
  return `${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}:${f.toString().padStart(2, "0")}`;
}
