#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        
        // include!("cxx-qt-lib/qlist.h");
        // #[namespace = "qobjects"]
        // type QWorkspace;
        // type QList_QWorkspace = cxx_qt_lib::QList<*const QWorkspace>;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qml_uncreatable]
        #[namespace = "qobjects"]
        type QWorkspaceRegistry = super::WorkspaceRegistry;
    }
}

use cxx_qt_lib::QString;
use std::{collections::HashMap, pin::Pin};
use crate::qobjects::workspace::qobject::QWorkspace;

pub struct WorkspaceRegistry {
    // workspaces: Vec<cxx::UniquePtr<QWorkspace>>,
}

impl Default for WorkspaceRegistry {
    fn default() -> Self {
        let workspace =  QWorkspace::make_unique();

        Self {}
    }
}
