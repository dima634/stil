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
    Q_PROPERTY(std::int32_t workspace READ getWorkspace CONSTANT)

  public:
    explicit QClient(std::size_t address, const QString &className, std::int32_t workspace, QObject *parent = nullptr);

    std::size_t getAddress() const;
    const QString &getClass() const;
    std::int32_t getWorkspace() const;

  private:
    std::size_t m_address;
    QString m_class;
    std::int32_t m_workspace;
};
