#pragma once

#include <core/src/lib.cxxqt.h>
#include <QtQmlIntegration/QtQmlIntegration>

class MyObject : public my_object::MyObject
{
    Q_OBJECT
    QML_ELEMENT
};
