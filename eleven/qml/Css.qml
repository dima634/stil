pragma Singleton
import Quickshell
import QtQuick

Singleton {
    function rgba(r, g, b, a) {
        return Qt.rgba(r / 255, g / 255, b / 255, a);
    }
}
