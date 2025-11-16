#pragma once

#include <QtQmlIntegration/QtQmlIntegration>

/// @brief Hyprland client
class QClient : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_UNCREATABLE("Clients are managed by Hyprland")

    Q_PROPERTY(std::size_t address READ getAddress CONSTANT)
    Q_PROPERTY(QString className READ getClass CONSTANT)
    Q_PROPERTY(QString workspaceName READ getWorkspace CONSTANT)

  public:
    explicit QClient(std::size_t address, const QString &className, const QString &workspaceName,
                     QObject *parent = nullptr);

    std::size_t getAddress() const;
    const QString &getClass() const;
    const QString &getWorkspace() const;

  private:
    std::size_t m_address;
    QString m_class;
    QString m_workspace;
};
