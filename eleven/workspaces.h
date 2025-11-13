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
    Q_PROPERTY(QList<QWorkspace *> all READ getAll NOTIFY allChanged)
    Q_PROPERTY(QWorkspace *current READ getCurrentWorkspace NOTIFY currentChanged)

  public:
    explicit QWorkspaces(QObject *parent = nullptr);

    const QList<QWorkspace *> &getAll() const;
    QWorkspace *getCurrentWorkspace() const;

  signals:
    void currentChanged();
    void allChanged();

  private:
    QList<QWorkspace *> m_workspaces;
    QWorkspace *m_currentWorkspace = nullptr;

    bool removeWorkspace(std::int32_t workspaceId);
};
