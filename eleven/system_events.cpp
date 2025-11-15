#include "system_events.h"
#include <QThread>

WindowOpen::WindowOpen(core::OpenWindow const &window)
    : address(window.address()),
      workspaceName(QString::fromUtf8(window.workspace_name().cbegin(), window.workspace_name().size())),
      title(QString::fromUtf8(window.title().cbegin(), window.title().size())),
      className(QString::fromUtf8(window.class_().cbegin(), window.class_().size()))
{
}

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
                Q_EMIT QSystemEvents::instance()->workspaceCreated(event.into_raw()); // TODO: memory leak
                break;
            case core::EventKind::WorkspaceDestroyed:
                Q_EMIT QSystemEvents::instance()->workspaceRemoved(event->workspace_destroyed());
                break;
            case core::EventKind::WorkspaceFocused:
                Q_EMIT QSystemEvents::instance()->workspaceFocused(event->workspace_focused());
                break;
            case core::EventKind::WindowOpen: {
                WindowOpen windowOpen(*event->window_open());
                Q_EMIT QSystemEvents::instance()->windowOpen(windowOpen);
                break;
            }
            case core::EventKind::WindowClose: {
                Q_EMIT QSystemEvents::instance()->windowClose(event->window_close());
                break;
            }
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
