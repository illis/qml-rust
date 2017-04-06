#![allow(non_snake_case)]
extern crate qml;

use qml::QQuickView;

pub fn get_local_file(path: &str) -> String {
    let path_raw = ::std::env::current_dir().unwrap().join(path);
    if cfg!(windows) {
        format!("file:///{}", path_raw.display())
    } else {
        format!("file://{}", path_raw.display())
    }
}

fn main() {
    let mut view = QQuickView::new();

    view.load_url(&get_local_file("examples/simple.qml"));
    view.exec();
}

