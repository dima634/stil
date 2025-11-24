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
            duration: 150
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
        color: "transparent"

        Rectangle {
            opacity: mouse.hovered || windowDelegate.window.focused ? 1 : 0
            anchors.fill: parent
            radius: 3
            color: mouse.hovered ? Theme.windowBackgroundHovered : Theme.windowBackground
            border.color: Theme.windowBorder

            Behavior on opacity {
                NumberAnimation {
                    duration: 50
                    easing.type: Easing.InOutQuad
                }
            }
        }

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

            Behavior on width {
                NumberAnimation {
                    duration: 50
                    easing.type: Easing.InOutQuad
                }
            }
        }

        HoverHandler {
            id: mouse
            acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
        }
    }
}
