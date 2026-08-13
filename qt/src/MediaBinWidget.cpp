/**
 * MediaBinWidget.cpp — left panel showing imported media assets.
 */

#include "MediaBinWidget.h"
#include <QHeaderView>
#include <QMenu>
#include <QAction>
#include <QFileDialog>
#include <QMessageBox>

MediaBinWidget::MediaBinWidget(EngineBridge *engine, QWidget *parent)
    : QTreeWidget(parent), m_engine(engine)
{
    setColumnCount(3);
    setHeaderLabels({"Name", "Duration", "Type"});
    header()->setSectionResizeMode(0, QHeaderView::Stretch);
    header()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
    header()->setSectionResizeMode(2, QHeaderView::ResizeToContents);
    setRootIsDecorated(false);
    setAlternatingRowColors(true);
    setContextMenuPolicy(Qt::CustomContextMenu);

    connect(this, &QTreeWidget::itemDoubleClicked, this, &MediaBinWidget::onItemDoubleClicked);
    connect(this, &QTreeWidget::customContextMenuRequested, this, &MediaBinWidget::onContextMenu);

    refresh();
}

void MediaBinWidget::refresh() {
    clear();
    int count = m_engine->mediaCount();
    for (int i = 0; i < count; i++) {
        MediaInfo info = m_engine->mediaInfo(i);
        QTreeWidgetItem *item = new QTreeWidgetItem;

        item->setText(0, info.name);

        // Format duration
        if (info.duration > 0) {
            int min = (int)(info.duration / 60);
            int sec = (int)(info.duration) % 60;
            item->setText(1, QString::asprintf("%d:%02d", min, sec));
        } else if (info.width > 0) {
            item->setText(1, QString("%1x%2").arg(info.width).arg(info.height));
        } else {
            item->setText(1, "—");
        }

        // Type badge
        QString typeLabel;
        QColor typeColor;
        if (info.kind == "video") { typeLabel = "VID"; typeColor = QColor(58, 93, 143); }
        else if (info.kind == "audio") { typeLabel = "AUD"; typeColor = QColor(45, 122, 78); }
        else if (info.kind == "image") { typeLabel = "IMG"; typeColor = QColor(138, 109, 46); }
        else { typeLabel = "???"; typeColor = QColor(150, 150, 150); }

        item->setText(2, typeLabel);
        item->setForeground(2, typeColor);

        // Store media ID in data
        item->setData(0, Qt::UserRole, info.id);
        item->setData(0, Qt::UserRole + 1, info.kind);

        addTopLevelItem(item);
    }
}

void MediaBinWidget::onItemDoubleClicked(QTreeWidgetItem *item, int) {
    if (!item) return;

    QString mediaId = item->data(0, Qt::UserRole).toString();
    QString kind = item->data(0, Qt::UserRole + 1).toString();

    // Find appropriate track
    int trackCount = m_engine->trackCount();
    for (int i = 0; i < trackCount; i++) {
        TrackInfo track = m_engine->trackInfo(i);
        if (track.locked) continue;

        bool isVideoTrack = (track.kind == 0);
        bool isAudioMedia = (kind == "audio");

        if ((isVideoTrack && !isAudioMedia) || (!isVideoTrack && isAudioMedia)) {
            m_engine->addClip(mediaId, track.id, 0.0);
            break;
        }
    }
}

void MediaBinWidget::onImportClicked() {
    QStringList files = QFileDialog::getOpenFileNames(this, "Import Media", "",
        "Media Files (*.mp4 *.mov *.mkv *.avi *.webm *.mp3 *.wav *.aac *.flac *.ogg *.png *.jpg *.jpeg *.bmp *.webp)");
    for (const QString &file : files) {
        m_engine->importMedia(file);
    }
    refresh();
}

void MediaBinWidget::onContextMenu(const QPoint &pos) {
    QTreeWidgetItem *item = itemAt(pos);
    if (!item) return;

    QMenu menu(this);
    QAction *addAction = menu.addAction("Add to Timeline");
    QAction *removeAction = menu.addAction("Remove from Bin");

    QAction *selected = menu.exec(mapToGlobal(pos));
    if (selected == addAction) {
        onItemDoubleClicked(item, 0);
    } else if (selected == removeAction) {
        QString id = item->data(0, Qt::UserRole).toString();
        m_engine->removeMedia(id);
        refresh();
    }
}
