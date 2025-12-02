#include "windows.h"

QWindows::QWindows(QObject *parent) : QAbstractListModel(parent)
{
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
        return QVariant{};
    }

    if (role == Qt::DisplayRole)
    {
        QHyprWindow *window = m_windows.at(index.row());
        return QVariant::fromValue(window);
    }
    else
    {
        return QVariant{};
    }
}

QHash<int, QByteArray> QWindows::roleNames() const
{
    QHash<int, QByteArray> roles;
    roles[Qt::DisplayRole] = "window";
    return roles;
}

QHyprWindow *QWindows::getByAddress(std::size_t address) const
{
    for (QHyprWindow *window : m_windows)
    {
        if (window->getAddress() == address)
        {
            return window;
        }
    }
    return nullptr;
}

QHyprWindow *QWindows::removeWindow(std::size_t address)
{
    auto it = std::ranges::find_if(m_windows,
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