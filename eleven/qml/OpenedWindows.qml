import QtQuick
import QtQuick.Layouts

ListView {
    id: windowsList
    orientation: ListView.Horizontal
    spacing: 2

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
        color: windowDelegate.window.focused ? Theme.windowBackground : "transparent"
        border.color: windowDelegate.window.focused ? Theme.windowBorder : "transparent"

        Image {
            anchors.centerIn: parent
            width: parent.width - 8
            height: width
            source: windowDelegate.window.iconPath
        }

        Rectangle {
            anchors {
                bottom: parent.bottom
                bottomMargin: 1
            }
            anchors.horizontalCenter: parent.horizontalCenter
            height: 2
            width: windowDelegate.window.focused ? parent.width - 14 : parent.width - 20
            radius: 5
            color: windowDelegate.window.focused ? Theme.windowActive : Theme.windowRunning
            visible: windowDelegate.window.running
        }
    }
}
