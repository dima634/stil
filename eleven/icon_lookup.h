#pragma once

#include <QtQmlIntegration/QtQmlIntegration>
#include <stil_core/src/freedesktop/ffi.rs.h>

class QIconLookup : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("IconLookup cannot be created directly")

  public:
    Q_INVOKABLE QString find(const QString &iconName, quint32 size);

  private:
    rust::Box<core::IconLookup> m_iconLookup = core::IconLookup::create();
};
