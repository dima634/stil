#pragma once

#include <QtQmlIntegration/QtQmlIntegration>

/// @brief Hyprland client
class QHyprWindow : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_UNCREATABLE("Windows are managed by Hyprland")

    Q_PROPERTY(qsizetype address READ getAddress CONSTANT)
    Q_PROPERTY(QString className READ getClass CONSTANT)
    Q_PROPERTY(QString name READ getName CONSTANT)
    Q_PROPERTY(QString iconPath READ getIconPath CONSTANT)
    Q_PROPERTY(bool focused READ isFocused NOTIFY focusChanged)
    Q_PROPERTY(bool running READ isRunning NOTIFY runningChanged)
    Q_PROPERTY(bool pinned READ isPinned NOTIFY pinnedChanged)

  public:
    explicit QHyprWindow(std::size_t address, const QString &className, QObject *parent = nullptr);

    std::size_t getAddress() const;
    const QString &getClass() const;
    const QString &getName() const;
    const QString &getIconPath() const;
    bool isFocused() const;
    bool isRunning() const;
    void setIsRunning(bool running);
    bool isPinned() const;
    void setIsPinned(bool pinned);

  signals:
    void focusChanged();
    void runningChanged();
    void pinnedChanged();

  private:
    bool m_focused = false;
    bool m_pinned = false;
    bool m_running = true;
    std::size_t m_address = 0;
    QString m_class;
    QString m_name;
    QString m_iconPath;
};
