pragma Singleton
import Quickshell
import QtQuick

Singleton {
    function rgba(r, g, b, a) {
        return Qt.rgba(r / 255, g / 255, b / 255, a);
    }

    function hex(hex, a) {
        const aHex = Math.round(a * 255).toString(16).padStart(2, '0');
        return `#${aHex}${hex}`;
    }
}
