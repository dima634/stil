#pragma once

#include <QtQmlIntegration>

class QKeyboard : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("QKeyboard cannot be created in QML")
    Q_PROPERTY(QString currentLayout READ getCurrentLayout NOTIFY currentLayoutChanged)

  public:
    explicit QKeyboard(QObject *parent = nullptr);
    const QString &getCurrentLayout() const;

  signals:
    void currentLayoutChanged();

  public:
    QString m_currentLayout = "TODO unknown";
};
