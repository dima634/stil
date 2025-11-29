use crate::{
    db::models,
    freedesktop::{DesktopEntry, IconLookup, find_application_desktop_entries},
    repos,
};
use std::{path::PathBuf, sync::LazyLock};

pub struct Application {
    desktop_entry: DesktopEntry,
    app: Option<models::Application>,
    icon_path: Option<PathBuf>,
}

impl Application {
    #[inline]
    pub fn icon_path(&self) -> Option<&PathBuf> {
        self.icon_path.as_ref()
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.desktop_entry.name
    }

    #[inline]
    pub fn is_pinned(&self) -> bool {
        self.app.as_ref().map(|app| app.is_pinned).unwrap_or(false)
    }
}

pub struct ApplicationManager {
    applications: Vec<Application>,
}

impl ApplicationManager {
    pub fn find_by_wmclass(&self, wmclass: &str) -> Option<&Application> {
        self.applications
            .iter()
            .find(|app| app.desktop_entry.wm_class.as_deref() == Some(wmclass))
            .or_else(|| self.applications.iter().find(|app| app.desktop_entry.id == wmclass))
    }
}

impl Default for ApplicationManager {
    fn default() -> Self {
        let app_repo = repos::ApplicationRepo::default();
        let mut icon_lookup = IconLookup::default();
        let applications = find_application_desktop_entries()
            .into_iter()
            .map(|desktop_entry| build_application(desktop_entry, &app_repo, &mut icon_lookup))
            .collect();

        ApplicationManager { applications }
    }
}

pub fn application_manager() -> &'static ApplicationManager {
    &APPLICATION_MANAGER
}

static APPLICATION_MANAGER: LazyLock<ApplicationManager> = LazyLock::new(ApplicationManager::default);

fn build_application(
    desktop_entry: DesktopEntry,
    app_repo: &repos::ApplicationRepo,
    icon_lookup: &mut IconLookup,
) -> Application {
    let icon_path = desktop_entry
        .icon
        .as_ref()
        .map(|icon| icon_lookup.find_icon(icon, 48, 1))
        .flatten();
    let app = app_repo.get_by_id(&desktop_entry.id);
    Application {
        desktop_entry: desktop_entry,
        app,
        icon_path,
    }
}

#[cxx::bridge(namespace = "core::app")]
mod ffi {
    extern "Rust" {
        type Application;
        fn name(self: &Application) -> &str;
        fn is_pinned(self: &Application) -> bool;
        #[cxx_name = "icon_path"]
        fn icon_path_ffi(self: &Application) -> String;

        type ApplicationManager;
        #[cxx_name = "find_by_wmclass"]
        fn find_by_wmclass_ffi(self: &ApplicationManager, wmclass: &str) -> *const Application;
        fn application_manager() -> &'static ApplicationManager;
    }
}

impl Application {
    fn icon_path_ffi(&self) -> String {
        self.icon_path()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}

impl ApplicationManager {
    fn find_by_wmclass_ffi(&self, wmclass: &str) -> *const Application {
        if let Some(app) = self.find_by_wmclass(wmclass) {
            app as *const Application
        } else {
            std::ptr::null()
        }
    }
}
