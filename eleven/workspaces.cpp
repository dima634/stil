#include "workspaces.h"
#include "system_events.h"
#include <QtLogging>
#include <algorithm>
#include <stil_core/src/ffi.rs.h>
#include <stil_core/src/system_events.rs.h>

QWorkspaces::QWorkspaces(QObject *parent) : QAbstractListModel(parent)
{
    auto workspaces = core::desktop::get_workspaces();
    std::size_t currentId = core::desktop::get_current_workspace_id();

    for (std::size_t i = 0; i < workspaces.size(); ++i)
    {
        auto &ws = workspaces[i];
        auto qws = new QWorkspace(ws.id, ws.name.c_str(), this);
        m_workspaces.append(qws);

        if (ws.id == currentId)
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

    connect(QSystemEvents::instance(), &QSystemEvents::windowOpen, this, [this](core::WindowOpened window) {
        const QString workspaceName = window.workspace_name.c_str();
        QWorkspace *workspace = findWorkspaceByName(workspaceName);
        Q_ASSERT(workspace != nullptr);
        auto *client = new QHyprWindow(window.address);
        workspace->addWindow(client);
    });

    connect(QSystemEvents::instance(), &QSystemEvents::windowClose, this, [this](std::size_t windowAddress) {
        auto *window = findWindowByAddress(windowAddress);
        if (window == nullptr)
        {
            return;
        }

        auto *removed = removeWindow(windowAddress);
        removed->deleteLater();
    });

    connect(QSystemEvents::instance(), &QSystemEvents::windowMoved, this, [this](core::WindowMoved event) {
        QHyprWindow *movedWindow = removeWindow(event.address);
        Q_ASSERT(movedWindow != nullptr);
        QWorkspace *newWorkspace = findWorkspaceByName(event.workspace_name.c_str());
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

QHyprWindow *QWorkspaces::findWindowByAddress(std::size_t address) const
{
    for (auto *workspace : m_workspaces)
    {
        auto *window = workspace->getWindowByAddress(address);
        if (window)
        {
            return window;
        }
    }
    return nullptr;
}

QWorkspace *QWorkspaces::findWorkspaceByName(const QString &name) const
{
    for (auto *workspace : m_workspaces)
    {
        if (workspace->getName() == name)
        {
            return workspace;
        }
    }
    return nullptr;
}
