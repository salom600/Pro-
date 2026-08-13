/**
 * MainWindow.cpp — main application window with professional Qt layout.
 */

#include "MainWindow.h"
#include "MediaBinWidget.h"
#include "TimelineWidget.h"
#include "PreviewWidget.h"
#include "PropertiesPanel.h"

#include <QMenuBar>
#include <QToolBar>
#include <QStatusBar>
#include <QDockWidget>
#include <QAction>
#include <QIcon>
#include <QFileDialog>
#include <QMessageBox>
#include <QLabel>
#include <QTimer>
#include <QStyle>
#include <QApplication>
#include <QInputDialog>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , m_engine(new EngineBridge())
    , m_playTimer(new QTimer(this))
{
    setWindowTitle("Pro Video Editor");
    resize(1440, 900);
    setMinimumSize(1024, 640);

    applyStyleSheet();
    createMenuBar();
    createToolBar();
    createDockWidgets();

    // Status bar
    statusBar()->showMessage("Ready");

    // Playback timer — advances playhead at 60fps
    m_playTimer->setInterval(16); // ~60fps
    connect(m_playTimer, &QTimer::timeout, this, &MainWindow::onTick);

    updateTitle();
}

MainWindow::~MainWindow() {
    delete m_engine;
}

void MainWindow::createMenuBar() {
    // File menu
    QMenu *fileMenu = menuBar()->addMenu("&File");

    fileMenu->addAction("&New Project", this, &MainWindow::onNewProject, QKeySequence::New);
    fileMenu->addAction("&Open...", this, &MainWindow::onOpenProject, QKeySequence::Open);
    fileMenu->addAction("&Save", this, &MainWindow::onSaveProject, QKeySequence::Save);
    fileMenu->addSeparator();
    fileMenu->addAction("&Import Media...", this, &MainWindow::onImportMedia, QKeySequence("Ctrl+I"));
    fileMenu->addSeparator();
    fileMenu->addAction("E&xport...", this, &MainWindow::onExport, QKeySequence("Ctrl+E"));

    // Edit menu
    QMenu *editMenu = menuBar()->addMenu("&Edit");
    editMenu->addAction("&Split at Playhead", this, &MainWindow::onSplit, QKeySequence("S"));
    editMenu->addSeparator();
    editMenu->addAction("Add Video Track", this, &MainWindow::onAddVideoTrack);
    editMenu->addAction("Add Audio Track", this, &MainWindow::onAddAudioTrack);

    // View menu
    QMenu *viewMenu = menuBar()->addMenu("&View");
    viewMenu->addAction(m_mediaBin->toggleViewAction());
    viewMenu->addAction(m_properties->toggleViewAction());
}

void MainWindow::createToolBar() {
    QToolBar *toolbar = addToolBar("Main");
    toolbar->setMovable(false);
    toolbar->setIconSize(QSize(20, 20));

    // Transport controls
    toolbar->addAction(style()->standardIcon(QStyle::SP_MediaSkipBackward), "Go to Start", this, &MainWindow::onGoStart);
    toolbar->addAction(style()->standardIcon(QStyle::SP_MediaSeekBackward), "Previous Frame", this, &MainWindow::onPrevFrame);

    m_playAction = toolbar->addAction(style()->standardIcon(QStyle::SP_MediaPlay), "Play/Pause", this, &MainWindow::onPlayPause);
    m_playAction->setShortcut(QKeySequence(Qt::Key_Space));

    toolbar->addAction(style()->standardIcon(QStyle::SP_MediaSeekForward), "Next Frame", this, &MainWindow::onNextFrame);
    toolbar->addAction(style()->standardIcon(QStyle::SP_MediaSkipForward), "Go to End", this, &MainWindow::onGoEnd);

    toolbar->addSeparator();

    // Split
    toolbar->addAction(style()->standardIcon(QStyle::SP_DialogResetButton), "Split", this, &MainWindow::onSplit);

    toolbar->addSeparator();

    // Spacer
    QWidget *spacer = new QWidget();
    spacer->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
    toolbar->addWidget(spacer);

    // Timecode label (right side)
    QLabel *tcLabel = new QLabel("00:00:00:00");
    tcLabel->setStyleSheet("color: #1a8cff; font-family: monospace; font-size: 14px; padding: 0 10px;");
    tcLabel->setObjectName("timecodeLabel");
    toolbar->addWidget(tcLabel);
}

