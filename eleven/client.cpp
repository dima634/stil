#include "client.h"

QClient::QClient(std::size_t address, const QString &className, const QString &workspaceName, QObject *parent)
    : QObject(parent), m_address(address), m_class(className), m_workspace(workspaceName)
{
}

std::size_t QClient::getAddress() const
{
    return m_address;
}

const QString &QClient::getClass() const
{
    return m_class;
}

const QString &QClient::getWorkspace() const
{
    return m_workspace;
}
