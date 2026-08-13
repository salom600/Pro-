#pragma once
#include <QWidget>
#include "EngineBridge.h"

class TimelineWidget : public QWidget {
    Q_OBJECT
public:
    explicit TimelineWidget(EngineBridge *engine, QWidget *parent = nullptr);
    void refresh();

protected:
    void paintEvent(QPaintEvent *event) override;
    void mousePressEvent(QMouseEvent *event) override;
    void mouseMoveEvent(QMouseEvent *event) override;
    void wheelEvent(QWheelEvent *event) override;

private:
    EngineBridge *m_engine;
    double m_zoom;        // pixels per second
    double m_playhead;
    int m_trackHeight;
    int m_headerWidth;
    bool m_dragging;

    void drawRuler(QPainter &p, const QRect &rect);
    void drawTracks(QPainter &p, const QRect &rect);
    void drawPlayhead(QPainter &p, const QRect &rect);
    int timeToX(double time, int scrollX) const;
    double xToTime(int x, int scrollX) const;
};
