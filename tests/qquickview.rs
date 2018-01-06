extern crate qml;

#[cfg(debug_assertions)]
use qml::*;

#[cfg(debug_assertions)]
#[test]
fn test_qquickview_memory() {
    let mut view = QQuickView::new();
    let url = QUrl::new("qrc:///qml/tst_simple.qml").unwrap();

    unsafe {
        init_testresources();
    }

    view.load_url(url);
    view.exec();
}

extern "C" {
    #[cfg(debug_assertions)]
    fn init_testresources();
}
