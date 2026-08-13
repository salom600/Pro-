/**
 * EngineBridge.cpp — C++ wrapper around the Rust FFI.
 */

#include "EngineBridge.h"
#include <cstring>

EngineBridge::EngineBridge() : m_engine(pro_engine_new()) {}

EngineBridge::~EngineBridge() {
    if (m_engine) pro_engine_free(m_engine);
}

void EngineBridge::newProject() { pro_engine_new_project(m_engine); }

bool EngineBridge::saveProject(const QString &path) {
    return pro_engine_save_project(m_engine, path.toUtf8().constData());
}

bool EngineBridge::openProject(const QString &path) {
    return pro_engine_open_project(m_engine, path.toUtf8().constData());
}

QString EngineBridge::projectName() const {
    char *name = pro_engine_get_project_name(m_engine);
    QString result = QString::fromUtf8(name);
    pro_string_free(name);
    return result;
}

QString EngineBridge::importMedia(const QString &path) {
    char *id = pro_engine_import_media(m_engine, path.toUtf8().constData());
    QString result = QString::fromUtf8(id);
    pro_string_free(id);
    return result;
}

int EngineBridge::mediaCount() const {
    return pro_engine_get_media_count(m_engine);
}

MediaInfo EngineBridge::mediaInfo(int index) const {
    ProMediaInfo raw = pro_engine_get_media_info(m_engine, index);
    MediaInfo info;
    info.id = QString::fromUtf8(raw.id);
    info.name = QString::fromUtf8(raw.name);
    info.path = QString::fromUtf8(raw.path);
    info.kind = QString::fromUtf8(raw.kind);
    info.duration = raw.duration;
    info.width = raw.width;
    info.height = raw.height;
    info.fps = raw.fps;
    info.hasThumbnail = raw.has_thumbnail != 0;
    pro_media_info_free(&raw);
    return info;
}

void EngineBridge::removeMedia(const QString &id) {
    pro_engine_remove_media(m_engine, id.toUtf8().constData());
}

int EngineBridge::trackCount() const {
    return pro_engine_get_track_count(m_engine);
}

TrackInfo EngineBridge::trackInfo(int index) const {
    ProTrackInfo raw = pro_engine_get_track_info(m_engine, index);
    TrackInfo info;
    info.id = QString::fromUtf8(raw.id);
    info.name = QString::fromUtf8(raw.name);
    info.kind = raw.kind;
    info.locked = raw.locked != 0;
    info.muted = raw.muted != 0;
    info.solo = raw.solo != 0;
    info.hidden = raw.hidden != 0;
    info.clipCount = raw.clip_count;
    pro_track_info_free(&raw);
    return info;
}

void EngineBridge::addVideoTrack() { pro_engine_add_video_track(m_engine); }
void EngineBridge::addAudioTrack() { pro_engine_add_audio_track(m_engine); }

void EngineBridge::removeTrack(const QString &id) {
    pro_engine_remove_track(m_engine, id.toUtf8().constData());
}

void EngineBridge::toggleTrackLock(const QString &id) {
    pro_engine_toggle_track_lock(m_engine, id.toUtf8().constData());
}

void EngineBridge::toggleTrackMute(const QString &id) {
    pro_engine_toggle_track_mute(m_engine, id.toUtf8().constData());
}

void EngineBridge::toggleTrackSolo(const QString &id) {
    pro_engine_toggle_track_solo(m_engine, id.toUtf8().constData());
}

void EngineBridge::toggleTrackVisibility(const QString &id) {
    pro_engine_toggle_track_visibility(m_engine, id.toUtf8().constData());
}

QString EngineBridge::addClip(const QString &mediaId, const QString &trackId, double start) {
    char *id = pro_engine_add_clip(m_engine, mediaId.toUtf8().constData(), trackId.toUtf8().constData(), start);
    if (!id) return QString();
    QString result = QString::fromUtf8(id);
    pro_string_free(id);
    return result;
}

void EngineBridge::removeClip(const QString &clipId) {
    pro_engine_remove_clip(m_engine, clipId.toUtf8().constData());
}

void EngineBridge::splitAt(double time) {
    pro_engine_split_at(m_engine, time);
}

int EngineBridge::clipCount(int trackIndex) const {
    return pro_engine_get_clip_count(m_engine, trackIndex);
}

ClipInfo EngineBridge::clipInfo(int trackIndex, int clipIndex) const {
    ProClipInfo raw = pro_engine_get_clip_info(m_engine, trackIndex, clipIndex);
    ClipInfo info;
    info.id = QString::fromUtf8(raw.id);
    info.name = QString::fromUtf8(raw.name);
    info.kind = raw.kind;
    info.timelineStart = raw.timeline_start;
    info.duration = raw.duration;
    info.sourceIn = raw.source_in;
    info.sourceOut = raw.source_out;
    pro_clip_info_free(&raw);
    return info;
}

double EngineBridge::playhead() const { return pro_engine_get_playhead(m_engine); }
void EngineBridge::setPlayhead(double time) { pro_engine_set_playhead(m_engine, time); }
bool EngineBridge::isPlaying() const { return pro_engine_is_playing(m_engine) != 0; }
void EngineBridge::play() { pro_engine_play(m_engine); }
void EngineBridge::pause() { pro_engine_pause(m_engine); }
double EngineBridge::timelineDuration() const { return pro_engine_get_timeline_duration(m_engine); }
double EngineBridge::fps() const { return pro_engine_get_fps(m_engine); }
void EngineBridge::tick(double deltaSeconds) { pro_engine_tick(m_engine, deltaSeconds); }

QImage EngineBridge::decodeFrame(const QString &mediaId, double timestamp) {
    ProFrameData frame = pro_engine_decode_frame(m_engine, mediaId.toUtf8().constData(), timestamp);
    QImage image;
    if (frame.data && frame.width > 0 && frame.height > 0) {
        image = QImage(frame.data, frame.width, frame.height, frame.width * 4,
                       QImage::Format_RGBA8888).copy();
    }
    pro_frame_data_free(&frame);
    return image;
}

int EngineBridge::exportPresetCount() {
    return pro_engine_get_export_preset_count();
}

ExportPreset EngineBridge::exportPreset(int index) {
    ProExportPreset raw = pro_engine_get_export_preset(index);
    ExportPreset p;
    p.id = QString::fromUtf8(raw.id);
    p.name = QString::fromUtf8(raw.name);
    p.container = QString::fromUtf8(raw.container);
    p.videoCodec = QString::fromUtf8(raw.video_codec);
    p.resolution = QString::fromUtf8(raw.resolution);
    p.fps = raw.fps;
    p.bitrateMbps = raw.bitrate_mbps;
    pro_export_preset_free(&raw);
    return p;
}

bool EngineBridge::exportProject(const QString &outputPath, const QString &presetId) {
    return pro_engine_export(m_engine, outputPath.toUtf8().constData(), presetId.toUtf8().constData()) != 0;
}

bool EngineBridge::hasFfmpeg() {
    return pro_engine_has_ffmpeg() != 0;
}
