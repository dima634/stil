#pragma once

#include "window.h"
#include <ipc/ipc.h>
#include <QtCore/QObject>
#include <QtQmlIntegration/QtQmlIntegration>
#include <QtQml/QQmlListProperty>

class Desktop: public QObject
{
    Q_OBJECT
    Q_PROPERTY(QQmlListProperty<Window> windows READ getWindows NOTIFY windowsChanged)

    QML_ELEMENT
    QML_SINGLETON

    public:
    Desktop();
    QQmlListProperty<Window> getWindows() { return QQmlListProperty<Window>(this, &m_windows); }
    void addWindow(Window* window);
    void removeWindow(const QString& address);

    signals:
    void windowsChanged();

    private:
    QList<Window*> m_windows;
    Ipc m_ipc;
};
