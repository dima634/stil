import Quickshell
import QtQuick

PopupWindow {
    id: root
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

        opacity: root.visible ? 1.0 : 0.0
        scale: root.visible ? 1.0 : 0.9
        transformOrigin: Item.Bottom

        Behavior on opacity {
            NumberAnimation {
                duration: 200
                easing.type: Easing.OutCubic
            }
        }

        Behavior on scale {
            NumberAnimation {
                duration: 200
                easing.type: Easing.OutBack
            }
        }
    }
}
