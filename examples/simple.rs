#![allow(non_snake_case)]
extern crate qml;

use qml::*;

pub fn get_local_file(path: &str) -> String {
    let path_raw = ::std::env::current_dir().unwrap().join(path);
    if cfg!(windows) {
        format!("file:///{}", path_raw.display())
    } else {
        format!("file://{}", path_raw.display())
    }
}

struct Test {

}

/*
q_object!(pub Test as QTest {
    signals:
    slots:
    properties:
});
*/

fn main() {
    // let mut test = QTest::new();
    /*let mut view = QQuickView::new();
    view.load_url(&get_local_file("examples/simple.qml"));
    view.exec();*/
}

