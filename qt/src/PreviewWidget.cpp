/**
 * PreviewWidget.cpp — video preview with QPainter.
 */

#include "PreviewWidget.h"
#include <QPainter>
#include <QPaintEvent>

PreviewWidget::PreviewWidget(EngineBridge *engine, QWidget *parent)
    : QWidget(parent), m_engine(engine)
{
    setMinimumSize(320, 180);
    setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);
}

void PreviewWidget::refresh() {
    // Try to decode the frame at the playhead
    double ph = m_engine->playhead();

    // Find clip at playhead
    int trackCount = m_engine->trackCount();
    for (int i = 0; i < trackCount; i++) {
        int clipCount = m_engine->clipCount(i);
        for (int j = 0; j < clipCount; j++) {
            ClipInfo clip = m_engine->clipInfo(i, j);
            if (ph >= clip.timelineStart && ph < clip.timelineStart + clip.duration) {
                // Found clip at playhead — try to decode
                double sourceTs = clip.sourceIn + (ph - clip.timelineStart);

                // Find media path for this clip
                // For now, we try to decode using the clip's media_id
                // The engine's decode_frame function takes media_id
                // But we don't have a direct way to get media_id from clip...
                // We need to add this to the FFI. For now, just update.
                break;
            }
        }
    }

    update();
}

void PreviewWidget::paintEvent(QPaintEvent *) {
    QPainter p(this);
    p.setRenderHint(QPainter::SmoothPixmapTransform, true);

    // Black background
    p.fillRect(rect(), Qt::black);

    if (!m_currentFrame.isNull()) {
        // Draw frame fit to widget
        QImage scaled = m_currentFrame.scaled(size(), Qt::KeepAspectRatio, Qt::SmoothTransformation);
        int x = (width() - scaled.width()) / 2;
        int y = (height() - scaled.height()) / 2;
        p.drawImage(x, y, scaled);
    } else {
        // Placeholder text
        p.setPen(QColor(0x6a, 0x6a, 0x6a));
        p.setFont(QFont("Arial", 12));
        p.drawText(rect(), Qt::AlignCenter, "No Preview\n\nAdd clips to timeline to see preview");

        // Timecode at bottom
        double ph = m_engine->playhead();
        double fps = m_engine->fps();
        if (fps <= 0) fps = 30.0;
        int total = (int)(ph * fps + 0.5);
        int h = total / (3600 * (int)fps);
        int m = (total / (60 * (int)fps)) % 60;
        int s = (total / (int)fps) % 60;
        int f = total % (int)fps;

        p.setPen(QColor(0x1a, 0x8c, 0xff));
        p.setFont(QFont("Consolas", 11));
        p.drawText(10, height() - 25, QString::asprintf("%02d:%02d:%02d:%02d", h, m, s, f));
    }
}
