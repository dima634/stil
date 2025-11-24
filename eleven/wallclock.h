#pragma once

#include <QtQmlIntegration/QtQmlIntegration>

class QWallclock : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("Wallclock cannot be created directly")

    Q_PROPERTY(int year READ getYear NOTIFY datetimeChanged);
    Q_PROPERTY(unsigned month READ getMonth NOTIFY datetimeChanged);
    Q_PROPERTY(unsigned day READ getDay NOTIFY datetimeChanged);
    Q_PROPERTY(qint64 hour READ getHour NOTIFY datetimeChanged);
    Q_PROPERTY(qint64 minute READ getMinute NOTIFY datetimeChanged);
    Q_PROPERTY(qint64 second READ getSecond NOTIFY datetimeChanged);
    Q_PROPERTY(int updateInterval READ getUpdateInterval WRITE setUpdateInterval NOTIFY updateIntervalChanged)

  public:
    explicit QWallclock(QObject *parent = nullptr);
    int getYear() const;
    unsigned getMonth() const;
    unsigned getDay() const;
    std::int64_t getHour() const;
    std::int64_t getMinute() const;
    std::int64_t getSecond() const;
    int getUpdateInterval() const;
    void setUpdateInterval(int intervalMs);

  signals:
    void datetimeChanged();
    void updateIntervalChanged();

  private:
    std::chrono::hh_mm_ss<std::chrono::milliseconds> m_hms;
    std::chrono::year_month_day m_ymd;
    int m_updateIntervalMs{1000 * 60};

    void updateTime();
    void scheduleNextUpdate();
};