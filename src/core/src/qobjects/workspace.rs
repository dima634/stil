#[cxx_qt::bridge]
pub mod qobject {
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[cxx_name = "make_unique"]
        fn qworkspace_make_unique() -> UniquePtr<QWorkspace>;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_uncreatable]
        #[qproperty(i32, id, READ)]
        #[qproperty(QString, name, READ)]
        #[namespace = "qobjects"]
        type QWorkspace = super::Workspace;
    }
}

use cxx_qt_lib::{QListElement, QString};

#[derive(Debug, Default)]
pub struct Workspace {
    id: i32,
    name: QString, // empty means unnamed
    running_applications: Vec<QString>,
}

impl qobject::QWorkspace {
    pub fn make_unique() -> cxx::UniquePtr<qobject::QWorkspace> {
        qobject::qworkspace_make_unique()
    }
}
