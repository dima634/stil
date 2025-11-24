import QtQuick
import QtQuick.Layouts
import Quickshell
import "." as Eleven

PanelWindow {
    anchors {
        left: true
        right: true
        bottom: true
    }

    margins {
        bottom: 2
        left: 2
        right: 2
    }

    implicitHeight: 32
    color: 'transparent'

    Rectangle {
        anchors.fill: parent
        color: Eleven.Theme.shellBackground
        radius: 4

        RowLayout {
            anchors.fill: parent
            anchors.margins: 3
            spacing: 4

            // Workspaces
            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
                implicitWidth: 1

                WorkspaceList {
                    anchors.verticalCenter: parent.verticalCenter
                }
            }

            // Windows opened in current workspace
            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
                implicitWidth: 2

                WindowList {
                    x: (parent.width - width) / 2
                    anchors.verticalCenter: parent.verticalCenter
                    model: QWorkspaces.current ? QWorkspaces.current.windows : null

                    Behavior on x {
                        NumberAnimation {
                            duration: 150
                            easing.type: Easing.OutCubic
                        }
                    }
                }
            }

            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
                implicitWidth: 1

                FlexboxLayout {
                    anchors.verticalCenter: parent.verticalCenter
                    gap: 4

                    Text {
                        text: `${QCpu.totalUsage.toFixed(2)}%`
                    }

                    Text {
                        text: `${QCpu.temperature.toFixed(0)}°C`
                    }

                    // Text {
                    //     text: QCpu.brand
                    // }

                    Text {
                        text: `${gb(QMemory.usedRam)}G / ${gb(QMemory.totalRam)}G   aval ${gb(QMemory.availableRam)}G   free ${gb(QMemory.freeRam)}G`
                    }

                    Text {
                        text: `${QWallclock.day}/${QWallclock.month}/${QWallclock.year} ${QWallclock.hour}:${QWallclock.minute.toString().padStart(2, '0')}:${QWallclock.second.toString().padStart(2, '0')}`
                    }
                }
            }
        }
    }

    function gb(bytes) {
        return (bytes / (1024 * 1024 * 1024)).toFixed(2);
    }
}
