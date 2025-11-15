#include "client.h"

QClient::QClient(std::size_t address, const QString &className, std::int32_t workspace, QObject *parent)
    : QObject(parent), m_address(address), m_class(className), m_workspace(workspace)
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

std::int32_t QClient::getWorkspace() const
{
    return m_workspace;
}
