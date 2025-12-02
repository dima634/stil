#include "memory.h"
#include <stil_core/src/system.rs.h>

QMemory::QMemory(QObject *parent) : QObject(parent), m_timer(new QTimer(this))
{
    connect(m_timer, &QTimer::timeout, this, [this]() {
        auto usage = core::memory::get_memory_usage();
        m_totalRam = usage.totalRam;
        m_usedRam = usage.usedRam;
        m_freeRam = usage.freeRam;
        m_availableRam = usage.availableRam;
        m_totalSwap = usage.totalSwap;
        m_usedSwap = usage.usedSwap;
        m_freeSwap = usage.freeSwap;
        Q_EMIT usageChanged();
    });

    m_timer->start(m_updateIntervalMs);
}

std::uint64_t QMemory::totalRam() const
{
    return m_totalRam;
}

std::uint64_t QMemory::usedRam() const
{
    return m_usedRam;
}

std::uint64_t QMemory::freeRam() const
{
    return m_freeRam;
}

std::uint64_t QMemory::availableRam() const
{
    return m_availableRam;
}

std::uint64_t QMemory::totalSwap() const
{
    return m_totalSwap;
}

std::uint64_t QMemory::usedSwap() const
{
    return m_usedSwap;
}

std::uint64_t QMemory::freeSwap() const
{
    return m_freeSwap;
}

int QMemory::getUpdateInterval() const
{
    return m_updateIntervalMs;
}

void QMemory::setUpdateInterval(int interval)
{
    if (m_updateIntervalMs != interval)
    {
        m_updateIntervalMs = interval;
        m_timer->setInterval(m_updateIntervalMs);
        Q_EMIT updateIntervalChanged();
    }
}
