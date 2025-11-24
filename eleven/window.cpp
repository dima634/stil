#include "window.h"
#include "system_events.h"
#include <stil_core/src/application.rs.h>

QHyprWindow::QHyprWindow(std::size_t address, const QString &className, QObject *parent)
    : QObject(parent), m_address(address), m_class(className)
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

    connect(QSystemEvents::instance(), &QSystemEvents::windowFocused, this, [this](std::size_t windowAddress) {
        if (windowAddress == m_address && !m_focused)
        {
            m_focused = true;
            Q_EMIT focusChanged();
        }
        else if (windowAddress != m_address && m_focused)
        {
            m_focused = false;
            Q_EMIT focusChanged();
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

const QString &QHyprWindow::getName() const
{
    return m_name;
}

const QString &QHyprWindow::getIconPath() const
{
    return m_iconPath;
}

bool QHyprWindow::isFocused() const
{
    return m_focused;
}

bool QHyprWindow::isRunning() const
{
    return m_running;
}
