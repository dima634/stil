#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qml_uncreatable]
        #[namespace = "qobjects"]
        type QApplicationRegistry = super::ApplicationRegistry;

        #[qinvokable]
        fn get_app_icon(self: Pin<&mut QApplicationRegistry>, app_id: &QString) -> QString;

        #[qinvokable]
        fn get_app_name(self: Pin<&mut QApplicationRegistry>, app_id: &QString) -> QString;
    }
}

use crate::freedesktop::{DesktopEntry, find_application_desktop_entries};
use cxx_qt_lib::QString;
use std::{collections::HashMap, pin::Pin};

pub struct ApplicationRegistry {
    applications: HashMap<String, DesktopEntry>,
}

impl Default for ApplicationRegistry {
    fn default() -> Self {
        let app_desktop_entries = find_application_desktop_entries();
        let applications = app_desktop_entries
            .into_iter()
            .map(|entry| (entry.id.clone(), entry))
            .collect();

        Self { applications }
    }
}

impl qobject::QApplicationRegistry {
    pub fn get_app_icon(self: Pin<&mut Self>, app_id: &QString) -> QString {
        let icon = self
            .applications
            .get(&app_id.to_string())
            .map(|app| app.icon.as_ref())
            .flatten();

        if let Some(icon_path) = icon {
            QString::from(icon_path.to_str().expect("should be valid unicode"))
        } else {
            QString::default()
        }
    }

    pub fn get_app_name(self: Pin<&mut Self>, app_id: &QString) -> QString {
        self.applications
            .get(&app_id.to_string())
            .map(|app| app.name.clone().into())
            .unwrap_or_default()
    }
}
