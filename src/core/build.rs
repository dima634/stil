
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule::<&str, &str> {
            uri: "core",
            version_major: 1,
            version_minor: 0,
            rust_files: &["src/qobjects/application.rs"],
            qml_files: &[],
            qrc_files: &[],
        })
        .build();
}
