#[macro_use]
extern crate cstr;
use qmetaobject::prelude::*;

/// Ничего не регистрируем (пустой плагин), только демонстрируем загрузку.
#[no_mangle]
pub unsafe extern "C" fn plugin_entry(_engine: &mut QmlEngine) {
    println!("plugin_hello загружен");
}
