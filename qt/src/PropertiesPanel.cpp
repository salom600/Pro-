/**
 * PropertiesPanel.cpp — right panel showing clip properties.
 */

#include "PropertiesPanel.h"
#include <QVBoxLayout>
#include <QLabel>
#include <QSlider>
#include <QGroupBox>
#include <QFormLayout>

PropertiesPanel::PropertiesPanel(EngineBridge *engine, QWidget *parent)
    : QDockWidget("Properties", parent), m_engine(engine)
{
    m_content = new QWidget();
    QVBoxLayout *layout = new QVBoxLayout(m_content);

    QLabel *title = new QLabel("No clip selected");
    title->setStyleSheet("color: #6a6a6a; font-size: 12px;");
    layout->addWidget(title);

    // Transform group
    QGroupBox *transformGroup = new QGroupBox("Transform");
    QFormLayout *transformLayout = new QFormLayout(transformGroup);

    QSlider *scaleSlider = new QSlider(Qt::Horizontal);
    scaleSlider->setRange(10, 500);
    scaleSlider->setValue(100);
    transformLayout->addRow("Scale:", scaleSlider);

    QSlider *opacitySlider = new QSlider(Qt::Horizontal);
    opacitySlider->setRange(0, 100);
    opacitySlider->setValue(100);
    transformLayout->addRow("Opacity:", opacitySlider);

    layout->addWidget(transformGroup);

    // Timing group
    QGroupBox *timingGroup = new QGroupBox("Timing");
    QFormLayout *timingLayout = new QFormLayout(timingGroup);
    timingLayout->addRow("Start:", new QLabel("0.00s"));
    timingLayout->addRow("Duration:", new QLabel("5.00s"));
    layout->addWidget(timingGroup);

    layout->addStretch();

    setWidget(m_content);
}

void PropertiesPanel::refresh() {
    // TODO: Update with selected clip's properties
}
