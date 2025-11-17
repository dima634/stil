#include "windows.h"
#include "system_events.h"
#include <stil_core/src/ffi/hyprland.rs.h>

QWindows::QWindows(QObject *parent) : QObject(parent)
{
    auto clients = core::get_hyprland_clients();

    for (std::size_t i = 0; i < clients.size(); ++i)
    {
        const auto &client = clients[i];
        const std::size_t address = client.address();
        const auto className = QString::fromUtf8(client.class_().cbegin(), client.class_().size());
        const auto workspaceName = QString::fromUtf8(client.workspace_name().cbegin(), client.workspace_name().size());
        auto qclient = new QWindow(address, className, workspaceName, this);
        m_clients.append(qclient);
    }

    connect(QSystemEvents::instance(), &QSystemEvents::windowOpen, this, [this](core::WindowOpened window) {
        const QString className = window.class_name.c_str();
        const QString workspaceName = window.workspace_name.c_str();
        auto client = new QWindow(window.address, className, workspaceName, this);
        m_clients.append(client);
        Q_EMIT allChanged();
    });

    connect(QSystemEvents::instance(), &QSystemEvents::windowClose, this, [this](std::size_t windowAddress) {
        auto it = std::find_if(m_clients.begin(), m_clients.end(), [windowAddress](const QWindow *client) {
            return client->getAddress() == windowAddress;
        });

        if (it != m_clients.end())
        {
            m_clients.erase(it);
            Q_EMIT allChanged();
        }
    });
}

const QList<QWindow *> &QWindows::getAll() const
{
    return m_clients;
}
