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
    void workspaceCreated(rust::Box<core::Event> event);
    void workspaceRemoved(std::int32_t workspaceId);
    void workspaceFocused();
};
