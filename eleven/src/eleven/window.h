#pragma once
    
#include <proto/hyprland.pb.h>
#include <QtCore/QObject>
#include <QtQmlIntegration/QtQmlIntegration>

class Window: public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString address READ default WRITE default BINDABLE bindableAddress)
    Q_PROPERTY(QString className READ default WRITE default BINDABLE bindableClassName)
    QML_ELEMENT
    QML_UNCREATABLE("Window cannot be created in QML")
    
    public:
    explicit Window(const protos::Client& proto, QObject* parent = nullptr) : QObject(parent) 
    {
        m_address = QString::fromStdString(proto.address());
        m_className = QString::fromStdString(proto.class_());
    }

    QBindable<QString> bindableAddress() { return &m_address; }
    const QString& getAddress() const { return m_address; }
    void setAddress(const QString& address) { m_address = address; }

    QBindable<QString> bindableClassName() { return &m_className; }
    const QString& getClassName() const { return m_className; }
    void setClassName(const QString& className) { m_className = className; }

    signals:
    void addressChanged();
    void classNameChanged();

    private:
    Q_OBJECT_BINDABLE_PROPERTY(Window, QString, m_address, &Window::addressChanged);
    Q_OBJECT_BINDABLE_PROPERTY(Window, QString, m_className, &Window::classNameChanged);
};
