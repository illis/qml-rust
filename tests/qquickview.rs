extern crate qml;
use qml::QQuickView;

#[link(name="testresources", kind="static")]
#[test]
fn test_qquickview_memory() {
    let mut view = QQuickView::new();

    unsafe {
        init_testresources();
    }

    view.load_url("qrc:///qml/autoclose.qml");
    view.exec();
}

extern "C" {
    fn init_testresources();
}