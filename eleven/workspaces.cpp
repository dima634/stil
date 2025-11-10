#include "workspaces.h"
#include <algorithm>
#include <core/src/ffi/mod.rs.h>
#include <unordered_set>

QWorkspaces::QWorkspaces(QObject *parent) : QObject(parent)
{
    auto workspaces = get_workspace_registry().workspaces();
    std::size_t currentId = get_workspace_registry().current_workspace_id();

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
}

QQmlListProperty<QWorkspace> QWorkspaces::getAllWorkspaces()
{
    return QQmlListProperty<QWorkspace>(this, &m_workspaces);
}

QWorkspace *QWorkspaces::getCurrentWorkspace() const
{
    return m_currentWorkspace;
}
