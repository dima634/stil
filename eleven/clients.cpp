#include "clients.h"
#include "system_events.h"
#include <core/src/ffi/hyprland.rs.h>

QClients::QClients(QObject *parent) : QObject(parent)
{
    auto clients = core::get_hyprland_clients();

    for (std::size_t i = 0; i < clients.size(); ++i)
    {
        const auto &client = clients[i];
        const std::size_t address = client.address();
        const auto className = QString::fromUtf8(client.class_().cbegin(), client.class_().size());
        auto qclient = new QClient(address, className, client.workspace(), this);
        m_clients.append(qclient);
    }

    connect(QSystemEvents::instance(), &QSystemEvents::windowOpen, this, [this](core::WindowOpened window) {
        QString className = window.className.c_str();
        auto client = new QClient(window.address, className, window.workspace, this);
        m_clients.append(client);
        Q_EMIT allChanged();
    });

    connect(QSystemEvents::instance(), &QSystemEvents::windowClose, this, [this](std::size_t windowAddress) {
        auto it = std::find_if(m_clients.begin(), m_clients.end(), [windowAddress](const QClient *client) {
            return client->getAddress() == windowAddress;
        });

        if (it != m_clients.end())
        {
            m_clients.erase(it);
            Q_EMIT allChanged();
        }
    });
}

const QList<QClient *> &QClients::getAll() const
{
    return m_clients;
}
