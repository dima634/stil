import QtQuick

ListView {
    orientation: ListView.Horizontal
    spacing: 2
    model: QWorkspaces

    implicitWidth: contentWidth
    height: 28

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

    delegate: TaskbarButton {
        id: template
        required property var modelData
        highlighted: QWorkspaces.current?.id === template.modelData.id
        statusBar: QWorkspaces.current?.id === template.modelData.id ? TaskbarButton.StatusBar.Focused : TaskbarButton.StatusBar.Hidden
        width: 28
        height: 28

        Text {
            anchors.centerIn: parent
            text: template.modelData.name
            font.pixelSize: 18
        }
    }
}
