import QtQuick 2.0
import testing 1.0

Rectangle {
    id: container
    width: 400
    height: 300
    color: "red"

    TestQObject {
        Component.onCompleted: {
            timer.start()
        }
    }

    Timer {
        id: timer
        interval: 100
        onTriggered: {
            Qt.quit()
        }
    }
}
