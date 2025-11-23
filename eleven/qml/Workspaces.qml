import QtQuick
import QtQuick.Layouts
import "." as Eleven

ListView {
    orientation: ListView.Horizontal
    spacing: 2
    model: QWorkspaces

    implicitWidth: contentWidth
    height: 26

    add: Transition {
        NumberAnimation {
            property: "scale"
            from: 0
            to: 1
            duration: 500
            easing.type: Easing.OutCubic
        }
    }

    displaced: Transition {
        NumberAnimation {
            properties: "x"
            duration: 100
            easing.type: Easing.InOutQuad
        }
        NumberAnimation {
            properties: "scale"
            to: 1
            duration: 500
            easing.type: Easing.InOutQuad
        }
    }

    delegate: Item {
        id: workspaceDelegate
        required property var modelData

        anchors.verticalCenter: parent?.verticalCenter
        width: 26
        height: 26

        Rectangle {
            width: parent.width
            height: parent.height
            radius: 3
            color: QWorkspaces.current?.name === workspaceDelegate.modelData.name ? Eleven.Theme.workspaceActive : Eleven.Theme.workspace

            Text {
                anchors.centerIn: parent
                text: workspaceDelegate.modelData.name
                font.pixelSize: 18
            }
        }
    }
}
