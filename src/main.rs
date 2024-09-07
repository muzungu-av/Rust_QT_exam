use qmetaobject::{
    qt_base_class, qt_method, qt_signal, QObject, QObjectPinned, QString, QmlEngine,
};
use std::cell::RefCell;

#[derive(Default, QObject)]
struct SimpleWindow {
    base: qt_base_class!(trait QObject),
    // Определяем сигнал для пункта меню "Выход"
    exit_triggered: qt_signal!(),
    // Определяем слот для обработки сигнала
    exit_slot: qt_method!(fn(&self)),
}

impl SimpleWindow {
    // Обработчик выхода, привязанный к слоту
    fn exit_slot(&self) {
        println!("Приложение завершено.");
        std::process::exit(0);
    }
}

fn main() {
    // Создаем объект SimpleWindow и оборачиваем его в RefCell
    let window = RefCell::new(SimpleWindow::default());

    // Закрепляем объект SimpleWindow для использования в Qt
    let pinned_window = unsafe { QObjectPinned::new(&window) };

    // Создаем QML Engine
    let mut engine = QmlEngine::new();

    // Устанавливаем объект окна в контекст QML, используя QString для имени свойства
    engine.set_object_property(QString::from("window"), pinned_window);

    // Загружаем QML-код
    engine.load_data(
        r#"
        import QtQuick 2.15
        import QtQuick.Controls 2.15

        ApplicationWindow {
            visible: true
            width: 600
            height: 400
            title: "Rust Qt App with Menu"

            // Создание верхнего меню
            menuBar: MenuBar {
                Menu {
                    title: "Файл"
                    Action {
                        text: "Новый"
                        onTriggered: console.log("Создать новый файл")
                    }
                    Action {
                        text: "Открыть"
                        onTriggered: console.log("Открыть файл")
                    }
                    Action {
                        text: "Выход"
                        onTriggered: {
                            console.log("Приложение завершает работу")
                            // Вызов метода выхода в Rust через сигнал
                            window.exit_slot()
                        }
                    }
                }
                Menu {
                    title: "Помощь"
                    Action {
                        text: "О программе"
                        onTriggered: console.log("Показать информацию о программе")
                    }
                }
            }

            // Основное содержимое окна
            Text {
                text: "Приложение на Rust с верхним меню"
                anchors.centerIn: parent
                font.pointSize: 16
            }
        }
        "#
        .into(),
    );

    // Запуск Qt-приложения
    engine.exec();
}
