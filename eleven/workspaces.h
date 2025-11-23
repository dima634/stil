#pragma once

#include "window.h"
#include "workspace.h"
#include <QAbstractListModel>
#include <QtQmlIntegration/QtQmlIntegration>

class QWorkspaces : public QAbstractListModel
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("Workspaces are managed by Hyprland")
    Q_PROPERTY(QWorkspace *current READ getCurrentWorkspace NOTIFY currentChanged)

  public:
    explicit QWorkspaces(QObject *parent = nullptr);

    // QAbstractListModel interface
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QHash<int, QByteArray> roleNames() const override;

    QWorkspace *getCurrentWorkspace() const;

  signals:
    void currentChanged();

  private:
    QList<QWorkspace *> m_workspaces;
    QMap<QString, QWorkspace *> m_nameToWorkspace;
    QMap<std::int32_t, QWorkspace *> m_idToWorkspace;
    QWorkspace *m_currentWorkspace = nullptr;

    bool removeWorkspace(std::int32_t workspaceId);
    QHyprWindow *removeWindow(std::size_t windowAddress);
};
