#pragma once

#include "app.h"
#include <QAbstractListModel>
#include <QtQmlIntegration>

class QPinnedApps : public QAbstractListModel
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("Cannot be created in QML")

  public:
    explicit QPinnedApps(QObject *parent = nullptr);

    // QAbstractListModel interface
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QHash<int, QByteArray> roleNames() const override;

  private:
    QList<QApp *> m_apps;
};
