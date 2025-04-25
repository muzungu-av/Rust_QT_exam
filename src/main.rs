#[macro_use]
extern crate cstr;

use qmetaobject::prelude::*;
use std::rc::Rc;

#[derive(QObject, Default)]
#[allow(non_snake_case)]
struct Backend {
    // 3) Указываем базовый класс
    base: qt_base_class!(trait QObject),
    // 4) Объявляем метод, body прямо в макросе
    on_mouse_click: qt_method!(
        fn on_mouse_click(&self, x: f32, y: f32) {
            println!("Клик по канве: x = {}, y = {}", x, y);
        }
    ),
}

fn main() {
    // 5) Регистрируем тип в QML под именем Backend 1.0
    qml_register_type::<Backend>(cstr!("Backend"), 1, 0, cstr!("Backend"));

    // 6) Запускаем QML-движок
    let mut engine = QmlEngine::new();
    let qml_data = include_str!("../src/main.qml");
    // Загружаем _только_ данные
    engine.load_data(qml_data.into());
    engine.exec();
}
