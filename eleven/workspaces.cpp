#include "workspaces.h"
#include "system_events.h"
#include <QtLogging>
#include <core/src/ffi/mod.rs.h>
#include <core/src/ffi/system_events.rs.h>
#include <algorithm>

QWorkspaces::QWorkspaces(QObject *parent) : QObject(parent)
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

        if (ws.id() == currentId)
        {
            m_currentWorkspace = qws;
        }
    }

    std::sort(m_workspaces.begin(), m_workspaces.end(),
              [](QWorkspace *a, QWorkspace *b) { return a->getId() < b->getId(); });

    connect(QSystemEvents::instance(), &QSystemEvents::workspaceCreated, this, [this](const core::Event *event) {
        auto &workspace = event->workspace_created();
        if (removeWorkspace(workspace.id()))
        {
            qWarning("workspace with id %d already exists, removing...", workspace.id());
        }

        QString name = QString::fromUtf8(workspace.name().cbegin(), workspace.name().size());
        auto qws = new QWorkspace(workspace.id(), name, this);
        auto insertPos = std::lower_bound(m_workspaces.cbegin(), m_workspaces.cend(), workspace.id(),
                                          [](const QWorkspace *ws, std::int32_t id) { return ws->getId() < id; });
        m_workspaces.insert(insertPos, qws);

        Q_EMIT allChanged();
    });

    connect(QSystemEvents::instance(), &QSystemEvents::workspaceRemoved, this, [this](std::int32_t workspaceId) {
        removeWorkspace(workspaceId);
        Q_EMIT allChanged();
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
}

const QList<QWorkspace *> &QWorkspaces::getAll() const
{
    return m_workspaces;
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

    if (m_currentWorkspace == *it)
    {
        m_currentWorkspace = nullptr;
    }

    (*it)->deleteLater();
    m_workspaces.erase(it);

    return true;
}
