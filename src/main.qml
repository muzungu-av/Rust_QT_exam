import QtQuick 2.15
import QtQuick.Window 2.15
import Backend 1.0

Window {
    visible: true
    width: 800; height: 600
    title: "Canvas + Rust + QML"

    Backend { id: backend }

    Canvas {
        id: canvas
        anchors.fill: parent
        onPaint: {
            let ctx = getContext("2d")
            ctx.clearRect(0, 0, width, height)
            // пример одной «свечи»
            ctx.fillStyle = "green"
            ctx.fillRect(100, 100, 10, 50)
            ctx.beginPath()
            ctx.moveTo(105, 80)
            ctx.lineTo(105, 150)
            ctx.stroke()
        }
        MouseArea {
            anchors.fill: parent
            onClicked: (mouse) => backend.on_mouse_click(mouse.x, mouse.y)
        }
    }

    Timer {
        interval: 16; running: true; repeat: true
        onTriggered: canvas.requestPaint()
    }
}


хорошо. все работает. сейчас у меня главная программа и окно и 2 файла main.rs и main.qml
это компилируется в один файл и запускается на линукс

Что если я захочу написать еще добавочный плагин или библиотеку (также на rust и qml) которая содержит
свой rust-код и qml-файл. Потом скомпилировать их тоже и подключить , просто положив в папку lib. Нужно чтобы основная программа подхватила 
код плагина или библиотеки и смогла его вызывать. Возможно ли такое?