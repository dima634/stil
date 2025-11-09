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
        #[qproperty(QString, id)]
        #[qproperty(QString, icon)]
        #[namespace = "qobjects"]
        type QApplication = super::Application;
    }
}

use cxx_qt_lib::QString;

#[derive(Default)]
pub struct Application {
    id: QString,
    icon: QString,
}
