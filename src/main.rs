use libloading::Library;
use qmetaobject::{prelude::*, QObjectPinned, QVariantList};
use std::{cell::RefCell, fs, path::PathBuf};

// Интерфейс в QML: свойство pluginUrls — обычный JS-массив строк
#[derive(QObject, Default)]
struct App {
    base: qt_base_class!(trait QObject),
    // Q_PROPERTY var pluginUrls READ plugin_urls NOTIFY pluginUrlsChanged
    pluginUrls: qt_property!(QVariantList; NOTIFY pluginUrlsChanged),
    pluginUrlsChanged: qt_signal!(),
}

type PluginEntry = unsafe extern "C" fn(&mut QmlEngine);

fn main() {
    let mut engine = QmlEngine::new();
    let app = RefCell::new(App::default());

    // 1) Динамически грузим .so и даём плагинам зарегистрировать свои Rust-типы
    let mut urls = Vec::new();
    for entry in fs::read_dir("plugins").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("so") {
            unsafe {
                let lib = Library::new(&path).unwrap();
                let func: libloading::Symbol<PluginEntry> = lib.get(b"plugin_entry").unwrap();
                func(&mut engine);
                std::mem::forget(lib);
            }
            // 2) Ищем рядом QML-файл плагина
            let mut qml = path.clone();
            qml.set_file_name("ui.qml"); // или как у вас называется
            urls.push(qml.to_string_lossy().into_owned());
        }
    }

    // 1) Сначала конвертируем Vec<String> → QVariantList
    let qlist = urls
        .into_iter()
        .map(|url| {
            // 1) QString из &str
            let qstr = QString::from(url.as_str());
            // 2) Явно превращаем в QVariant
            QVariant::from(qstr)
        })
        // 3) Турбо-рычаг для FromIterator<QVariant> → QVariantList
        .collect::<QVariantList>();
    // 2) Записываем в свойство внутри RefCell<App>
    {
        let mut app_mut = app.borrow_mut();
        app_mut.pluginUrls = qlist;
        app_mut.pluginUrlsChanged();
    }

    // 2) Застолбим его в памяти и получим QObjectPinned
    let pinned: QObjectPinned<'_, App> = unsafe { QObjectPinned::new(&app) };

    // 3) Положим в QML-контекст под именем "app"
    engine.set_object_property(QString::from("app"), pinned);

    let qml_data = include_str!("../src/main.qml");
    // Загружаем _только_ данные
    engine.load_data(qml_data.into());
    engine.exec();
}
