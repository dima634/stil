#pragma once

#include <QtCore/QObject>
#include <QtQmlIntegration/QtQmlIntegration>

class Desktop: public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString name READ default WRITE default BINDABLE bindableName)
    QML_ELEMENT
    QML_SINGLETON

    public:
    QBindable<QString> bindableName() { return &m_name; }
    void setName(const QString &name) { m_name = name; }

    signals:
    void nameChanged();

    private:
    Q_OBJECT_BINDABLE_PROPERTY_WITH_ARGS(Desktop, QString, m_name, "Hello, from plugin", &Desktop::nameChanged);
};
