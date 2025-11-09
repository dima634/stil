#pragma once

#include <QtQmlIntegration/QtQmlIntegration>

class QWorkspace : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_UNCREATABLE("Workspaces are managed by Hyprland")

    Q_PROPERTY(std::int32_t id READ getId CONSTANT)
    Q_PROPERTY(QString name READ getName CONSTANT)

  public:
    explicit QWorkspace(std::int32_t id, const QString &name, QObject *parent = nullptr)
        : QObject(parent), m_id(id), m_name(name)
    {
    }

    std::int32_t getId() const
    {
        return m_id;
    }

    const QString &getName() const
    {
        return m_name;
    }

  private:
    std::int32_t m_id;
    QString m_name;
};