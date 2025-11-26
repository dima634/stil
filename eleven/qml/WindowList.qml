import QtQuick

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
        highlighted: window.focused
        statusBar: window.focused ? TaskbarButton.StatusBar.Focused : (window.running ? TaskbarButton.StatusBar.Active : TaskbarButton.StatusBar.Hidden)
        width: 28
        height: 28

        Image {
            anchors.centerIn: parent
            width: parent.width - 6
            height: width
            source: template.window.iconPath
        }
    }
}
