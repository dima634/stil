import QtQuick
import "." as Eleven

Rectangle {
    anchors.fill: parent
    color: Eleven.Theme.shell

    Row {
        spacing: 4
        padding: 4

        Repeater {
            model: QWorkspaces.all
            Rectangle {
                width: 24
                height: 24
                color: QWorkspaces.current?.id === id ? "red" : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: modelData.name
                    font.pixelSize: 18
                }
            }
        }

        Repeater {
            property var filteredClients: QWindows.all.filter(c => c.workspaceName === QWorkspaces.current?.name)

            model: filteredClients
            Rectangle {
                height: 24
                width: childrenRect.width
                border.color: "black"

                Row {
                    Image {
                        anchors.verticalCenter: parent.verticalCenter
                        width: 20
                        height: 20
                        source: modelData.iconPath
                    }

                    Text {
                        anchors.verticalCenter: parent.verticalCenter
                        text: modelData.name
                        font.pixelSize: 18
                    }
                }
            }
        }

        Text {
            text: `${QCpu.totalUsage.toFixed(2)}%`
        }

        Text {
            text: `${QCpu.temperature.toFixed(0)}°C`
        }

        Text {
            text: QCpu.brand
        }

        Rectangle {
            height: parent.height - 8
            width: 10
            color: "black"
        }

        Text {
            text: `${gb(QMemory.usedRam)}G / ${gb(QMemory.totalRam)}G   aval ${gb(QMemory.availableRam)}G   free ${gb(QMemory.freeRam)}G`
        }

        Rectangle {
            height: parent.height - 8
            width: 10
            color: "black"
        }

        Text {
            text: `${QWallclock.day}/${QWallclock.month}/${QWallclock.year} ${QWallclock.hour}:${QWallclock.minute.toString().padStart(2, '0')}:${QWallclock.second.toString().padStart(2, '0')}`
        }
    }

    function gb(bytes) {
        return (bytes / (1024 * 1024 * 1024)).toFixed(2);
    }
}
