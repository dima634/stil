import QtQuick
import QtQuick.Layouts
import QtQuick.Effects

FlexboxLayout {
    anchors.fill: parent
    alignItems: FlexboxLayout.AlignCenter
    justifyContent: FlexboxLayout.JustifyEnd
    gap: 2

    TaskbarButton {
        implicitWidth: 48
        implicitHeight: 28

        Row {
            anchors.centerIn: parent
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
