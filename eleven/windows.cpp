#include "windows.h"

QWindows::QWindows(QObject *parent) : QAbstractListModel(parent)
{
    // auto clients = core::get_hyprland_clients();

    // for (std::size_t i = 0; i < clients.size(); ++i)
    // {
    //     const auto &client = clients[i];
    //     const std::size_t address = client.address();
    //     const auto className = QString::fromUtf8(client.class_().cbegin(), client.class_().size());
    //     const auto workspaceName = QString::fromUtf8(client.workspace_name().cbegin(),
    //     client.workspace_name().size()); auto qclient = new QHyprWindow(address, className, workspaceName, this);
    //     m_windows.append(qclient);
    // }

    // connect(QSystemEvents::instance(), &QSystemEvents::windowOpen, this, [this](core::WindowOpened window) {
    //     const QString className = window.class_name.c_str();
    //     const QString workspaceName = window.workspace_name.c_str();

    //     int row = m_windows.count();
    //     beginInsertRows(QModelIndex(), row, row);
    //     auto client = new QHyprWindow(window.address, className, workspaceName, this);
    //     m_windows.append(client);
    //     endInsertRows();
    // });

    // connect(QSystemEvents::instance(), &QSystemEvents::windowClose, this, [this](std::size_t windowAddress) {
    //     auto it = std::find_if(m_windows.begin(), m_windows.end(), [windowAddress](const QHyprWindow *client) {
    //         return client->getAddress() == windowAddress;
    //     });

    //     if (it != m_windows.end())
    //     {
    //         int row = std::distance(m_windows.begin(), it);
    //         beginRemoveRows(QModelIndex(), row, row);
    //         (*it)->deleteLater();
    //         m_windows.erase(it);
    //         endRemoveRows();
    //     }
    // });
}

int QWindows::rowCount(const QModelIndex &parent) const
{
    if (parent.isValid())
    {
        return 0;
    }
    return m_windows.count();
}

QVariant QWindows::data(const QModelIndex &index, int role) const
{
    if (!index.isValid() || index.row() >= m_windows.count())
    {
        return QVariant();
    }

    QHyprWindow *window = m_windows.at(index.row());

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

QHyprWindow *QWindows::removeWindow(std::size_t address)
{
    auto it = std::find_if(m_windows.begin(), m_windows.end(),
                           [address](const QHyprWindow *window) { return window->getAddress() == address; });

    if (it != m_windows.end())
    {
        auto *window = *it;
        window->setParent(nullptr);

        int row = std::distance(m_windows.begin(), it);
        beginRemoveRows(QModelIndex(), row, row);
        m_windows.erase(it);
        endRemoveRows();

        return window;
    }

    return nullptr;
}

void QWindows::addWindow(QHyprWindow *window)
{
    window->setParent(this);
    int row = m_windows.count();
    beginInsertRows(QModelIndex(), row, row);
    m_windows.append(window);
    endInsertRows();
}