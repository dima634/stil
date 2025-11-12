#include "system_events.h"
#include <QThread>
#include <core/src/ffi/mod.rs.h>

QSystemEvents::QSystemEvents()
{
    QThread *systemEventListenerThread = QThread::create([]() {
        auto dispatcher = SystemEvents::create();
        while (true)
        {
            auto event = dispatcher->next();
            switch (event->kind())
            {
            case EventKind::WorkspaceCreated:
                Q_EMIT QSystemEvents::instance()->workspaceCreated();
                break;
            case EventKind::WorkspaceDestroyed:
                Q_EMIT QSystemEvents::instance()->workspaceRemoved();
                break;
            case EventKind::WorkspaceFocused:
                Q_EMIT QSystemEvents::instance()->workspaceFocused();
                break;
            default:
                break;
            }
        }
    });
    this->moveToThread(systemEventListenerThread);
    systemEventListenerThread->start();
}

QSystemEvents *QSystemEvents::instance()
{
    static QSystemEvents instance;
    return &instance;
}
