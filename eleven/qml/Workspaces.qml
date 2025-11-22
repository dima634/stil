import QtQuick
import QtQuick.Layouts
import "." as Eleven

FlexboxLayout {
    Repeater {
        model: QWorkspaces.all
        Rectangle {
            width: 24
            height: 24
            color: QWorkspaces.current?.id === id ? Eleven.Theme.taskbarItemBackground : "transparent"
            radius: 4

            Text {
                anchors.centerIn: parent
                text: modelData.name
                font.pixelSize: 18
            }
        }
    }
}
