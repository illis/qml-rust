import QtQuick 2.0

Rectangle {
    id: container
    width: 400
    height: 300
    color: "red"

    Component.onCompleted: {
        timer.start()
    }

    Timer {
        id: timer
        interval: 100
        onTriggered: {
            Qt.quit()
        }
    }
}
