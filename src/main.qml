import QtQuick 2.15
import QtQuick.Controls 2.15

ApplicationWindow {
    visible: true; width: 400; height: 300

    ListView {
        anchors.fill: parent
        model: app.pluginUrls   // QVariantList автоматически «превращается» в JS-массив
        delegate: Text { text: modelData }
    }
}