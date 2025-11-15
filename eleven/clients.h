#pragma once

#include "client.h"
#include <QtQmlIntegration/QtQmlIntegration>

class QClients : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("Clients are managed by Hyprland")
    Q_PROPERTY(QList<QClient *> all READ getAll NOTIFY allChanged)

  public:
    explicit QClients(QObject *parent = nullptr);
    const QList<QClient *> &getAll() const;

  signals:
    void allChanged();

  private:
    QList<QClient *> m_clients;
};
