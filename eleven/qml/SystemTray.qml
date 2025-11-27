import Quickshell
import QtQuick
import QtQuick.Layouts
import QtQuick.Effects

FlexboxLayout {
    anchors.fill: parent
    alignItems: FlexboxLayout.AlignCenter
    justifyContent: FlexboxLayout.JustifyEnd
    gap: 2

    // Power controls
    TaskbarButton {
        id: powerBtn
        implicitHeight: 28
        implicitWidth: 28
        highlighted: powerMenu.visible

        Image {
            anchors.centerIn: parent
            height: parent.height - 8
            width: height
            source: `file:/${QIconLookup.find("system-shutdown-symbolic", 24)}`

            layer.enabled: true
            layer.effect: MultiEffect {
                colorization: 1.0
                brightness: 1.0
                colorizationColor: "black"
            }

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    powerMenu.visible = !powerMenu.visible;
                }
            }
        }

        PowerControlMenu {
            id: powerMenu
            anchor.item: powerBtn
            anchor.rect.y: powerBtn.y - height - 4
            anchor.rect.x: (powerBtn.width - width) / 2
            visible: false
        }
    }

    // CPU Usage Indicator
    TaskbarButton {
        implicitWidth: 44
        implicitHeight: 28

        Row {
            height: parent.height

            Image {
                anchors.verticalCenter: parent.verticalCenter
                height: parent.height - 4
                width: height
                source: `file:/${QIconLookup.find("cpu-symbolic", 24)}`

                layer.enabled: true
                layer.effect: MultiEffect {
                    colorization: 1.0
                    brightness: 1.0
                    colorizationColor: {
                        const usage = QCpu.totalUsage;
                        if (usage < 25) {
                            return '#5fb554';
                        } else if (usage < 75) {
                            return "#FCE100";
                        } else {
                            return '#9f3936';
                        }
                    }

                    Behavior on colorizationColor {
                        ColorAnimation {
                            duration: 500
                            easing.type: Easing.InOutQuad
                        }
                    }
                }
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: `${Math.round(QCpu.totalUsage)}%`
                font.pixelSize: 10
            }
        }
    }

    // Volume Indicator
    TaskbarButton {
        implicitWidth: volumeIcon.implicitWidth
        implicitHeight: 28

        Image {
            id: volumeIcon
            anchors.centerIn: parent
            height: parent.height - 8
            width: height
            source: `file:/${QIconLookup.find("volume-level-high", 24)}`

            layer.enabled: true
            layer.effect: MultiEffect {
                colorization: 1.0
                brightness: 1.0
                colorizationColor: "black"
            }
        }
    }

    // Clock
    TaskbarButton {
        implicitWidth: clock.implicitWidth + 10
        implicitHeight: 28

        Column {
            id: clock
            anchors.centerIn: parent

            Text {
                anchors.horizontalCenter: parent.horizontalCenter
                text: `${QWallclock.hour}:${QWallclock.minute.toString().padStart(2, '0')}`
                font.pixelSize: 10
            }

            Text {
                text: `${QWallclock.day}/${QWallclock.month}/${QWallclock.year}`
                font.pixelSize: 10
            }
        }
    }
}
