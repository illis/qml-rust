#![allow(non_snake_case)]
extern crate qml;
use qml::{QQuickView, QUrl};

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
    let url = QUrl::new(&get_local_file("examples/simple.qml"));
    view.load_url(url);
    view.exec();
}

