import { useProjectStore, selectSelectedClip } from "../../stores/projectStore";

import "./Inspector.css";

export default function Inspector() {
  const clip = useProjectStore(selectSelectedClip);
  const project = useProjectStore((s) => s.project);

  if (!clip) {
    return (
      <div className="panel inspector">
        <div className="panel-header">
          <span>Inspector</span>
        </div>
        <div className="panel-body">
          <div className="empty-state">
            <div className="empty-icon">⚙</div>
            <div>Select a clip to edit its properties</div>
          </div>
        </div>
      </div>
    );
  }

  const asset = project.media_assets.find((a) => a.id === clip.media_id);

  return (
    <div className="panel inspector">
      <div className="panel-header">
        <span>Inspector</span>
        <span className="inspector-clip-id">#{clip.id.slice(0, 6)}</span>
      </div>
      <div className="panel-body">
        <Section title="Source">
          <Field label="Name" value={clip.name} />
          <Field label="Type" value={clip.kind.toUpperCase()} />
          {asset && <Field label="Resolution" value={`${asset.width}×${asset.height}`} />}
          {asset && asset.fps > 0 && <Field label="FPS" value={asset.fps.toFixed(2)} />}
        </Section>

        <Section title="Timing">
          <NumberField label="Timeline Start (s)" value={clip.timeline_start} step={0.1} />
          <NumberField label="Duration (s)" value={clip.duration} step={0.1} />
          <NumberField label="Source In (s)" value={clip.source_in} step={0.1} />
          <NumberField label="Source Out (s)" value={clip.source_out} step={0.1} />
        </Section>

        <Section title="Transform">
          <NumberField label="Position X" value={clip.transform.x} step={1} />
          <NumberField label="Position Y" value={clip.transform.y} step={1} />
          <SliderField label="Scale" value={clip.transform.scale} min={0.1} max={5} step={0.01} />
          <SliderField label="Rotation" value={clip.transform.rotation} min={-360} max={360} step={1} unit="°" />
          <SliderField label="Opacity" value={clip.transform.opacity} min={0} max={1} step={0.01} />
        </Section>

        {clip.kind === "audio" && (
          <Section title="Audio">
            <SliderField label="Volume" value={clip.volume} min={0} max={2} step={0.01} />
          </Section>
        )}

        <Section title="Effects Applied">
          {clip.effects.length === 0 ? (
            <div className="empty-state" style={{ padding: "12px 8px" }}>
              <div>No effects applied</div>
            </div>
          ) : (
            <ul className="effects-list">
              {clip.effects.map((fx) => (
                <li key={fx} className="effect-chip">
                  {fx}
                </li>
              ))}
            </ul>
          )}
        </Section>
      </div>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="inspector-section">
      <div className="section-title">{title}</div>
      <div className="section-body">{children}</div>
    </div>
  );
}

function Field({ label, value }: { label: string; value: string }) {
  return (
    <div className="field-row">
      <label>{label}</label>
      <span className="field-value">{value}</span>
    </div>
  );
}

function NumberField({ label, value, step }: { label: string; value: number; step: number }) {
  return (
    <div className="field-row">
      <label>{label}</label>
      <input type="number" defaultValue={value} step={step} />
    </div>
  );
}

function SliderField({
  label,
  value,
  min,
  max,
  step,
  unit,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  unit?: string;
}) {
  return (
    <div className="field-row column">
      <div className="field-row-header">
        <label>{label}</label>
        <span className="field-value">
          {value.toFixed(2)}
          {unit}
        </span>
      </div>
      <input type="range" defaultValue={value} min={min} max={max} step={step} />
    </div>
  );
}
