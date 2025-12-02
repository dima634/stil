#include "window.h"
#include "system_events.h"
#include <stil_core/src/ffi.rs.h>

QHyprWindow::QHyprWindow(std::size_t address, QObject *parent) : QObject(parent), m_address(address)
{
    auto window = core::desktop::get_window(address);
    m_class = window.class_name.c_str();
    m_iconPath = window.icon.c_str();
    m_iconPath.prepend("file://");
    m_focused = window.is_focused;

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

const QString &QHyprWindow::getIconPath() const
{
    return m_iconPath;
}

bool QHyprWindow::isFocused() const
{
    return m_focused;
}
