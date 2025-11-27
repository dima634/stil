import QtQuick

Rectangle {
    id: host
    property string icon
    property string text
    signal clicked
    height: childrenRect.height
    width: parent.width
    implicitWidth: content.width + 3
    radius: 3
    color: mouse.hovered ? Theme.listItemBackgroundHovered : "transparent"

    Behavior on color {
        ColorAnimation {
            duration: 83
            easing.type: Easing.Linear
        }
    }

    Row {
        id: content
        spacing: 1

        Image {
            anchors.verticalCenter: parent.verticalCenter
            height: 20
            width: height
            source: host.icon
            visible: !!host.icon
        }

        Text {
            anchors.bottom: parent.bottom
            anchors.bottomMargin: 1
            text: host.text
            font.pixelSize: 12
        }
    }

    MouseArea {
        anchors.fill: parent
        onClicked: {
            host.clicked();
        }
    }

    HoverHandler {
        id: mouse
        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
    }
}
