import QtQuick 2.15
import QtQuick.Controls 2.15

ApplicationWindow {
    visible: true
    width: 600; height: 400

    //Список плагинов сверху
    ListView {
        id: listview
        width: parent.width
        height: parent.height / 2
        model: app.pluginInfos

        delegate: Rectangle {
            width: listview.width
            height: 80
            color: "lightgray"
            border.color: "gray"; border.width: 1

            Column {
                anchors.fill: parent; anchors.margins: 10

                Text { text: modelData.name; font.bold: true }
                Text { text: modelData.description; font.pointSize: 10 }
                Button {
                    text: "Открыть"
                    onClicked: {
                        // создаём UI плагина в pluginColumn
                        var obj = Qt.createQmlObject(
                            modelData.ui_data,
                            pluginColumn   // <-- Column-позиционер
                        );
                        if (!obj)
                            console.error("Не удалось создать UI плагина");
                    }
                }
            }
        }
    }

    // Колонка для плагинов под ListView
    Column {
        id: pluginColumn
        width: parent.width
        // привязываем верхнюю грань к низу listview
        anchors.top: listview.bottom
        spacing: 10
    }
}
