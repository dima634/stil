#pragma once

#include <QtQmlIntegration>

class QApp : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_UNCREATABLE("Cannot be created in QML")
    Q_PROPERTY(QString appId READ getAppId CONSTANT)
    Q_PROPERTY(QString name READ getName CONSTANT)
    Q_PROPERTY(QString icon READ getIcon CONSTANT)

  public:
    QApp(QString appId, QString name, QString icon, QObject *parent = nullptr);
    const QString &getAppId() const;
    const QString &getName() const;
    const QString &getIcon() const;

  private:
    QString m_appId;
    QString m_name;
    QString m_icon;
};
