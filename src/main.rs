use libloading::Library;
use qmetaobject::{prelude::*, QObjectPinned, QVariantList};
use std::{cell::RefCell, fs, path::PathBuf};

// Интерфейс в QML-свойство: список путей к UI-плагинам
#[derive(QObject, Default)]
struct App {
    base: qt_base_class!(trait QObject),
    // Q_PROPERTY var pluginUrls READ plugin_urls NOTIFY pluginUrlsChanged
    pluginUrls: qt_property!(QVariantList; NOTIFY pluginUrlsChanged),
    pluginUrlsChanged: qt_signal!(),
}

// Подпись экспортируемой функции в плагине
type PluginEntry = unsafe extern "C" fn(&mut QmlEngine);

fn main() {
    let mut engine = QmlEngine::new();
    let app = RefCell::new(App::default());

    // Динамически грузим .so и Собираем список QML-файлов для подпапок plugins/
    let mut urls = Vec::new();
    for entry in fs::read_dir("plugins").unwrap() {
        // let path = entry.unwrap().path();
        // if path.extension().and_then(|e| e.to_str()) == Some("so") {
        //     unsafe {
        //         let lib = Library::new(&path).unwrap();
        //         let func: libloading::Symbol<PluginEntry> = lib.get(b"plugin_entry").unwrap();
        //         func(&mut engine);
        //         std::mem::forget(lib);
        //     }
        //     // 2) Ищем рядом QML-файл плагина
        //     let mut qml = path.clone();
        //     qml.set_file_name("ui.qml"); // или как у вас называется
        //     urls.push(qml.to_string_lossy().into_owned());
        // }
        let dir = entry.unwrap().path();
        if dir.is_dir() {
            // 1) ищем .so Файл
            let so = dir.join(format!(
                "lib{}.so",
                dir.file_name().unwrap().to_string_lossy()
            ));
            if so.exists() {
                // загружаем плагин
                unsafe {
                    let lib = Library::new(&so).unwrap();
                    let func: libloading::Symbol<PluginEntry> = lib.get(b"plugin_entry").unwrap();
                    func(&mut engine);
                    std::mem::forget(lib);
                }
                // 2) собираем путь к ui.qml в этой папке
                let ui = dir.join("ui.qml");
                if ui.exists() {
                    urls.push(ui.to_string_lossy().into_owned());
                }
            }
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

    // Пробрасываем в QML
    let pinned: QObjectPinned<'_, App> = unsafe { QObjectPinned::new(&app) };
    engine.set_object_property(QString::from("app"), pinned);

    // Загружаем главное QML
    let qml_data = include_str!("../src/main.qml");
    engine.load_data(qml_data.into());
    engine.exec();
}
