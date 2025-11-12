#pragma once

#include <QObject>

class QSystemEvents : public QObject
{
    Q_OBJECT

  public:
    QSystemEvents();
    static QSystemEvents *instance();

  signals:
    void workspaceCreated();
    void workspaceRemoved();
    void workspaceFocused();
};
