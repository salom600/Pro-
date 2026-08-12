import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import { useProjectStore, selectSelectedClip } from "../../stores/projectStore";
import type { EffectDescriptor } from "../../types";

import "./EffectsPanel.css";

export default function EffectsPanel() {
  const [effects, setEffects] = useState<EffectDescriptor[]>([]);
  const [transitions, setTransitions] = useState<EffectDescriptor[]>([]);
  const [tab, setTab] = useState<"effects" | "transitions">("effects");
  const [query, setQuery] = useState("");

  const selectedClip = useProjectStore(selectSelectedClip);
  const applyEffect = useProjectStore((s) => s.applyEffect);

  useEffect(() => {
    invoke<EffectDescriptor[]>("list_effects").then(setEffects).catch(() => {});
    invoke<EffectDescriptor[]>("list_transitions").then(setTransitions).catch(() => {});
  }, []);

  const list = tab === "effects" ? effects : transitions;
  const filtered = list.filter(
    (e) =>
      e.name.toLowerCase().includes(query.toLowerCase()) ||
      e.category.toLowerCase().includes(query.toLowerCase())
  );

  const handleApply = (effectId: string) => {
    if (!selectedClip) return;
    applyEffect(selectedClip.id, effectId);
  };

  return (
    <div className="panel effects-panel">
      <div className="panel-header">
        <span>Effects & Transitions</span>
      </div>

      <div className="fx-tabs">
        <button
          className={`fx-tab ${tab === "effects" ? "active" : ""}`}
          onClick={() => setTab("effects")}
        >
          Effects
        </button>
        <button
          className={`fx-tab ${tab === "transitions" ? "active" : ""}`}
          onClick={() => setTab("transitions")}
        >
          Transitions
        </button>
      </div>

      <div className="fx-search">
        <input
          type="text"
          placeholder={`Search ${tab}…`}
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
      </div>

      <div className="panel-body fx-list">
        {!selectedClip && (
          <div className="fx-hint">
            Select a clip to apply {tab}.
          </div>
        )}
        {filtered.map((fx) => (
          <div
            key={fx.id}
            className="fx-item"
            draggable
            onDragStart={(e) => {
              e.dataTransfer.setData("application/x-effect-id", fx.id);
              e.dataTransfer.setData("application/x-effect-category", fx.category);
            }}
            onDoubleClick={() => handleApply(fx.id)}
            title={fx.description}
          >
            <div className={`fx-icon cat-${fx.category}`}>
              {categoryIcon(fx.category)}
            </div>
            <div className="fx-meta">
              <div className="fx-name">{fx.name}</div>
              <div className="fx-desc">{fx.description}</div>
            </div>
            <div className={`fx-cat-badge cat-${fx.category}`}>{fx.category}</div>
          </div>
        ))}
        {filtered.length === 0 && (
          <div className="empty-state">
            <div>No matches for "{query}"</div>
          </div>
        )}
      </div>
    </div>
  );
}

function categoryIcon(cat: string): string {
  switch (cat) {
    case "color": return "🎨";
    case "image": return "✨";
    case "audio": return "🔊";
    case "transition": return "🔄";
    default: return "⬡";
  }
}
