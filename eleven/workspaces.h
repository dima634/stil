#pragma once

#include "workspace.h"
#include <QQmlListProperty>
#include <QtQmlIntegration/QtQmlIntegration>

class QWorkspaces : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("Workspaces are managed by Hyprland")
    Q_PROPERTY(QQmlListProperty<QWorkspace> workspaces READ getWorkspaces CONSTANT)

  public:
    explicit QWorkspaces(QObject *parent = nullptr);
    QQmlListProperty<QWorkspace> getWorkspaces()
    {
        return QQmlListProperty<QWorkspace>(this, &m_workspaces);
    }

  private:
    QList<QWorkspace *> m_workspaces;
};
