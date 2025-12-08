import QtQuick

ListView {
    orientation: ListView.Horizontal
    implicitWidth: contentWidth
    height: 28
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

    delegate: TaskbarButton {
        id: template
        required property var app
        width: 28
        height: 28

        Image {
            anchors.centerIn: parent
            width: parent.width - 6
            height: width
            source: template.app.icon

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    QSystem.launchApp(template.app.appId);
                }
            }
        }
    }
}
