/**
 * MainWindow.h — main application window.
 */

#pragma once

#include <QMainWindow>
#include <QTimer>
#include "EngineBridge.h"

class MediaBinWidget;
class TimelineWidget;
class PreviewWidget;
class PropertiesPanel;

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void onNewProject();
    void onOpenProject();
    void onSaveProject();
    void onImportMedia();
    void onExport();
    void onPlayPause();
    void onSplit();
    void onGoStart();
    void onGoEnd();
    void onPrevFrame();
    void onNextFrame();
    void onTick();
    void onAddVideoTrack();
    void onAddAudioTrack();

private:
    void createMenuBar();
    void createToolBar();
    void createDockWidgets();
    void applyStyleSheet();
    void updateTitle();

    EngineBridge *m_engine;
    MediaBinWidget *m_mediaBin;
    TimelineWidget *m_timeline;
    PreviewWidget *m_preview;
    PropertiesPanel *m_properties;
    QTimer *m_playTimer;
    QAction *m_playAction;
};
