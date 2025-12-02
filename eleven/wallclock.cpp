#include "wallclock.h"

QWallclock::QWallclock(QObject *parent) : QObject(parent), m_ymd{}
{
    updateTime();
    scheduleNextUpdate();
}

int QWallclock::getYear() const
{
    return int(m_ymd.year());
}

unsigned QWallclock::getMonth() const
{
    return unsigned(m_ymd.month());
}

unsigned QWallclock::getDay() const
{
    return unsigned(m_ymd.day());
}

std::int64_t QWallclock::getHour() const
{
    return m_hms.hours().count();
}

std::int64_t QWallclock::getMinute() const
{
    return m_hms.minutes().count();
}

std::int64_t QWallclock::getSecond() const
{
    return m_hms.seconds().count();
}

int QWallclock::getUpdateInterval() const
{
    return m_updateIntervalMs;
}

void QWallclock::setUpdateInterval(int intervalMs)
{
    m_updateIntervalMs = intervalMs;
    Q_EMIT updateIntervalChanged();
}

void QWallclock::updateTime()
{
    using namespace std::chrono;

    auto now = zoned_time{current_zone(), system_clock::now()}.get_local_time();
    auto nowDays = floor<days>(now);
    m_ymd = year_month_day{nowDays};
    m_hms = hh_mm_ss{floor<milliseconds>(now - nowDays)};
    Q_EMIT datetimeChanged();
}

void QWallclock::scheduleNextUpdate()
{
    using namespace std::chrono;

    auto now = system_clock::now();
    auto nowMs = duration_cast<milliseconds>(now.time_since_epoch());

    // Calculate next boundary aligned to update interval
    auto intervalMs = milliseconds(m_updateIntervalMs);
    auto nextBoundary = (nowMs / intervalMs + 1) * intervalMs;
    auto delay = nextBoundary - nowMs;

    // Schedule next update with precise timer
    QTimer::singleShot(delay.count(), Qt::PreciseTimer, this, [this]() {
        updateTime();
        scheduleNextUpdate();
    });
}
