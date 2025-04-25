#[macro_use]
extern crate cstr;
use qmetaobject::prelude::*;

#[no_mangle]
pub unsafe extern "C" fn plugin_entry(_engine: &mut QmlEngine) {
    println!("plugin_world загружен");
}
