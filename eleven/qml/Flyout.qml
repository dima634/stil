import Quickshell
import QtQuick

PopupWindow {
    default property alias contents: flyout.data
    implicitHeight: flyout.height
    implicitWidth: flyout.width
    color: "transparent"

    Rectangle {
        id: flyout
        width: childrenRect.width + 8
        height: childrenRect.height + 8
        color: Theme.flyoutBackground
        radius: 7
        border.width: 1
        border.color: Theme.flyoutBorder
    }
}
