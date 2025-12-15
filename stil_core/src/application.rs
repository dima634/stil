use crate::{freedesktop, repos};

#[derive(Debug)]
pub struct App {
    desktop_entry: freedesktop::DesktopEntry,
    icon_path: Option<std::path::PathBuf>,
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
    pub fn icon_path(&self) -> Option<&std::path::PathBuf> {
        self.icon_path.as_ref()
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
        let mut icon_lookup = freedesktop::IconLookup::default();
        let applications = desktop_entries
            .into_iter()
            .map(|desktop_entry| App {
                is_pinned: pinned_apps.iter().any(|app| app.id == desktop_entry.id),
                icon_path: desktop_entry
                    .icon
                    .as_ref()
                    .map(|icon| icon_lookup.find_icon(icon, 48, 1))
                    .flatten(),
                desktop_entry,
            })
            .collect();

        Self { applications }
    }
}
