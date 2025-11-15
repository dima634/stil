#pragma once

#include <QObject>
#include <core/src/ffi/system_events.rs.h>

struct WindowOpen
{
    std::size_t address;
    QString workspaceName;
    QString title;
    QString className;

    WindowOpen(const core::OpenWindow &window);
};

class QSystemEvents : public QObject
{
    Q_OBJECT

  public:
    QSystemEvents();
    static QSystemEvents *instance();

  signals:
    void workspaceCreated(const core::Event *event);
    void workspaceRemoved(std::int32_t workspaceId);
    void workspaceFocused(std::int32_t workspaceId);
    void windowOpen(WindowOpen event);
    void windowClose(std::size_t windowAddress);
};
