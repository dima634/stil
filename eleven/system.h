#pragma once

#include <QtQmlIntegration/QtQmlIntegration>

class QSystem : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("QSystem cannot be created in QML")

  public:
    Q_INVOKABLE bool poweroff() const;
};
