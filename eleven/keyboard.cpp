#include "keyboard.h"
#include "system_events.h"

QKeyboard::QKeyboard(QObject *parent) : QObject(parent)
{
    connect(QSystemEvents::instance(), &QSystemEvents::keyboardLayoutChanged, this, [this](QString layoutName) {
        m_currentLayout = std::move(layoutName);
        Q_EMIT currentLayoutChanged();
    });
}

const QString &QKeyboard::getCurrentLayout() const
{
    return m_currentLayout;
}
