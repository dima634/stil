#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_uncreatable]
        #[qproperty(QString, address, READ)]
        #[qproperty(QString, app_id, READ)]
        #[namespace = "qobjects"]
        type QWindow = super::Window;
    }
}

use cxx_qt_lib::QString;

#[derive(Debug, Default)]
pub struct Window {
    app_id: QString,
    address: QString,
}
