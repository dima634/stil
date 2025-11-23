#pragma once

#include "window.h"
#include <QAbstractListModel>
#include <QtQmlIntegration/QtQmlIntegration>

class QWindows : public QAbstractListModel
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("Clients are managed by Hyprland")

  public:
    explicit QWindows(QObject *parent = nullptr);

    // QAbstractListModel interface
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QHash<int, QByteArray> roleNames() const override;

  private:
    QList<QHyprWindow *> m_clients;
};
