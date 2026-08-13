/**
 * Pro Video Editor — C ABI for the Rust engine.
 *
 * The Qt/C++ frontend links against libpro_video_editor and calls
 * these functions to interact with the Rust engine.
 *
 * Memory management:
 * - Strings returned from Rust must be freed with pro_string_free().
 * - Structs returned from Rust must be freed with their respective *_free() functions.
 * - The engine handle must be freed with pro_engine_free().
 */

#ifndef PRO_ENGINE_H
#define PRO_ENGINE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ── Opaque handle ──
typedef struct ProEngine ProEngine;

// ── Data structs ──

typedef struct {
    char *id;
    char *name;
    char *path;
    char *kind;        // "video", "audio", "image", "unknown"
    double duration;
    uint32_t width;
    uint32_t height;
    double fps;
    int has_thumbnail;
} ProMediaInfo;

typedef struct {
    char *id;
    char *name;
    int kind;          // 0=video, 1=audio
    int locked;
    int muted;
    int solo;
    int hidden;
    int clip_count;
} ProTrackInfo;

typedef struct {
    char *id;
    char *name;
    int kind;          // 0=video, 1=audio, 2=image, 3=text
    double timeline_start;
    double duration;
    double source_in;
    double source_out;
} ProClipInfo;

typedef struct {
    char *id;
    char *name;
    char *container;
    char *video_codec;
    char *resolution;
    double fps;
    double bitrate_mbps;
} ProExportPreset;

typedef struct {
    int width;
    int height;
    uint8_t *data;     // RGBA pixel data (4 * width * height bytes)
    int data_len;
} ProFrameData;

// ── Engine lifecycle ──
ProEngine* pro_engine_new(void);
void pro_engine_free(ProEngine *engine);

// ── Memory management ──
void pro_string_free(char *s);
void pro_media_info_free(ProMediaInfo *info);
void pro_track_info_free(ProTrackInfo *info);
void pro_clip_info_free(ProClipInfo *info);
void pro_frame_data_free(ProFrameData *frame);
void pro_export_preset_free(ProExportPreset *p);

// ── Project ──
void pro_engine_new_project(ProEngine *engine);
int pro_engine_save_project(ProEngine *engine, const char *path);
int pro_engine_open_project(ProEngine *engine, const char *path);
char* pro_engine_get_project_name(ProEngine *engine);

// ── Media ──
char* pro_engine_import_media(ProEngine *engine, const char *path);
int pro_engine_get_media_count(ProEngine *engine);
ProMediaInfo pro_engine_get_media_info(ProEngine *engine, int index);
void pro_engine_remove_media(ProEngine *engine, const char *id);

// ── Tracks ──
int pro_engine_get_track_count(ProEngine *engine);
ProTrackInfo pro_engine_get_track_info(ProEngine *engine, int index);
void pro_engine_add_video_track(ProEngine *engine);
void pro_engine_add_audio_track(ProEngine *engine);
void pro_engine_remove_track(ProEngine *engine, const char *id);
void pro_engine_toggle_track_lock(ProEngine *engine, const char *id);
void pro_engine_toggle_track_mute(ProEngine *engine, const char *id);
void pro_engine_toggle_track_solo(ProEngine *engine, const char *id);
void pro_engine_toggle_track_visibility(ProEngine *engine, const char *id);

// ── Clips ──
char* pro_engine_add_clip(ProEngine *engine, const char *media_id, const char *track_id, double timeline_start);
void pro_engine_remove_clip(ProEngine *engine, const char *clip_id);
void pro_engine_split_at(ProEngine *engine, double at_time);
int pro_engine_get_clip_count(ProEngine *engine, int track_index);
ProClipInfo pro_engine_get_clip_info(ProEngine *engine, int track_index, int clip_index);

// ── Playback ──
double pro_engine_get_playhead(ProEngine *engine);
void pro_engine_set_playhead(ProEngine *engine, double time);
int pro_engine_is_playing(ProEngine *engine);
void pro_engine_play(ProEngine *engine);
void pro_engine_pause(ProEngine *engine);
double pro_engine_get_timeline_duration(ProEngine *engine);
double pro_engine_get_fps(ProEngine *engine);
void pro_engine_tick(ProEngine *engine, double delta_seconds);

// ── Frame decoding ──
ProFrameData pro_engine_decode_frame(ProEngine *engine, const char *media_id, double timestamp);

// ── Export ──
int pro_engine_get_export_preset_count(void);
ProExportPreset pro_engine_get_export_preset(int index);
int pro_engine_export(ProEngine *engine, const char *output_path, const char *preset_id);

// ── Info ──
int pro_engine_has_ffmpeg(void);

#ifdef __cplusplus
}
#endif

#endif // PRO_ENGINE_H
