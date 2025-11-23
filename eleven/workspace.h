#pragma once

#include "windows.h"
#include <QtQmlIntegration/QtQmlIntegration>

class QWorkspace : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_UNCREATABLE("Workspaces are managed by Hyprland")

    Q_PROPERTY(std::int32_t id READ getId CONSTANT)
    Q_PROPERTY(QString name READ getName CONSTANT)
    Q_PROPERTY(const QWindows *windows READ getWindows CONSTANT)

  public:
    explicit QWorkspace(std::int32_t id, const QString &name, QObject *parent = nullptr);

    std::int32_t getId() const;
    const QString &getName() const;
    const QWindows *getWindows() const;

    QHyprWindow *getWindowByAddress(std::size_t address) const;
    QHyprWindow *removeWindow(std::size_t address);
    void addWindow(QHyprWindow *window);
    bool hasWindow(std::size_t address) const;

  private:
    std::int32_t m_id;
    QString m_name;
    QWindows m_windows;
};