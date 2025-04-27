use libc::c_char;
use libloading::Library;
use once_cell::sync::Lazy;
use qmetaobject::{
    prelude::*, QMetaType, QObjectPinned, QString, QVariantList, QVariantMap, QmlEngine,
};
use std::{
    cell::RefCell,
    ffi::CStr,
    fs,
    sync::{Arc, Mutex},
};

// --- Описание плагина ---
#[derive(Clone)]
struct PluginInfo {
    name: String,
    description: String,
    ui_data: String,
}

static PLUGIN_INFOS: Lazy<Arc<Mutex<Vec<PluginInfo>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

// --- Сигнатура для регистрации из плагинов ---
#[no_mangle]
pub extern "C" fn register_plugin(
    name: *const c_char,
    description: *const c_char,
    ui_data: *const c_char,
) {
    // Распаковываем C-строки
    let name = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let description = unsafe { CStr::from_ptr(description).to_string_lossy().into_owned() };
    let ui_data = unsafe { CStr::from_ptr(ui_data).to_string_lossy().into_owned() };

    println!("register_plugin: name={}, desc={}", name, description);

    // Сохраняем в общий вектор
    PLUGIN_INFOS.lock().unwrap().push(PluginInfo {
        name,
        description,
        ui_data,
    });
}

type PluginEntry = unsafe extern "C" fn(&mut QmlEngine);

#[derive(QObject, Default)]
struct App {
    base: qt_base_class!(trait QObject),
    pluginInfos: qt_property!(QVariantList; NOTIFY pluginInfosChanged),
    pluginInfosChanged: qt_signal!(),
}

fn main() {
    let mut engine = QmlEngine::new();
    let app = RefCell::new(App::default());

    // Привязываем App к QML
    let pinned: QObjectPinned<'_, App> = unsafe { QObjectPinned::new(&app) };
    engine.set_object_property(QString::from("app"), pinned);

    // Загружаем главный QML
    let qml_data = include_str!("../src/main.qml");
    engine.load_data(qml_data.into());

    // Подгружаем плагины: все файлы .so из папки plugins/
    let plugins_dir = std::path::Path::new("plugins");
    if !plugins_dir.exists() {
        eprintln!("Папка plugins/ не найдена, работаем без плагинов");
    } else {
        for entry in fs::read_dir(plugins_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("so") {
                println!("Загружаем плагин: {}", path.display());
                unsafe {
                    let lib = Library::new(&path).unwrap();
                    let func: libloading::Symbol<PluginEntry> = lib.get(b"plugin_entry").unwrap();
                    func(&mut engine);
                    std::mem::forget(lib);
                }
            }
        }
    }

    // Формируем модель и шлём сигнал
    let infos = PLUGIN_INFOS.lock().unwrap();
    println!("Всего плагинов: {}", infos.len());
    let mut qlist: QVariantList = QVariantList::default();
    for info in infos.iter() {
        let mut map = QVariantMap::default();
        map.insert(
            "name".into(),
            QVariant::from(QString::from(info.name.as_str())),
        );
        map.insert(
            "description".into(),
            QVariant::from(QString::from(info.description.as_str())),
        );
        map.insert(
            "ui_data".into(),
            QVariant::from(QString::from(info.ui_data.as_str())),
        );
        qlist.push(map.to_qvariant());
    }
    {
        let mut app_mut = app.borrow_mut();
        app_mut.pluginInfos = qlist;
        app_mut.pluginInfosChanged();
    }

    engine.exec();
}
