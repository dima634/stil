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

    delegate: TaskbarButton {
        id: template
        required property var window
        mode: window.focused ? TaskbarButton.Mode.Focused : (window.running ? TaskbarButton.Mode.Active : TaskbarButton.Mode.Default)

        Image {
            anchors.centerIn: parent
            width: parent.width - 6
            height: width
            source: template.window.iconPath
        }
    }
}
