/**
 * TimelineWidget.cpp — custom-painted timeline with tracks and clips.
 *
 * Uses QPainter for smooth, professional rendering — like DaVinci Resolve.
 */

#include "TimelineWidget.h"
#include <QPainter>
#include <QMouseEvent>
#include <QWheelEvent>
#include <QScrollBar>
#include <QStyle>

TimelineWidget::TimelineWidget(EngineBridge *engine, QWidget *parent)
    : QWidget(parent), m_engine(engine), m_zoom(50.0), m_playhead(0.0),
      m_trackHeight(50), m_headerWidth(100), m_dragging(false)
{
    setMinimumHeight(200);
    setMouseTracking(true);
    setFocusPolicy(Qt::StrongFocus);
}

void TimelineWidget::refresh() {
    m_playhead = m_engine->playhead();
    update();
}

void TimelineWidget::paintEvent(QPaintEvent *) {
    QPainter p(this);
    p.setRenderHint(QPainter::Antialiasing, true);

    // Background
    p.fillRect(rect(), QColor(0x1e, 0x1e, 0x1e));

    drawRuler(p, rect());
    drawTracks(p, rect());
    drawPlayhead(p, rect());
}

void TimelineWidget::drawRuler(QPainter &p, const QRect &r) {
    int rulerH = 22;
    QRect rulerRect(0, 0, r.width(), rulerH);

    p.fillRect(rulerRect, QColor(0x25, 0x25, 0x26));
    p.setPen(QColor(0x3c, 0x3c, 0x3c));
    p.drawLine(0, rulerH, r.width(), rulerH);

    // Time markers
    double duration = m_engine->timelineDuration();
    if (duration < 60) duration = 60;

    double interval = 30.0;
    if (m_zoom > 50) interval = 10.0;
    if (m_zoom > 100) interval = 5.0;
    if (m_zoom > 150) interval = 2.0;

    p.setPen(QColor(0x96, 0x96, 0x96));
    p.setFont(QFont("Consolas", 8));

    for (double t = 0; t <= duration; t += interval) {
        int x = m_headerWidth + (int)(t * m_zoom);
        if (x > r.width()) break;

        p.setPen(QColor(0x3c, 0x3c, 0x3c));
        p.drawLine(x, 0, x, rulerH);

        p.setPen(QColor(0x96, 0x96, 0x96));
        QString label;
        if (t >= 60) {
            label = QString::asprintf("%d:%02d", (int)(t / 60), (int)(t) % 60);
        } else {
            label = QString::asprintf("%02d", (int)t);
        }
        p.drawText(x + 3, 3, 50, rulerH - 3, Qt::AlignLeft | Qt::AlignVCenter, label);
    }
}

