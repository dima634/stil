import QtQuick
import QtQuick.Layouts

ListView {
    id: windowsList
    anchors.verticalCenter: parent.verticalCenter
    orientation: ListView.Horizontal
    spacing: 2
    model: QWindows

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

    implicitWidth: contentWidth
    height: 24

    delegate: Rectangle {
        id: windowDelegate
        required property var window

        visible: window.workspaceName === QWorkspaces.current?.name
        width: visible ? childrenRect.width : 0
        height: 24
        border.color: "black"

        Row {
            Image {
                anchors.verticalCenter: parent.verticalCenter
                width: 20
                height: 20
                source: windowDelegate.window.iconPath
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: windowDelegate.window.name
                font.pixelSize: 18
            }
        }
    }
}
