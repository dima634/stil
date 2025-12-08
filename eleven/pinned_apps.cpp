#include "pinned_apps.h"
#include <stil_core/src/ffi.rs.h>

QPinnedApps::QPinnedApps(QObject *parent) : QAbstractListModel(parent)
{
    auto pinnedApps = core::desktop::get_pinned_apps();

    for (auto app : pinnedApps)
    {
        QString icon = app.icon.c_str();
        icon.prepend("file://");
        auto *application = new QApp(app.id.c_str(), app.name.c_str(), icon, this);
        m_apps.append(application);
    }
}

int QPinnedApps::rowCount(const QModelIndex &parent) const
{
    if (parent.isValid())
    {
        return 0;
    }
    return m_apps.count();
}

QVariant QPinnedApps::data(const QModelIndex &index, int role) const
{
    if (!index.isValid() || index.row() >= m_apps.count())
    {
        return QVariant{};
    }

    if (role == Qt::DisplayRole)
    {
        QApp *app = m_apps.at(index.row());
        return QVariant::fromValue(app);
    }
    else
    {
        return QVariant{};
    }
}

QHash<int, QByteArray> QPinnedApps::roleNames() const
{
    QHash<int, QByteArray> roles;
    roles[Qt::DisplayRole] = "app";
    return roles;
}