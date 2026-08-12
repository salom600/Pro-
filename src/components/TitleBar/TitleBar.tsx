import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

import { useProjectStore } from "../../stores/projectStore";
import { useUIStore } from "../../stores/uiStore";
import type { AppInfo, PlatformInfo } from "../../types";

import "./TitleBar.css";

export default function TitleBar() {
  const project = useProjectStore((s) => s.project);
  const newProject = useProjectStore((s) => s.newProject);
  const openProject = useProjectStore((s) => s.openProject);
  const saveProject = useProjectStore((s) => s.saveProject);
  const setExportDialogOpen = useUIStore((s) => s.setExportDialogOpen);
  const toggleEffects = useUIStore((s) => s.toggleEffects);
  const toggleInspector = useUIStore((s) => s.toggleInspector);

  const [appInfo, setAppInfo] = useState<AppInfo | null>(null);
  const [platform, setPlatform] = useState<PlatformInfo | null>(null);

  useEffect(() => {
    invoke<AppInfo>("get_app_info").then(setAppInfo).catch(() => {});
    invoke<PlatformInfo>("get_platform_info").then(setPlatform).catch(() => {});
  }, []);

  const handleNew = () => newProject();
  const handleOpen = async () => {
    const path = await open({
      filters: [{ name: "Pro Project", extensions: ["prov", "json"] }],
    });
    if (typeof path === "string") await openProject(path);
  };
  const handleSave = async () => {
    const path = await save({
      defaultPath: `${project.name || "untitled"}.prov`,
      filters: [{ name: "Pro Project", extensions: ["prov"] }],
    });
    if (typeof path === "string") await saveProject(path);
  };

  return (
    <div className="titlebar" data-tauri-drag-region>
      <div className="titlebar-left" data-tauri-drag-region>
        <div className="app-logo">
          <svg width="20" height="20" viewBox="0 0 64 64" fill="none">
            <defs>
              <linearGradient id="tb-logo" x1="0" y1="0" x2="1" y2="1">
                <stop offset="0%" stopColor="#6366f1" />
                <stop offset="100%" stopColor="#8b5cf6" />
              </linearGradient>
            </defs>
            <rect width="64" height="64" rx="14" fill="url(#tb-logo)" />
            <path d="M24 20 L24 44 L44 32 Z" fill="white" />
          </svg>
        </div>
        <span className="app-name">Pro</span>
        <span className="title-sep">/</span>
        <span className="project-name">{project.name}</span>
        {project.modified_at && (
          <span className="title-modified">— edited {new Date(project.modified_at).toLocaleTimeString()}</span>
        )}
      </div>

      <div className="titlebar-center">
        <button className="tb-menu-btn" onClick={handleNew}>New</button>
        <button className="tb-menu-btn" onClick={handleOpen}>Open</button>
        <button className="tb-menu-btn" onClick={handleSave}>Save</button>
        <div className="divider-v" />
        <button className="tb-menu-btn primary" onClick={() => setExportDialogOpen(true)}>
          Export
        </button>
      </div>

      <div className="titlebar-right">
        <button className="tb-menu-btn" onClick={toggleInspector} title="Toggle Inspector">Inspect</button>
        <button className="tb-menu-btn" onClick={toggleEffects} title="Toggle Effects">Effects</button>
        <div className="divider-v" />
        {platform && (
          <span className="platform-info" title={`${platform.os} ${platform.arch}`}>
            {platform.os === "macos" ? "" : platform.os === "windows" ? "" : ""}
            <span className="status-dot" />
            {platform.os}
          </span>
        )}
        {appInfo && <span className="version-info">v{appInfo.version}</span>}
      </div>
    </div>
  );
}
