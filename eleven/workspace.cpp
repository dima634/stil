#include "workspace.h"
#include <stil_core/src/ffi.rs.h>

QWorkspace::QWorkspace(std::int32_t id, QString name, QObject *parent)
    : QObject(parent), m_id(id), m_name(std::move(name)), m_windows(this)
{
    auto windows = core::desktop::get_workspace_windows(id);
    for (auto windowAddress : windows)
    {
        auto *window = new QHyprWindow(windowAddress, this);
        m_windows.addWindow(window);
    }
}

const QWindows *QWorkspace::getWindows() const
{
    return &m_windows;
}

std::int32_t QWorkspace::getId() const
{
    return m_id;
}

const QString &QWorkspace::getName() const
{
    return m_name;
}

QHyprWindow *QWorkspace::getWindowByAddress(std::size_t address) const
{
    return m_windows.getByAddress(address);
}

QHyprWindow *QWorkspace::removeWindow(std::size_t address)
{
    return m_windows.removeWindow(address);
}

void QWorkspace::addWindow(QHyprWindow *window)
{
    m_windows.addWindow(window);
}
