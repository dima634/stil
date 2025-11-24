import QtQuick
import QtQuick.Layouts
import "." as Eleven

ListView {
    orientation: ListView.Horizontal
    spacing: 2
    model: QWorkspaces

    implicitWidth: contentWidth
    height: 26

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

        mode: QWorkspaces.current?.id === template.modelData.id ? TaskbarButton.Mode.Focused : TaskbarButton.Mode.Default

        Text {
            anchors.centerIn: parent
            text: template.modelData.name
            font.pixelSize: 18
        }
    }
}
