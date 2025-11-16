#pragma once

#include <QtQmlIntegration/QtQmlIntegration>

class QCpu : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    QML_UNCREATABLE("Clients are managed by Hyprland")

    Q_PROPERTY(QString brand READ getBrand CONSTANT)
    Q_PROPERTY(QList<float> usagePerCore READ getUsagePerCore NOTIFY usagePerCoreChanged)
    Q_PROPERTY(float totalUsage READ getTotalUsage NOTIFY totalUsageChanged);
    Q_PROPERTY(int updateInterval READ getUpdateInterval WRITE setUpdateInterval NOTIFY updateIntervalChanged)

  public:
    explicit QCpu(QObject *parent = nullptr);

    const QString &getBrand() const;
    const QList<float> &getUsagePerCore() const;
    float getTotalUsage() const;

    int getUpdateInterval() const;
    void setUpdateInterval(int interval);

  signals:
    void usagePerCoreChanged();
    void totalUsageChanged();
    void updateIntervalChanged();

  private:
    QList<float> m_usagePerCore;
    float m_totalUsage;
    QString m_brand;

    int m_updateIntervalMs = 1000;
    QTimer *m_timer;
};
