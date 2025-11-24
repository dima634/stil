pragma Singleton
import Quickshell
import QtQuick
import "." as Eleven

Singleton {
    readonly property color shellBackground: '#bec9eddb'
    readonly property color workspaceActive: '#4f003f92'
    readonly property color workspace: '#4fffffff'

    readonly property color taskbarButtonFocused: '#FF005FB8'
    readonly property color taskbarButtonActive: '#70353535'
    readonly property color taskbarButtonBorder: '#13000000'
    readonly property color taskbarButtonBackground: '#7FFFFFFF'
    readonly property color taskbarButtonBackgroundHovered: '#bbffffff'
}
