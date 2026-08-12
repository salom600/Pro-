import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";

import { useProjectStore } from "../../stores/projectStore";
import { useUIStore } from "../../stores/uiStore";
import type { ExportPreset, ExportResult } from "../../types";

import "./ExportDialog.css";

export default function ExportDialog() {
  const [presets, setPresets] = useState<ExportPreset[]>([]);
  const [selectedPreset, setSelectedPreset] = useState<string>("youtube-1080p");
  const [outputPath, setOutputPath] = useState<string>("");
  const [useRange, setUseRange] = useState(false);
  const [rangeStart, setRangeStart] = useState(0);
  const [rangeEnd, setRangeEnd] = useState(0);
  const [exporting, setExporting] = useState(false);
  const [result, setResult] = useState<ExportResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const project = useProjectStore((s) => s.project);
  const setExportDialogOpen = useUIStore((s) => s.setExportDialogOpen);

  useEffect(() => {
    invoke<ExportPreset[]>("get_export_presets")
      .then((p) => {
        setPresets(p);
        if (p.length > 0 && !p.find((x) => x.id === selectedPreset)) {
          setSelectedPreset(p[0].id);
        }
      })
      .catch(() => {});
    setRangeEnd(project.duration_seconds || 30);
  }, []);

  const handlePickPath = async () => {
    const preset = presets.find((p) => p.id === selectedPreset);
    const ext = preset?.container || "mp4";
    const path = await save({
      defaultPath: `${project.name || "untitled"}.${ext}`,
      filters: [{ name: ext.toUpperCase(), extensions: [ext] }],
    });
    if (typeof path === "string") setOutputPath(path);
  };

  const handleExport = async () => {
    if (!outputPath) {
      setError("Please choose an output path.");
      return;
    }
    setError(null);
    setResult(null);
    setExporting(true);
    try {
      const res = await invoke<ExportResult>("export_project", {
        request: {
          output_path: outputPath,
          preset_id: selectedPreset,
          start: useRange ? rangeStart : null,
          end: useRange ? rangeEnd : null,
        },
      });
      setResult(res);
    } catch (e) {
      setError(String(e));
    } finally {
      setExporting(false);
    }
  };

  const handleClose = () => setExportDialogOpen(false);

  const preset = presets.find((p) => p.id === selectedPreset);

  return (
    <div className="modal-overlay" onClick={handleClose}>
      <div className="modal export-dialog" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Export</h2>
          <button className="btn btn-ghost btn-icon" onClick={handleClose}>×</button>
        </div>

        <div className="modal-body">
          {result ? (
            <div className="export-success">
              <div className="success-icon">✓</div>
              <h3>Export Complete</h3>
              <p className="success-path">{result.path}</p>
              <div className="success-meta">
                <span>Duration: {result.duration_seconds.toFixed(1)}s</span>
              </div>
              <button className="btn btn-primary" onClick={handleClose}>Done</button>
            </div>
          ) : (
            <>
              <div className="form-group">
                <label>Preset</label>
                <div className="preset-grid">
                  {presets.map((p) => (
                    <button
                      key={p.id}
                      className={`preset-card ${selectedPreset === p.id ? "selected" : ""}`}
                      onClick={() => setSelectedPreset(p.id)}
                    >
                      <div className="preset-name">{p.name}</div>
                      <div className="preset-meta">
                        {p.resolution} · {p.video_codec} · {p.bitrate_mbps}Mbps
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              {preset && (
                <div className="preset-detail">
                  <div className="detail-row">
                    <span>Container</span>
                    <code>.{preset.container}</code>
                  </div>
                  <div className="detail-row">
                    <span>Video Codec</span>
                    <code>{preset.video_codec}</code>
                  </div>
                  <div className="detail-row">
                    <span>Audio Codec</span>
                    <code>{preset.audio_codec}</code>
                  </div>
                  <div className="detail-row">
                    <span>Frame Rate</span>
                    <code>{preset.fps} fps</code>
                  </div>
                </div>
              )}

              <div className="form-group">
                <label>Output Path</label>
                <div className="path-row">
                  <input
                    type="text"
                    value={outputPath}
                    onChange={(e) => setOutputPath(e.target.value)}
                    placeholder="Choose where to save…"
                  />
                  <button className="btn" onClick={handlePickPath}>Browse</button>
                </div>
              </div>

              <div className="form-group">
                <label className="checkbox-label">
                  <input
                    type="checkbox"
                    checked={useRange}
                    onChange={(e) => setUseRange(e.target.checked)}
                  />
                  Export range only
                </label>
                {useRange && (
                  <div className="range-row">
                    <input
                      type="number"
                      value={rangeStart}
                      onChange={(e) => setRangeStart(Number(e.target.value))}
                      step={0.1}
                      min={0}
                    />
                    <span>→</span>
                    <input
                      type="number"
                      value={rangeEnd}
                      onChange={(e) => setRangeEnd(Number(e.target.value))}
                      step={0.1}
                      min={rangeStart}
                    />
                    <span>seconds</span>
                  </div>
                )}
              </div>

              {error && <div className="form-error">{error}</div>}

              <div className="export-note">
                <strong>Note:</strong> This is the foundation release. The export pipeline
                currently writes a project manifest; full FFmpeg rendering ships in the next
                iteration.
              </div>
            </>
          )}
        </div>

        {!result && (
          <div className="modal-footer">
            <button className="btn" onClick={handleClose} disabled={exporting}>
              Cancel
            </button>
            <button className="btn btn-primary" onClick={handleExport} disabled={exporting}>
              {exporting ? "Exporting…" : "Start Export"}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
