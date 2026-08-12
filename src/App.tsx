import { useEffect } from "react";

import { useProjectStore } from "./stores/projectStore";
import { useUIStore } from "./stores/uiStore";

import TitleBar from "./components/TitleBar/TitleBar";
import Toolbar from "./components/Toolbar/Toolbar";
import MediaBin from "./components/MediaBin/MediaBin";
import Monitors from "./components/Monitors/Monitors";
import Timeline from "./components/Timeline/Timeline";
import Inspector from "./components/Inspector/Inspector";
import EffectsPanel from "./components/EffectsPanel/EffectsPanel";
import ExportDialog from "./components/ExportDialog/ExportDialog";

import "./App.css";

export default function App() {
  const newProject = useProjectStore((s) => s.newProject);
  const refresh = useProjectStore((s) => s.refreshFromBackend);
  const showEffects = useUIStore((s) => s.showEffects);
  const showInspector = useUIStore((s) => s.showInspector);
  const exportDialogOpen = useUIStore((s) => s.exportDialogOpen);

  useEffect(() => {
    // On first launch, create a fresh in-memory project so the UI has
    // a stable backend state to talk to.
    (async () => {
      try {
        await refresh();
        const { project } = useProjectStore.getState();
        if (!project.id) {
          await newProject();
        }
      } catch {
        await newProject();
      }
    })();
  }, [newProject, refresh]);

  return (
    <div className="app-shell" onContextMenu={(e) => e.preventDefault()}>
      <TitleBar />
      <Toolbar />
      <div className="main-area">
        <MediaBin />
        <div className="center-area">
          <Monitors />
          <Timeline />
        </div>
        <div className="right-panel">
          {showInspector && <Inspector />}
          {showEffects && <EffectsPanel />}
        </div>
      </div>
      {exportDialogOpen && <ExportDialog />}
    </div>
  );
}
