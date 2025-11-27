import Quickshell
import QtQuick

PopupWindow {
    implicitHeight: flyout.height
    implicitWidth: flyout.width
    color: "transparent"

    Flyout {
        id: flyout
        width: childrenRect.width + 8
        height: childrenRect.height + 8

        color: Theme.flyoutBackground

        Column {
            anchors.centerIn: parent
            width: Math.max(rect1.implicitWidth, rect2.implicitWidth)
            height: childrenRect.height
            spacing: 2

            ListItem {
                id: rect1
                icon: `file:/${QIconLookup.find("system-reboot-symbolic", 24)}`
                text: "Restart"
                onClicked: {
                    QSystem.reboot();
                }
            }

            ListItem {
                id: rect2
                icon: `file:/${QIconLookup.find("system-shutdown-symbolic", 24)}`
                text: "Shut Down"
                onClicked: {
                    QSystem.poweroff();
                }
            }
        }
    }
}
