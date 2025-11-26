import QtQuick
import QtQuick.Controls

Control {
    id: host
    enum StatusBar {
        Focused,
        Active,
        Hidden
    }
    property int statusBar: TaskbarButton.StatusBar.Hidden
    property bool highlighted: false
    default property alias content: container.data

    background: Rectangle {
        opacity: mouse.hovered || host.highlighted ? 1 : 0
        anchors.fill: parent
        radius: 3
        color: mouse.hovered ? Theme.taskbarButtonBackgroundHovered : Theme.taskbarButtonBackground
        border.color: Theme.taskbarButtonBorder

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
        width: host.statusBar === TaskbarButton.StatusBar.Focused ? parent.width - 14 : parent.width - 20
        radius: 5
        color: host.statusBar === TaskbarButton.StatusBar.Focused ? Theme.taskbarButtonFocused : Theme.taskbarButtonActive
        visible: host.statusBar !== TaskbarButton.StatusBar.Hidden

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