void MainWindow::createDockWidgets() {
    // Left: Media Bin
    QDockWidget *mediaDock = new QDockWidget("Media Bin", this);
    m_mediaBin = new MediaBinWidget(m_engine, mediaDock);
    mediaDock->setWidget(m_mediaBin);
    mediaDock->setMinimumWidth(250);
    mediaDock->setMaximumWidth(400);
    addDockWidget(Qt::LeftDockWidgetArea, mediaDock);

    // Right: Properties
    QDockWidget *propsDock = new QDockWidget("Properties", this);
    m_properties = new PropertiesPanel(m_engine, propsDock);
    propsDock->setWidget(m_properties);
    propsDock->setMinimumWidth(220);
    propsDock->setMaximumWidth(350);
    addDockWidget(Qt::RightDockWidgetArea, propsDock);

    // Center: Preview
    QDockWidget *previewDock = new QDockWidget("Preview", this);
    m_preview = new PreviewWidget(m_engine, previewDock);
    previewDock->setWidget(m_preview);
    previewDock->setMinimumHeight(200);
    addDockWidget(Qt::TopDockWidgetArea, previewDock);

    // Bottom: Timeline
    QDockWidget *timelineDock = new QDockWidget("Timeline", this);
    m_timeline = new TimelineWidget(m_engine, timelineDock);
    timelineDock->setWidget(m_timeline);
    timelineDock->setMinimumHeight(200);
    addDockWidget(Qt::BottomDockWidgetArea, timelineDock);
}

