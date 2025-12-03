#include "app.h"

QApp::QApp(QString appId, QString name, QString icon, QObject *parent)
    : QObject(parent), m_appId(std::move(appId)), m_name(std::move(name)), m_icon(std::move(icon))
{
}

const QString &QApp::getAppId() const
{
    return m_appId;
}

const QString &QApp::getName() const
{
    return m_name;
}

const QString &QApp::getIcon() const
{
    return m_icon;
}
