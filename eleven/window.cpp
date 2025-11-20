#include "window.h"
#include "system_events.h"
#include <stil_core/src/application.rs.h>

QHyprWindow::QHyprWindow(std::size_t address, const QString &className, const QString &workspaceName, QObject *parent)
    : QObject(parent), m_address(address), m_class(className), m_workspace(workspaceName)
{
    auto &appManager = core::app::application_manager();
    const auto *app = appManager.find_by_wmclass(className.toStdString().c_str());
    if (app != nullptr)
    {
        m_name = QString::fromUtf8(app->name().data(), app->name().size());
        m_iconPath = app->icon_path().c_str();
        m_iconPath.prepend("file://");
    }
    else
    {
        m_name = className;
        m_iconPath = "";
    }

    connect(QSystemEvents::instance(), &QSystemEvents::windowMoved, this, [this, address](core::WindowMoved event) {
        if (event.address == m_address)
        {
            m_workspace = event.workspace_name.c_str();
            Q_EMIT workspaceChanged();
        }
    });
}

std::size_t QHyprWindow::getAddress() const
{
    return m_address;
}

const QString &QHyprWindow::getClass() const
{
    return m_class;
}

const QString &QHyprWindow::getWorkspace() const
{
    return m_workspace;
}

const QString &QHyprWindow::getName() const
{
    return m_name;
}

const QString &QHyprWindow::getIconPath() const
{
    return m_iconPath;
}
