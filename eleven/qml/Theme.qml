pragma Singleton
import Quickshell
import QtQuick
import "." as Eleven

Singleton {
    readonly property color shell: Eleven.Css.rgba(191, 225, 234, 0.9)
    readonly property color taskbarItemFill: Eleven.Css.rgba(255, 255, 255, 0.5)
    readonly property color stroke: Eleven.Css.rgba(0, 0, 0, 0.09)
}
