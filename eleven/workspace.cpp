#include "workspace.h"

QWorkspace::QWorkspace(std::int32_t id, const QString &name, QObject *parent)
    : QObject(parent), m_id(id), m_name(name), m_windows(this)
{
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

QHyprWindow *QWorkspace::removeWindow(std::size_t address)
{
    return m_windows.removeWindow(address);
}

void QWorkspace::addWindow(QHyprWindow *window)
{
    m_windows.addWindow(window);
}
