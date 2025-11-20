#pragma once

#include <QtQmlIntegration/QtQmlIntegration>

/// @brief Hyprland client
class QHyprWindow : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_UNCREATABLE("Windows are managed by Hyprland")

    Q_PROPERTY(std::size_t address READ getAddress CONSTANT)
    Q_PROPERTY(QString className READ getClass CONSTANT)
    Q_PROPERTY(QString name READ getName CONSTANT)
    Q_PROPERTY(QString iconPath READ getIconPath CONSTANT)
    Q_PROPERTY(QString workspaceName READ getWorkspace NOTIFY workspaceChanged)

  public:
    explicit QHyprWindow(std::size_t address, const QString &className, const QString &workspaceName,
                         QObject *parent = nullptr);

    std::size_t getAddress() const;
    const QString &getClass() const;
    const QString &getWorkspace() const;
    const QString &getName() const;
    const QString &getIconPath() const;

  signals:
    void workspaceChanged();

  private:
    std::size_t m_address = 0;
    QString m_class;
    QString m_workspace;
    QString m_name;
    QString m_iconPath;
};
