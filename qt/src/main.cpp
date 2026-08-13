/**
 * main.cpp — Qt application entry point.
 */

#include <QApplication>
#include "MainWindow.h"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    app.setApplicationName("Pro Video Editor");
    app.setOrganizationName("salom600");

    MainWindow window;
    window.show();

    return app.exec();
}
