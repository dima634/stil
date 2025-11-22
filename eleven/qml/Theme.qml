pragma Singleton
import Quickshell
import QtQuick
import "." as Eleven

Singleton {
    readonly property color shellBackground: '#7cbfe1ea'
    readonly property color taskbarItemBackground: '#4f003f92'
    readonly property color taskbarItemFill: Eleven.Css.rgba(255, 255, 255, 0.5)
    readonly property color stroke: Eleven.Css.rgba(0, 0, 0, 0.09)
}
