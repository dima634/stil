#include "cpu.h"
#include <stil_core/src/system.rs.h>

QCpu::QCpu(QObject *parent) : QObject(parent)
{
    m_brand = core::cpu::get_brand().c_str();
    m_timer = new QTimer(this);
    m_timer->setInterval(m_updateIntervalMs);

    connect(m_timer, &QTimer::timeout, this, [this]() {
        auto usage = core::cpu::get_usage();
        m_totalUsage = usage.total;

        m_usagePerCore.clear();
        for (std::size_t i = 0; i < usage.num_cores; ++i)
        {
            m_usagePerCore.append(usage.cores[i]);
        }

        m_temperature = core::cpu::get_temp();

        Q_EMIT dataChanged();
    });

    m_timer->start();
}

const QString &QCpu::getBrand() const
{
    return m_brand;
}

const QList<float> &QCpu::getUsagePerCore() const
{
    return m_usagePerCore;
}

float QCpu::getTotalUsage() const
{
    return m_totalUsage;
}

float QCpu::getTemperature() const
{
    return m_temperature;
}

int QCpu::getUpdateInterval() const
{
    return m_updateIntervalMs;
}

void QCpu::setUpdateInterval(int interval)
{
    m_updateIntervalMs = interval;
    m_timer->setInterval(m_updateIntervalMs);
    Q_EMIT updateIntervalChanged();
}