#pragma once

#include <QtQmlIntegration/QtQmlIntegration>

class QMemory : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("QMemory cannot be created in QML")

    Q_PROPERTY(std::uint64_t totalRam READ totalRam NOTIFY usageChanged);
    Q_PROPERTY(std::uint64_t usedRam READ usedRam NOTIFY usageChanged);
    Q_PROPERTY(std::uint64_t freeRam READ freeRam NOTIFY usageChanged);
    Q_PROPERTY(std::uint64_t availableRam READ availableRam NOTIFY usageChanged);
    Q_PROPERTY(std::uint64_t totalSwap READ totalSwap NOTIFY usageChanged);
    Q_PROPERTY(std::uint64_t usedSwap READ usedSwap NOTIFY usageChanged);
    Q_PROPERTY(std::uint64_t freeSwap READ freeSwap NOTIFY usageChanged);
    Q_PROPERTY(int updateInterval READ getUpdateInterval WRITE setUpdateInterval NOTIFY updateIntervalChanged);

  public:
    explicit QMemory(QObject *parent = nullptr);

    std::uint64_t totalRam() const;
    std::uint64_t usedRam() const;
    std::uint64_t freeRam() const;
    std::uint64_t availableRam() const;
    std::uint64_t totalSwap() const;
    std::uint64_t usedSwap() const;
    std::uint64_t freeSwap() const;

    int getUpdateInterval() const;
    void setUpdateInterval(int interval);

  signals:
    void usageChanged();
    void updateIntervalChanged();

  private:
    std::uint64_t m_totalRam{0};
    std::uint64_t m_usedRam{0};
    std::uint64_t m_freeRam{0};
    std::uint64_t m_availableRam{0};
    std::uint64_t m_totalSwap{0};
    std::uint64_t m_usedSwap{0};
    std::uint64_t m_freeSwap{0};

    int m_updateIntervalMs{1000};
    QTimer *m_timer;
};
