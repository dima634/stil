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

            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true

                implicitWidth: 1

                Workspaces {
                    anchors.verticalCenter: parent.verticalCenter
                }
            }

            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
                implicitWidth: 3

                FlexboxLayout {
                    anchors.verticalCenter: parent.verticalCenter
                    gap: 4

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
