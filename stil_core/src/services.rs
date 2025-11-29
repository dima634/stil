use crate::{application::ApplicationManager, system_events::SystemEventDispatcher};
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, LazyLock, Mutex},
};

pub struct ServiceLocator {
    system_event_dispatcher: Arc<SystemEventDispatcher>,
    system_info: Mutex<sysinfo::System>,
    components_info: Mutex<sysinfo::Components>,
    system_dbus: zbus::blocking::Connection,
    application_manager: ApplicationManager,
}

impl ServiceLocator {
    #[inline]
    pub fn system_event_dispatcher() -> impl Deref<Target = SystemEventDispatcher> {
        SERVICE_LOCATOR.system_event_dispatcher.clone()
    }

    #[inline]
    pub fn system_info() -> impl DerefMut<Target = sysinfo::System> {
        SERVICE_LOCATOR.system_info.lock().unwrap()
    }

    #[inline]
    pub fn components_info() -> impl DerefMut<Target = sysinfo::Components> {
        SERVICE_LOCATOR.components_info.lock().unwrap()
    }

    #[inline]
    pub fn system_dbus() -> impl Deref<Target = zbus::blocking::Connection> {
        &SERVICE_LOCATOR.system_dbus
    }

    #[inline]
    pub fn application_manager() -> impl Deref<Target = ApplicationManager> {
        &SERVICE_LOCATOR.application_manager
    }
}

static SERVICE_LOCATOR: LazyLock<ServiceLocator> = LazyLock::new(|| ServiceLocator {
    system_event_dispatcher: create_system_event_dispatcher(),
    system_info: Mutex::new(sysinfo::System::new()),
    components_info: Mutex::new(sysinfo::Components::new_with_refreshed_list()),
    system_dbus: zbus::blocking::Connection::system().expect("failed to connect to D-Bus system bus"),
    application_manager: ApplicationManager::default(),
});

fn create_system_event_dispatcher() -> Arc<SystemEventDispatcher> {
    let (dispatcher, rx) = SystemEventDispatcher::new();
    let dispatcher = Arc::new(dispatcher);
    let dispatcher_listener = Arc::clone(&dispatcher);

    std::thread::Builder::new()
        .name("System Events Dispatcher".to_string())
        .spawn(move || {
            dispatcher_listener.listen(rx);
        })
        .expect("failed to create thread for system event dispatch");

    dispatcher
}