void MainWindow::applyStyleSheet() {
    QFile file(":/style.qss");
    if (file.open(QFile::ReadOnly)) {
        setStyleSheet(QString::fromUtf8(file.readAll()));
    } else {
        // Fallback inline stylesheet
        setStyleSheet(R"(
            QMainWindow { background: #1e1e1e; }
            QMenuBar { background: #252526; color: #e0e0e0; border-bottom: 1px solid #3c3c3c; }
            QMenuBar::item:selected { background: #37373a; }
            QMenu { background: #2d2d2d; color: #e0e0e0; border: 1px solid #3c3c3c; }
            QMenu::item:selected { background: #094d77; }
            QToolBar { background: #2d2d2d; border: none; border-bottom: 1px solid #3c3c3c; spacing: 2px; padding: 3px; }
            QStatusBar { background: #1e1e1e; color: #888; border-top: 1px solid #3c3c3c; }
            QDockWidget { titlebar-close-icon: none; titlebar-normal-icon: none; }
            QDockWidget::title { background: #2d2d2d; padding: 4px 8px; color: #969696; font-weight: bold; font-size: 10px; }
            QLabel { color: #e0e0e0; }
            QPushButton { background: #2d2d2d; color: #e0e0e0; border: 1px solid #3c3c3c; padding: 4px 12px; border-radius: 3px; }
            QPushButton:hover { background: #37373a; border-color: #007acc; }
            QPushButton:pressed { background: #094d77; }
            QTreeWidget { background: #1e1e1e; color: #e0e0e0; border: none; }
            QTreeWidget::item { padding: 4px; }
            QTreeWidget::item:selected { background: #094d77; }
            QSlider::groove:horizontal { height: 4px; background: #3c3c3c; border-radius: 2px; }
            QSlider::handle:horizontal { width: 12px; height: 12px; background: #007acc; border-radius: 6px; margin: -4px 0; }
            QScrollArea { border: none; }
        )");
    }
}

void MainWindow::updateTitle() {
    setWindowTitle(QString("%1 — Pro Video Editor").arg(m_engine->projectName()));
}

// ── Slots ──

void MainWindow::onNewProject() {
    m_engine->newProject();
    m_mediaBin->refresh();
    m_timeline->refresh();
    m_preview->refresh();
    updateTitle();
    statusBar()->showMessage("New project created");
}

void MainWindow::onOpenProject() {
    QString path = QFileDialog::getOpenFileName(this, "Open Project", "", "Pro Project (*.prov)");
    if (!path.isEmpty()) {
        if (m_engine->openProject(path)) {
            m_mediaBin->refresh();
            m_timeline->refresh();
            m_preview->refresh();
            updateTitle();
            statusBar()->showMessage("Opened: " + path);
        } else {
            QMessageBox::warning(this, "Error", "Failed to open project");
        }
    }
}

void MainWindow::onSaveProject() {
    QString path = QFileDialog::getSaveFileName(this, "Save Project", "untitled.prov", "Pro Project (*.prov)");
    if (!path.isEmpty()) {
        if (m_engine->saveProject(path)) {
            statusBar()->showMessage("Saved: " + path);
        } else {
            QMessageBox::warning(this, "Error", "Failed to save project");
        }
    }
}

void MainWindow::onImportMedia() {
    QStringList files = QFileDialog::getOpenFileNames(this, "Import Media", "",
        "Media Files (*.mp4 *.mov *.mkv *.avi *.webm *.mp3 *.wav *.aac *.flac *.ogg *.png *.jpg *.jpeg *.bmp *.webp)");
    for (const QString &file : files) {
        m_engine->importMedia(file);
    }
    if (!files.isEmpty()) {
        m_mediaBin->refresh();
        statusBar()->showMessage(QString("Imported %1 file(s)").arg(files.size()));
    }
}

void MainWindow::onExport() {
    // Simple export dialog
    QStringList presets;
    int count = EngineBridge::exportPresetCount();
    for (int i = 0; i < count; i++) {
        ExportPreset p = EngineBridge::exportPreset(i);
        presets << p.name;
    }

    bool ok;
    QString choice = QInputDialog::getItem(this, "Export", "Select preset:", presets, 0, false, &ok);
    if (!ok) return;

    QString path = QFileDialog::getSaveFileName(this, "Export Video", "output.mp4", "Video (*.mp4 *.mov)");
    if (path.isEmpty()) return;

    int idx = presets.indexOf(choice);
    if (idx >= 0) {
        ExportPreset p = EngineBridge::exportPreset(idx);
        if (m_engine->exportProject(path, p.id)) {
            statusBar()->showMessage("Exported: " + path);
        } else {
            QMessageBox::warning(this, "Error", "Export failed — timeline may be empty");
        }
    }
}

void MainWindow::onPlayPause() {
    if (m_engine->isPlaying()) {
        m_engine->pause();
        m_playTimer->stop();
        m_playAction->setIcon(style()->standardIcon(QStyle::SP_MediaPlay));
    } else {
        m_engine->play();
        m_playTimer->start();
        m_playAction->setIcon(style()->standardIcon(QStyle::SP_MediaPause));
    }
}

void MainWindow::onSplit() {
    m_engine->splitAt(m_engine->playhead());
    m_timeline->refresh();
    m_preview->refresh();
    statusBar()->showMessage("Split at playhead");
}

void MainWindow::onGoStart() {
    m_engine->setPlayhead(0.0);
    m_preview->refresh();
}

void MainWindow::onGoEnd() {
    m_engine->setPlayhead(m_engine->timelineDuration());
    m_preview->refresh();
}

void MainWindow::onPrevFrame() {
    double fps = m_engine->fps();
    m_engine->setPlayhead(m_engine->playhead() - 1.0 / fps);
    m_preview->refresh();
}

void MainWindow::onNextFrame() {
    double fps = m_engine->fps();
    m_engine->setPlayhead(m_engine->playhead() + 1.0 / fps);
    m_preview->refresh();
}

void MainWindow::onTick() {
    m_engine->tick(0.016); // 16ms = 60fps
    m_preview->refresh();

    // Update timecode label
    double ph = m_engine->playhead();
    double fps = m_engine->fps();
    if (fps <= 0) fps = 30.0;
    int total = (int)(ph * fps + 0.5);
    int h = total / (3600 * (int)fps);
    int m = (total / (60 * (int)fps)) % 60;
    int s = (total / (int)fps) % 60;
    int f = total % (int)fps;
    QLabel *tc = findChild<QLabel*>("timecodeLabel");
    if (tc) {
        tc->setText(QString::asprintf("%02d:%02d:%02d:%02d", h, m, s, f));
    }

    m_timeline->update();
}

void MainWindow::onAddVideoTrack() {
    m_engine->addVideoTrack();
    m_timeline->refresh();
}

void MainWindow::onAddAudioTrack() {
    m_engine->addAudioTrack();
    m_timeline->refresh();
}
