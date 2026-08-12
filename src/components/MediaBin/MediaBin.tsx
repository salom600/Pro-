import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import { useProjectStore } from "../../stores/projectStore";
import { useUIStore } from "../../stores/uiStore";
import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";

import "./MediaBin.css";

export default function MediaBin() {
  const project = useProjectStore((s) => s.project);
  const importMedia = useProjectStore((s) => s.importMedia);
  const removeMedia = useProjectStore((s) => s.removeMedia);
  const addClipToTimeline = useProjectStore((s) => s.addClipToTimeline);
  const setSourceMediaId = useUIStore((s) => s.setSourceMediaId);
  const sourceMediaId = useUIStore((s) => s.sourceMediaId);
  const [busy, setBusy] = useState(false);

  const handleImport = async () => {
    setBusy(true);
    try {
      const selected = await open({
        multiple: true,
        filters: [
          {
            name: "Media",
            extensions: [
              "mp4", "mov", "mkv", "avi", "webm", "m4v",
              "mp3", "wav", "aac", "flac", "ogg", "m4a",
              "png", "jpg", "jpeg", "bmp", "webp", "gif",
            ],
          },
        ],
      });
      if (!selected) return;
      const paths = Array.isArray(selected) ? selected : [selected];
      for (const p of paths) {
        await importMedia(p);
        // also try to generate thumbnail (non-fatal on failure)
        const asset = useProjectStore
          .getState()
          .project.media_assets.slice(-1)[0];
        if (asset) {
          invoke("generate_thumbnail", { mediaId: asset.id }).catch(() => {});
        }
      }
    } finally {
      setBusy(false);
    }
  };

  const handleAddToTimeline = async (mediaId: string) => {
    const asset = project.media_assets.find((a) => a.id === mediaId);
    if (!asset) return;
    const firstVideoTrack = project.tracks.find((t) => t.kind === "video" && !t.locked);
    const firstAudioTrack = project.tracks.find((t) => t.kind === "audio" && !t.locked);
    const track = asset.kind === "audio" ? firstAudioTrack : firstVideoTrack;
    if (!track) return;
    await addClipToTimeline({
      mediaId,
      trackId: track.id,
      name: asset.name,
      kind: asset.kind,
      duration: asset.duration_seconds || 5,
      timelineStart: 0,
    });
  };

  return (
    <div className="panel media-bin">
      <div className="panel-header">
        <span>Media Bin</span>
        <button className="btn btn-ghost btn-icon" onClick={handleImport} disabled={busy} title="Import media">
          +
        </button>
      </div>
      <div className="panel-body media-bin-body">
        {project.media_assets.length === 0 ? (
          <div className="empty-state">
            <div className="empty-icon">📂</div>
            <div>No media imported</div>
            <button className="btn btn-primary" onClick={handleImport} disabled={busy}>
              {busy ? "Importing..." : "Import Media"}
            </button>
          </div>
        ) : (
          <div className="media-grid">
            {project.media_assets.map((asset) => (
              <div
                key={asset.id}
                className={`media-card ${sourceMediaId === asset.id ? "selected" : ""}`}
                onClick={() => setSourceMediaId(asset.id)}
                onDoubleClick={() => handleAddToTimeline(asset.id)}
                title={`${asset.name}\nDouble-click to add to timeline`}
              >
                <div className="media-thumb">
                  {asset.thumbnail_path ? (
                    <img src={convertFileSrc(asset.thumbnail_path)} alt={asset.name} />
                  ) : (
                    <div className="media-thumb-placeholder">
                      <span>{kindIcon(asset.kind)}</span>
                    </div>
                  )}
                  <span className="media-kind-badge">{asset.kind}</span>
                  {asset.duration_seconds > 0 && (
                    <span className="media-duration">{formatDuration(asset.duration_seconds)}</span>
                  )}
                </div>
                <div className="media-info">
                  <div className="media-name" title={asset.name}>{asset.name}</div>
                  <div className="media-meta">
                    {asset.width > 0 && <span>{asset.width}×{asset.height}</span>}
                    {asset.fps > 0 && <span>{asset.fps.toFixed(1)}fps</span>}
                  </div>
                </div>
                <button
                  className="media-remove"
                  onClick={(e) => {
                    e.stopPropagation();
                    removeMedia(asset.id);
                  }}
                  title="Remove from bin"
                >
                  ×
                </button>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function kindIcon(kind: string): string {
  switch (kind) {
    case "video": return "🎬";
    case "audio": return "🎵";
    case "image": return "🖼";
    default: return "📄";
  }
}

function formatDuration(s: number): string {
  const m = Math.floor(s / 60);
  const sec = Math.floor(s % 60);
  return `${m}:${sec.toString().padStart(2, "0")}`;
}
