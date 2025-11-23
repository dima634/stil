import QtQuick
import QtQuick.Layouts

ListView {
    id: windowsList
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
    height: 32

    delegate: Rectangle {
        id: windowDelegate
        required property var window

        anchors.verticalCenter: parent?.verticalCenter

        width: 28
        height: 28

        radius: 3
        color: Theme.windowBackground
        border.color: Theme.windowBorder

        Image {
            anchors.centerIn: parent
            width: parent.width - 4
            height: width
            source: windowDelegate.window.iconPath
        }

        Rectangle {
            anchors {
                bottom: parent.bottom
            }
            anchors.horizontalCenter: parent.horizontalCenter
            height: 3
            width: parent.width - 12
            radius: 5
            color: Theme.windowActive
        }
    }
}
