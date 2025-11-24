import QtQuick
import QtQuick.Controls

Control {
    id: host
    enum Mode { Focused, Active, Default }
    property int mode: TaskbarButton.Mode.Default
    default property alias content: container.data

    anchors.verticalCenter: parent?.verticalCenter
    width: 28
    height: 28

    background: Rectangle {
        opacity: mouse.hovered || mode === TaskbarButton.Mode.Focused ? 1 : 0
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

    contentItem: Item {
        id: container
        anchors.fill: parent
    }

    Rectangle {
        anchors {
            bottom: parent.bottom
            bottomMargin: 1
        }
        anchors.horizontalCenter: parent.horizontalCenter
        height: 2
        width: mode === TaskbarButton.Mode.Focused ? parent.width - 14 : parent.width - 20
        radius: 5
        color: mode === TaskbarButton.Mode.Focused ? Theme.windowActive : Theme.windowRunning
        visible: mode !== TaskbarButton.Mode.Default

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
