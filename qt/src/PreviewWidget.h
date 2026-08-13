#pragma once
#include <QWidget>
#include "EngineBridge.h"

class PreviewWidget : public QWidget {
    Q_OBJECT
public:
    explicit PreviewWidget(EngineBridge *engine, QWidget *parent = nullptr);
    void refresh();

protected:
    void paintEvent(QPaintEvent *event) override;

private:
    EngineBridge *m_engine;
    QImage m_currentFrame;
};
