#pragma once

#include <QObject>
#include <core/src/ffi/system_events.rs.h>

class QSystemEvents : public QObject
{
    Q_OBJECT

  public:
    QSystemEvents();
    static QSystemEvents *instance();

  signals:
    void workspaceCreated(core::WorkspaceCreated event);
    void workspaceRemoved(std::int32_t workspaceId);
    void workspaceFocused(std::int32_t workspaceId);
    void windowOpen(core::WindowOpened event);
    void windowClose(std::size_t windowAddress);
};
