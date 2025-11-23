#include "workspaces.h"
#include "system_events.h"
#include <QtLogging>
#include <algorithm>
#include <stil_core/src/ffi/hyprland.rs.h>
#include <stil_core/src/ffi/mod.rs.h>
#include <stil_core/src/ffi/system_events.rs.h>

QWorkspaces::QWorkspaces(QObject *parent) : QAbstractListModel(parent)
{
    auto currentState = core::get_workspaces_state();
    std::size_t currentId = currentState->current_workspace_id();
    const auto &workspaces = currentState->workspaces();

    for (std::size_t i = 0; i < workspaces.size(); ++i)
    {
        const auto &ws = workspaces[i];
        const auto &name = QString::fromUtf8(ws.name().cbegin(), ws.name().size());
        auto qws = new QWorkspace(ws.id(), name, this);
        m_workspaces.append(qws);
        m_nameToWorkspace.insert(name, qws);
        m_idToWorkspace.insert(ws.id(), qws);

        if (ws.id() == currentId)
        {
            m_currentWorkspace = qws;
        }
    }

    std::sort(m_workspaces.begin(), m_workspaces.end(),
              [](QWorkspace *a, QWorkspace *b) { return a->getId() < b->getId(); });

    connect(
        QSystemEvents::instance(), &QSystemEvents::workspaceCreated, this, [this](core::WorkspaceCreated workspace) {
            if (removeWorkspace(workspace.id))
            {
                qWarning("workspace with id %d already exists, removing...", workspace.id);
            }

            // Insert keeping the list ordered
            auto insertPos = std::lower_bound(m_workspaces.begin(), m_workspaces.end(), workspace.id,
                                              [](const QWorkspace *ws, std::int32_t id) { return ws->getId() < id; });
            int row = std::distance(m_workspaces.begin(), insertPos);

            beginInsertRows(QModelIndex(), row, row);
            QString name = workspace.name.c_str();
            auto qws = new QWorkspace(workspace.id, name, this);
            m_workspaces.insert(insertPos, qws);
            m_nameToWorkspace.insert(name, qws);
            m_idToWorkspace.insert(workspace.id, qws);
            endInsertRows();
        });

    connect(QSystemEvents::instance(), &QSystemEvents::workspaceRemoved, this, [this](std::int32_t workspaceId) {
        auto it = std::find_if(m_workspaces.cbegin(), m_workspaces.cend(),
                               [workspaceId](QWorkspace *ws) { return ws->getId() == workspaceId; });

        if (!removeWorkspace(workspaceId))
        {
            qWarning() << "Workspace" << workspaceId << "not found for removal";
        }
    });

    connect(QSystemEvents::instance(), &QSystemEvents::workspaceFocused, this, [this](std::int32_t workspaceId) {
        auto it = std::find_if(m_workspaces.cbegin(), m_workspaces.cend(),
                               [workspaceId](QWorkspace *ws) { return ws->getId() == workspaceId; });
        if (it == m_workspaces.cend())
        {
            qWarning("could not find workspace with id %d", workspaceId);
            return;
        }

        m_currentWorkspace = *it;
        Q_EMIT currentChanged();
    });

    auto hyprWindows = core::get_hyprland_clients();

    for (std::size_t i = 0; i < hyprWindows.size(); ++i)
    {
        const auto &client = hyprWindows[i];
        QWorkspace *workspace = m_idToWorkspace.value(client.workspace());
        Q_ASSERT(workspace != nullptr);
        const std::size_t address = client.address();
        const auto className = QString::fromUtf8(client.class_().cbegin(), client.class_().size());
        auto *window = new QHyprWindow(address, className);
        workspace->addWindow(window);
    }

    connect(QSystemEvents::instance(), &QSystemEvents::windowOpen, this, [this](core::WindowOpened window) {
        const QString workspaceName = window.workspace_name.c_str();
        QWorkspace *workspace = m_nameToWorkspace.value(workspaceName);
        Q_ASSERT(workspace != nullptr);
        const QString className = window.class_name.c_str();
        auto *client = new QHyprWindow(window.address, className);
        workspace->addWindow(client);
    });

    connect(QSystemEvents::instance(), &QSystemEvents::windowClose, this, [this](std::size_t windowAddress) {
        auto *removed = removeWindow(windowAddress);
        Q_ASSERT(removed != nullptr);
        removed->deleteLater();
    });

    connect(QSystemEvents::instance(), &QSystemEvents::windowMoved, this, [this](core::WindowMoved event) {
        QHyprWindow *movedWindow = removeWindow(event.address);
        Q_ASSERT(movedWindow != nullptr);
        QWorkspace *newWorkspace = m_idToWorkspace.value(event.workspace_id);
        Q_ASSERT(newWorkspace != nullptr);
        newWorkspace->addWindow(movedWindow);
    });
}

int QWorkspaces::rowCount(const QModelIndex &parent) const
{
    if (parent.isValid())
    {
        return 0;
    }
    return m_workspaces.count();
}

QVariant QWorkspaces::data(const QModelIndex &index, int role) const
{
    if (!index.isValid() || index.row() >= m_workspaces.count())
    {
        return QVariant();
    }

    QWorkspace *workspace = m_workspaces.at(index.row());

    switch (role)
    {
    case Qt::DisplayRole:
        return QVariant::fromValue(workspace);
    default:
        return QVariant();
    }
}

QHash<int, QByteArray> QWorkspaces::roleNames() const
{
    QHash<int, QByteArray> roles;
    roles[Qt::DisplayRole] = "modelData";
    return roles;
}

QWorkspace *QWorkspaces::getCurrentWorkspace() const
{
    return m_currentWorkspace;
}

bool QWorkspaces::removeWorkspace(std::int32_t workspaceId)
{
    auto it = std::find_if(m_workspaces.cbegin(), m_workspaces.cend(),
                           [workspaceId](QWorkspace *ws) { return ws->getId() == workspaceId; });

    if (it == m_workspaces.cend())
    {
        return false;
    }

    int row = std::distance(m_workspaces.cbegin(), it);
    beginRemoveRows(QModelIndex(), row, row);

    auto *workspace = *it;
    if (m_currentWorkspace == workspace)
    {
        m_currentWorkspace = nullptr;
    }

    workspace->deleteLater();
    m_nameToWorkspace.remove(workspace->getName());
    m_idToWorkspace.remove(workspaceId);
    m_workspaces.erase(it);

    endRemoveRows();

    return true;
}

QHyprWindow *QWorkspaces::removeWindow(std::size_t windowAddress)
{
    for (auto *workspace : m_workspaces)
    {
        auto *window = workspace->removeWindow(windowAddress);
        if (window)
        {
            return window;
        }
    }
    return nullptr;
}
