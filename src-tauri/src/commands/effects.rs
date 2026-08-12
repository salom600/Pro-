use serde::{Deserialize, Serialize};

/// Returns the built-in effect catalogue for the frontend.
#[tauri::command]
pub fn list_effects() -> Vec<EffectDescriptor> {
    BUILTIN_EFFECTS.to_vec()
}

/// Returns the built-in transition catalogue.
#[tauri::command]
pub fn list_transitions() -> Vec<EffectDescriptor> {
    BUILTIN_TRANSITIONS.to_vec()
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

const BUILTIN_EFFECTS: &[EffectDescriptor] = &[
    EffectDescriptor {
        id: "color-grade".into(),
        name: "Color Grade".into(),
        category: "color".into(),
        description: "Adjust brightness, contrast, saturation, and temperature.".into(),
    },
    EffectDescriptor {
        id: "vignette".into(),
        name: "Vignette".into(),
        category: "image".into(),
        description: "Darken the corners of the frame for cinematic focus.".into(),
    },
    EffectDescriptor {
        id: "sharpen".into(),
        name: "Sharpen".into(),
        category: "image".into(),
        description: "Enhance edge detail for crisper footage.".into(),
    },
    EffectDescriptor {
        id: "blur".into(),
        name: "Gaussian Blur".into(),
        category: "image".into(),
        description: "Apply a soft blur — useful for backgrounds or censoring.".into(),
    },
    EffectDescriptor {
        id: "grain".into(),
        name: "Film Grain".into(),
        category: "image".into(),
        description: "Add subtle analog film grain for texture.".into(),
    },
    EffectDescriptor {
        id: "noise-reduce".into(),
        name: "Noise Reduce".into(),
        category: "audio".into(),
        description: "Reduce background hiss from dialogue tracks.".into(),
    },
    EffectDescriptor {
        id: "eq".into(),
        name: "Equalizer".into(),
        category: "audio".into(),
        description: "Shape frequency response for clarity and warmth.".into(),
    },
    EffectDescriptor {
        id: "compressor".into(),
        name: "Compressor".into(),
        category: "audio".into(),
        description: "Tame dynamic range for consistent levels.".into(),
    },
];

const BUILTIN_TRANSITIONS: &[EffectDescriptor] = &[
    EffectDescriptor {
        id: "fade".into(),
        name: "Fade".into(),
        category: "transition".into(),
        description: "Fade to/from black.".into(),
    },
    EffectDescriptor {
        id: "dissolve".into(),
        name: "Dissolve".into(),
        category: "transition".into(),
        description: "Cross-dissolve between two shots.".into(),
    },
    EffectDescriptor {
        id: "wipe".into(),
        name: "Wipe".into(),
        category: "transition".into(),
        description: "Wipe from one shot to the next.".into(),
    },
    EffectDescriptor {
        id: "slide".into(),
        name: "Slide".into(),
        category: "transition".into(),
        description: "Slide the next shot in over the previous.".into(),
    },
    EffectDescriptor {
        id: "zoom".into(),
        name: "Zoom".into(),
        category: "transition".into(),
        description: "Punch in/out between two shots.".into(),
    },
];
