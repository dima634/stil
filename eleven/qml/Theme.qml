pragma Singleton
import Quickshell
import QtQuick
import "." as Eleven

Singleton {
    readonly property color shellBackground: Css.hex("f3f3f3", 0.9)
    readonly property color workspaceActive: '#4f003f92'
    readonly property color workspace: '#4fffffff'

    readonly property color taskbarButtonFocused: '#FF005FB8'
    readonly property color taskbarButtonActive: '#70353535'
    readonly property color taskbarButtonBorder: Css.hex("000000", 0.16)
    readonly property color taskbarButtonBackground: Css.hex("ffffff", 0.5)
    readonly property color taskbarButtonBackgroundHovered: '#bbffffff'

    readonly property color flyoutBackground: Css.hex("fcfcfc", 0.95)
    readonly property color flyoutBorder: Css.hex("000000", 0.06)
}