void TimelineWidget::drawTracks(QPainter &p, const QRect &r) {
    int rulerH = 22;
    int trackCount = m_engine->trackCount();

    for (int i = 0; i < trackCount; i++) {
        TrackInfo track = m_engine->trackInfo(i);
        int y = rulerH + i * m_trackHeight;

        // Header
        QRect headerRect(0, y, m_headerWidth, m_trackHeight);
        p.fillRect(headerRect, QColor(0x25, 0x25, 0x26));
        p.setPen(QColor(0x3c, 0x3c, 0x3c));
        p.drawLine(m_headerWidth, y, m_headerWidth, y + m_trackHeight);

        // Accent stripe
        QColor accent = (track.kind == 0) ? QColor(58, 93, 143) : QColor(45, 122, 78);
        p.fillRect(0, y, 3, m_trackHeight, accent);

        // Track name
        p.setPen(QColor(0xe0, 0xe0, 0xe0));
        p.setFont(QFont("Consolas", 9, QFont::Bold));
        p.drawText(8, y + 4, m_headerWidth - 8, 20, Qt::AlignLeft, track.name);

        // Controls
        p.setFont(QFont("Arial", 8));
        int ctrlX = 8;
        int ctrlY = y + 26;

        // Lock indicator
        p.setPen(track.locked ? QColor(0xd6, 0x4a, 0x9c) : QColor(0x6a, 0x6a, 0x6a));
        p.drawText(ctrlX, ctrlY, 20, 16, Qt::AlignCenter, track.locked ? "L" : "·");
        ctrlX += 20;

        if (track.kind == 0) {
            // Video: eye
            p.setPen(track.hidden ? QColor(0x6a, 0x6a, 0x6a) : QColor(0x96, 0x96, 0x96));
            p.drawText(ctrlX, ctrlY, 20, 16, Qt::AlignCenter, track.hidden ? "—" : "O");
        } else {
            // Audio: M, S
            p.setPen(track.muted ? QColor(0xff, 0xc1, 0x07) : QColor(0x6a, 0x6a, 0x6a));
            p.drawText(ctrlX, ctrlY, 20, 16, Qt::AlignCenter, "M");
            ctrlX += 20;
            p.setPen(track.solo ? QColor(0xff, 0xc1, 0x07) : QColor(0x6a, 0x6a, 0x6a));
            p.drawText(ctrlX, ctrlY, 20, 16, Qt::AlignCenter, "S");
        }

        // Lane background
        QRect laneRect(m_headerWidth, y, r.width() - m_headerWidth, m_trackHeight);
        p.fillRect(laneRect, (i % 2 == 0) ? QColor(0x1e, 0x1e, 0x1e) : QColor(0x25, 0x25, 0x26));
        p.setPen(QColor(0x2a, 0x2a, 0x2a));
        p.drawLine(m_headerWidth, y + m_trackHeight, r.width(), y + m_trackHeight);

        // Clips
        int clipCount = m_engine->clipCount(i);
        for (int j = 0; j < clipCount; j++) {
            ClipInfo clip = m_engine->clipInfo(i, j);
            int cx = m_headerWidth + (int)(clip.timelineStart * m_zoom);
            int cw = (int)(clip.duration * m_zoom);
            if (cw < 6) cw = 6;

            QRect clipRect(cx, y + 3, cw, m_trackHeight - 6);

            // Clip color
            QColor clipColor;
            switch (clip.kind) {
                case 0: clipColor = QColor(58, 93, 143); break;  // video
                case 1: clipColor = QColor(45, 122, 78); break;   // audio
                case 2: clipColor = QColor(138, 109, 46); break;  // image
                case 3: clipColor = QColor(138, 58, 109); break;  // text
            }

            // Clip body with gradient
            QLinearGradient grad(clipRect.topLeft(), clipRect.bottomLeft());
            grad.setColorAt(0, clipColor.lighter(120));
            grad.setColorAt(1, clipColor);
            p.fillRect(clipRect, grad);

            // Border
            p.setPen(QPen(QColor(0x3c, 0x3c, 0x3c), 1));
            p.drawRoundedRect(clipRect, 2, 2);

            // Clip name
            p.setPen(QColor(0xff, 0xff, 0xff));
            p.setFont(QFont("Arial", 8));
            p.drawText(clipRect.adjusted(4, 2, -4, -2), Qt::AlignLeft | Qt::AlignTop,
                       clip.name.left(cw / 7));

            // Audio waveform
            if (clip.kind == 1 && cw > 20) {
                int midY = clipRect.center().y();
                p.setPen(QPen(QColor(255, 255, 255, 100), 1));
                for (int bx = 2; bx < cw - 2; bx += 3) {
                    double h = (m_trackHeight * 0.25) * (0.3 + 0.7 * qAbs(qSin(bx * 0.5)));
                    p.drawLine(cx + bx, midY - h, cx + bx, midY + h);
                }
            }
        }
    }
}

void TimelineWidget::drawPlayhead(QPainter &p, const QRect &r) {
    double ph = m_engine->playhead();
    int x = m_headerWidth + (int)(ph * m_zoom);
    if (x < m_headerWidth || x > r.width()) return;

    // Vertical line
    p.setPen(QPen(QColor(0x1a, 0x8c, 0xff), 2));
    p.drawLine(x, 0, x, r.height());

    // Handle (triangle at top)
    QPolygon triangle;
    triangle << QPoint(x - 6, 0) << QPoint(x + 6, 0) << QPoint(x, 10);
    p.setBrush(QColor(0x1a, 0x8c, 0xff));
    p.setPen(Qt::NoPen);
    p.drawPolygon(triangle);
}

void TimelineWidget::mousePressEvent(QMouseEvent *event) {
    if (event->button() == Qt::LeftButton) {
        int x = event->pos().x();
        if (x > m_headerWidth) {
            double time = (double)(x - m_headerWidth) / m_zoom;
            m_engine->setPlayhead(time);
            m_dragging = true;
            update();
        }
    }
    setFocus();
}

void TimelineWidget::mouseMoveEvent(QMouseEvent *event) {
    if (m_dragging) {
        int x = event->pos().x();
        if (x > m_headerWidth) {
            double time = (double)(x - m_headerWidth) / m_zoom;
            m_engine->setPlayhead(time);
            update();
        }
    }
}

void TimelineWidget::mouseReleaseEvent(QMouseEvent *) {
    m_dragging = false;
}

void TimelineWidget::wheelEvent(QWheelEvent *event) {
    if (event->modifiers() & Qt::ControlModifier) {
        // Zoom
        m_zoom += event->angleDelta().y() / 10.0;
        m_zoom = qBound(10.0, m_zoom, 500.0);
        update();
    } else {
        QWidget::wheelEvent(event);
    }
}

int TimelineWidget::timeToX(double time, int) const {
    return m_headerWidth + (int)(time * m_zoom);
}

double TimelineWidget::xToTime(int x, int) const {
    return (double)(x - m_headerWidth) / m_zoom;
}
