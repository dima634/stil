#include "windows.h"
#include "system_events.h"
#include <stil_core/src/ffi/hyprland.rs.h>

QWindows::QWindows(QObject *parent) : QAbstractListModel(parent)
{
    auto clients = core::get_hyprland_clients();

    for (std::size_t i = 0; i < clients.size(); ++i)
    {
        const auto &client = clients[i];
        const std::size_t address = client.address();
        const auto className = QString::fromUtf8(client.class_().cbegin(), client.class_().size());
        const auto workspaceName = QString::fromUtf8(client.workspace_name().cbegin(), client.workspace_name().size());
        auto qclient = new QHyprWindow(address, className, workspaceName, this);
        m_clients.append(qclient);
    }

    connect(QSystemEvents::instance(), &QSystemEvents::windowOpen, this, [this](core::WindowOpened window) {
        const QString className = window.class_name.c_str();
        const QString workspaceName = window.workspace_name.c_str();

        int row = m_clients.count();
        beginInsertRows(QModelIndex(), row, row);
        auto client = new QHyprWindow(window.address, className, workspaceName, this);
        m_clients.append(client);
        endInsertRows();
    });

    connect(QSystemEvents::instance(), &QSystemEvents::windowClose, this, [this](std::size_t windowAddress) {
        auto it = std::find_if(m_clients.begin(), m_clients.end(), [windowAddress](const QHyprWindow *client) {
            return client->getAddress() == windowAddress;
        });

        if (it != m_clients.end())
        {
            int row = std::distance(m_clients.begin(), it);
            beginRemoveRows(QModelIndex(), row, row);
            (*it)->deleteLater();
            m_clients.erase(it);
            endRemoveRows();
        }
    });
}

int QWindows::rowCount(const QModelIndex &parent) const
{
    if (parent.isValid())
    {
        return 0;
    }
    return m_clients.count();
}

QVariant QWindows::data(const QModelIndex &index, int role) const
{
    if (!index.isValid() || index.row() >= m_clients.count())
    {
        return QVariant();
    }

    QHyprWindow *window = m_clients.at(index.row());

    switch (role)
    {
    case Qt::DisplayRole:
        return QVariant::fromValue(window);
    default:
        return QVariant();
    }
}

QHash<int, QByteArray> QWindows::roleNames() const
{
    QHash<int, QByteArray> roles;
    roles[Qt::DisplayRole] = "window";
    return roles;
}
