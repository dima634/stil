#include "workspaces.h"
#include "system_events.h"
#include <algorithm>
#include <core/src/ffi/mod.rs.h>

QWorkspaces::QWorkspaces(QObject *parent) : QObject(parent)
{
    updateWorkspaces();

    connect(QSystemEvents::instance(), &QSystemEvents::workspaceCreated, this, [this]() { updateWorkspaces(); });

    connect(QSystemEvents::instance(), &QSystemEvents::workspaceRemoved, this, [this]() { updateWorkspaces(); });

    connect(QSystemEvents::instance(), &QSystemEvents::workspaceFocused, this, [this]() {
        updateWorkspaces();
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

void QWorkspaces::updateWorkspaces()
{
    m_workspaces.clear();
    m_currentWorkspace = nullptr;

    auto workspaces = Workspaces::lock();
    std::size_t currentId = workspaces->current_workspace_id();
    const auto &allWorkspaces = workspaces->all();

    for (std::size_t i = 0; i < allWorkspaces.size(); ++i)
    {
        const auto &ws = allWorkspaces[i];
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

    Q_EMIT allChanged();
}
