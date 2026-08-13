#pragma once
#include <QDockWidget>
#include "EngineBridge.h"

class PropertiesPanel : public QDockWidget {
    Q_OBJECT
public:
    explicit PropertiesPanel(EngineBridge *engine, QWidget *parent = nullptr);
    void refresh();

private:
    EngineBridge *m_engine;
    QWidget *m_content;
};
