/**
 * EngineBridge.h — C++ wrapper around the Rust FFI.
 *
 * Provides RAII and C++-friendly interfaces to the C ABI functions.
 */

#pragma once

#include <QString>
#include <QImage>
#include <vector>
#include <memory>
#include "pro_engine.h"

struct MediaInfo {
    QString id;
    QString name;
    QString path;
    QString kind;
    double duration;
    int width;
    int height;
    double fps;
    bool hasThumbnail;
};

struct TrackInfo {
    QString id;
    QString name;
    int kind;       // 0=video, 1=audio
    bool locked;
    bool muted;
    bool solo;
    bool hidden;
    int clipCount;
};

struct ClipInfo {
    QString id;
    QString name;
    int kind;       // 0=video, 1=audio, 2=image, 3=text
    double timelineStart;
    double duration;
    double sourceIn;
    double sourceOut;
};

struct ExportPreset {
    QString id;
    QString name;
    QString container;
    QString videoCodec;
    QString resolution;
    double fps;
    double bitrateMbps;
};

class EngineBridge {
public:
    EngineBridge();
    ~EngineBridge();

    // Project
    void newProject();
    bool saveProject(const QString &path);
    bool openProject(const QString &path);
    QString projectName() const;

    // Media
    QString importMedia(const QString &path);
    int mediaCount() const;
    MediaInfo mediaInfo(int index) const;
    void removeMedia(const QString &id);

    // Tracks
    int trackCount() const;
    TrackInfo trackInfo(int index) const;
    void addVideoTrack();
    void addAudioTrack();
    void removeTrack(const QString &id);
    void toggleTrackLock(const QString &id);
    void toggleTrackMute(const QString &id);
    void toggleTrackSolo(const QString &id);
    void toggleTrackVisibility(const QString &id);

    // Clips
    QString addClip(const QString &mediaId, const QString &trackId, double start);
    void removeClip(const QString &clipId);
    void splitAt(double time);
    int clipCount(int trackIndex) const;
    ClipInfo clipInfo(int trackIndex, int clipIndex) const;

    // Playback
    double playhead() const;
    void setPlayhead(double time);
    bool isPlaying() const;
    void play();
    void pause();
    double timelineDuration() const;
    double fps() const;
    void tick(double deltaSeconds);

    // Frame decoding
    QImage decodeFrame(const QString &mediaId, double timestamp);

    // Export
    static int exportPresetCount();
    static ExportPreset exportPreset(int index);
    bool exportProject(const QString &outputPath, const QString &presetId);

    // Info
    static bool hasFfmpeg();

private:
    ProEngine *m_engine;
};
