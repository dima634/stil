#pragma once

#include "window.h"
#include <QtQmlIntegration/QtQmlIntegration>

class QWindows : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("Clients are managed by Hyprland")
    Q_PROPERTY(QList<QHyprWindow *> all READ getAll NOTIFY allChanged)

  public:
    explicit QWindows(QObject *parent = nullptr);
    const QList<QHyprWindow *> &getAll() const;

  signals:
    void allChanged();

  private:
    QList<QHyprWindow *> m_clients;
};
