import QtQuick 2.0
import test.submodule 1.0
import debug 1.0

Rectangle {
    id: container
    width: 400
    height: 300
    color: "blue"
    property int value

    QTestListModel {
        id: testObject
        onValueChanged: {
            console.log("[QML] signal onValueChanged()")
            container.value = value
        }
    }

    Component.onCompleted: {
        console.log("[QML] invoking set_value(42)")
        testObject.set_value(42)

        console.log("[QML] invoking get_value()")
        if (testObject.get_value() !== 42) {
            return
        }

        if (container.value !== 42) {
            return
        }

        if (testObject.value !== 42) {
            return
        }

        if (testObject.value2 !== 42) {
            return
        }

        if (testObject.value3 !== 42) {
            return
        }

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
