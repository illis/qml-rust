extern crate qml;
use qml::{QQuickView, QUrl};

#[link(name="testresources", kind="static")]
#[test]
fn test_qquickview_memory() {
    let mut view = QQuickView::new();
    let url = QUrl::new("qrc:///qml/tst_simple.qml");

    unsafe {
        init_testresources();
    }

    view.load_url(url);
    view.exec();
}

extern "C" {
    fn init_testresources();
}
