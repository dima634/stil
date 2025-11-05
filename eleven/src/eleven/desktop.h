#pragma once

#include <QtCore/QObject>
#include <QtQmlIntegration/QtQmlIntegration>

class Desktop: public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString name MEMBER m_name)
    QML_ELEMENT
    QML_SINGLETON

    private:
    QString m_name = "Hello, Eleven Desktop!";
};
