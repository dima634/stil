import Quickshell
import QtQuick

Flyout {
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
