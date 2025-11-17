#include "window.h"

QWindow::QWindow(std::size_t address, const QString &className, const QString &workspaceName, QObject *parent)
    : QObject(parent), m_address(address), m_class(className), m_workspace(workspaceName)
{
}

std::size_t QWindow::getAddress() const
{
    return m_address;
}

const QString &QWindow::getClass() const
{
    return m_class;
}

const QString &QWindow::getWorkspace() const
{
    return m_workspace;
}
