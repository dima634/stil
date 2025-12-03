import QtQuick
import QtQuick.Layouts
import Quickshell

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
        color: Theme.shellBackground
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

                Row {
                    x: (parent.width - width) / 2
                    spacing: 2

                    PinnedAppsList {
                        anchors.verticalCenter: parent.verticalCenter
                        model: QPinnedApps
                    }

                    WindowList {
                        anchors.verticalCenter: parent.verticalCenter
                        model: QWorkspaces.current ? QWorkspaces.current.windows : null
                    }
                }
            }

            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
                implicitWidth: 1

                SystemTray {}
            }
        }
    }
}
