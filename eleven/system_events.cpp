#include "system_events.h"
#include <QThread>

QSystemEvents::QSystemEvents()
{
    QThread *systemEventListenerThread = QThread::create([]() {
        auto dispatcher = core::SystemEvents::create();
        while (true)
        {
            auto event = dispatcher->next();
            switch (event->kind())
            {
            case core::EventKind::WorkspaceCreated:
                Q_EMIT QSystemEvents::instance()->workspaceCreated(event->workspace_created()); // TODO: memory leak
                break;
            case core::EventKind::WorkspaceDestroyed:
                Q_EMIT QSystemEvents::instance()->workspaceRemoved(event->workspace_destroyed());
                break;
            case core::EventKind::WorkspaceFocused:
                Q_EMIT QSystemEvents::instance()->workspaceFocused(event->workspace_focused());
                break;
            case core::EventKind::WindowOpen:
                Q_EMIT QSystemEvents::instance()->windowOpen(event->window_opened());
                break;
            case core::EventKind::WindowClose:
                Q_EMIT QSystemEvents::instance()->windowClose(event->window_closed());
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
