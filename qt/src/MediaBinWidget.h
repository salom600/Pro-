#pragma once
#include <QTreeWidget>
#include "EngineBridge.h"

class MediaBinWidget : public QTreeWidget {
    Q_OBJECT
public:
    explicit MediaBinWidget(EngineBridge *engine, QWidget *parent = nullptr);
    void refresh();

private slots:
    void onItemDoubleClicked(QTreeWidgetItem *item, int column);
    void onImportClicked();
    void onContextMenu(const QPoint &pos);

private:
    EngineBridge *m_engine;
};
