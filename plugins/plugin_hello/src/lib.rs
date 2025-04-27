use libc::c_char;
use qmetaobject::QmlEngine;
use std::ffi::CString;

// Встраиваем текст ui.qml прямо в библиотеку
static UI_QML: &str = include_str!("../src/ui.qml");

// extern-функция хоста принимает (name, description, ui_data)
extern "C" {
    fn register_plugin(name: *const c_char, description: *const c_char, ui_data: *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn plugin_entry(_engine: &mut QmlEngine) {
    let name = CString::new("Hello").unwrap();
    let description = CString::new("Плагин Hello").unwrap();
    let ui_data = CString::new(UI_QML).unwrap();

    // Передаём в хост текст QML, хост сохранит его в модели
    register_plugin(name.as_ptr(), description.as_ptr(), ui_data.as_ptr());
}
