use crate::{freedesktop, repos};

#[derive(Debug)]
pub struct App {
    desktop_entry: freedesktop::DesktopEntry,
    is_pinned: bool,
}

impl App {
    #[inline]
    pub fn id(&self) -> &String {
        &self.desktop_entry.id
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.desktop_entry.name
    }

    #[inline]
    pub fn icon(&self) -> Option<&String> {
        self.desktop_entry.icon.as_ref()
    }

    #[inline]
    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[derive(Debug)]
pub struct ApplicationService {
    applications: Vec<App>,
}

impl ApplicationService {
    pub fn get_app_by_id(&self, app_id: &str) -> Option<&App> {
        self.applications.iter().find(|app| app.desktop_entry.id == app_id)
    }

    pub fn find_app_by_wmclass(&self, wm_class: &str) -> Option<&App> {
        self.applications
            .iter()
            .find(|app| app.desktop_entry.wm_class.as_deref() == Some(wm_class))
            .or_else(|| self.applications.iter().find(|app| app.desktop_entry.id == wm_class))
    }
}

impl Default for ApplicationService {
    fn default() -> Self {
        let pinned_apps = repos::ApplicationRepo::default().get_pinned();
        let desktop_entries = freedesktop::find_application_desktop_entries();
        let applications = desktop_entries
            .into_iter()
            .map(|desktop_entry| App {
                is_pinned: pinned_apps.iter().any(|app| app.id == desktop_entry.id),
                desktop_entry,
            })
            .collect();

        Self { applications }
    }
}
