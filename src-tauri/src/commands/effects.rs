use serde::{Deserialize, Serialize};

/// Returns the built-in effect catalogue for the frontend.
#[tauri::command]
pub fn list_effects() -> Vec<EffectDescriptor> {
    builtin_effects()
}

/// Returns the built-in transition catalogue.
#[tauri::command]
pub fn list_transitions() -> Vec<EffectDescriptor> {
    builtin_transitions()
}

/// Attaches an effect to a clip (stored in clip.effects).
/// Real rendering happens in the export pipeline.
#[tauri::command]
pub fn apply_effect(
    clip_id: String,
    effect_id: String,
    state: tauri::State<'_, crate::models::project::ProjectState>,
) -> Result<(), String> {
    let mut p = state.inner.write();
    for t in p.tracks.iter_mut() {
        for c in t.clips.iter_mut() {
            if c.id == clip_id && !c.effects.contains(&effect_id) {
                c.effects.push(effect_id.clone());
                p.modified_at = chrono::Utc::now().to_rfc3339();
                return Ok(());
            }
        }
    }
    Err(format!("Clip {clip_id} not found"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectDescriptor {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
}

const BUILTIN_EFFECTS: &[(&str, &str, &str, &str)] = &[
    ("color-grade", "Color Grade", "color", "Adjust brightness, contrast, saturation, and temperature."),
    ("vignette", "Vignette", "image", "Darken the corners of the frame for cinematic focus."),
    ("sharpen", "Sharpen", "image", "Enhance edge detail for crisper footage."),
    ("blur", "Gaussian Blur", "image", "Apply a soft blur — useful for backgrounds or censoring."),
    ("grain", "Film Grain", "image", "Add subtle analog film grain for texture."),
    ("noise-reduce", "Noise Reduce", "audio", "Reduce background hiss from dialogue tracks."),
    ("eq", "Equalizer", "audio", "Shape frequency response for clarity and warmth."),
    ("compressor", "Compressor", "audio", "Tame dynamic range for consistent levels."),
];

const BUILTIN_TRANSITIONS: &[(&str, &str, &str, &str)] = &[
    ("fade", "Fade", "transition", "Fade to/from black."),
    ("dissolve", "Dissolve", "transition", "Cross-dissolve between two shots."),
    ("wipe", "Wipe", "transition", "Wipe from one shot to the next."),
    ("slide", "Slide", "transition", "Slide the next shot in over the previous."),
    ("zoom", "Zoom", "transition", "Punch in/out between two shots."),
];

fn builtin_effects() -> Vec<EffectDescriptor> {
    BUILTIN_EFFECTS
        .iter()
        .map(|(id, name, category, description)| EffectDescriptor {
            id: (*id).to_string(),
            name: (*name).to_string(),
            category: (*category).to_string(),
            description: (*description).to_string(),
        })
        .collect()
}

fn builtin_transitions() -> Vec<EffectDescriptor> {
    BUILTIN_TRANSITIONS
        .iter()
        .map(|(id, name, category, description)| EffectDescriptor {
            id: (*id).to_string(),
            name: (*name).to_string(),
            category: (*category).to_string(),
            description: (*description).to_string(),
        })
        .collect()
}
