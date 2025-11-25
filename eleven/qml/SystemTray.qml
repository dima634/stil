import QtQuick
import QtQuick.Layouts

FlexboxLayout {
    anchors.fill: parent
    alignItems: FlexboxLayout.AlignCenter
    justifyContent: FlexboxLayout.JustifyEnd
    gap: 2

    Column {
        Text {
            anchors.horizontalCenter: parent.horizontalCenter
            text: `${QWallclock.hour}:${QWallclock.minute.toString().padStart(2, '0')}`
            font.pixelSize: 10
        }
        Text {
            text: `${QWallclock.day}/${QWallclock.month}/${QWallclock.year}`
            font.pixelSize: 10
        }
    }
}