#include "icon_lookup.h"

QString QIconLookup::find(const QString &iconName, quint32 size)
{
    auto iconPath = m_iconLookup->find_icon(iconName.toStdString().c_str(), size);
    return iconPath.c_str();
}
