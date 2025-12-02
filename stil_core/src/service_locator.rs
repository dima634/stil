use crate::*;
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, LazyLock, Mutex, RwLock},
};

pub struct ServiceLocator {
    system_event_dispatcher: Arc<system_events::SystemEventDispatcher>,
    system_info: Mutex<sysinfo::System>,
    components_info: Mutex<sysinfo::Components>,
    system_dbus: LazyLock<zbus::blocking::Connection>,
    db_conn_pool: db::DbConnPool,
    desktop: RwLock<desktop::Desktop>,
}

impl ServiceLocator {
    #[inline]
    pub fn system_event_dispatcher() -> impl Deref<Target = system_events::SystemEventDispatcher> {
        SERVICE_LOCATOR.system_event_dispatcher.clone()
    }

    #[inline]
    pub fn system_info() -> impl DerefMut<Target = sysinfo::System> {
        SERVICE_LOCATOR.system_info.lock().expect("poisoned")
    }

    #[inline]
    pub fn components_info() -> impl DerefMut<Target = sysinfo::Components> {
        SERVICE_LOCATOR.components_info.lock().expect("poisoned")
    }

    #[inline]
    pub fn system_dbus() -> impl Deref<Target = zbus::blocking::Connection> {
        &*SERVICE_LOCATOR.system_dbus
    }

    #[inline]
    pub fn db_conn() -> impl DerefMut<Target = rusqlite::Connection> {
        SERVICE_LOCATOR.db_conn_pool.get_conn()
    }

    #[inline]
    pub fn desktop() -> impl Deref<Target = desktop::Desktop> {
        SERVICE_LOCATOR.desktop.read().expect("poisoned")
    }

    #[inline]
    pub fn desktop_mut() -> impl DerefMut<Target = desktop::Desktop> {
        SERVICE_LOCATOR.desktop.write().expect("poisoned")
    }
}

static SERVICE_LOCATOR: LazyLock<ServiceLocator> = LazyLock::new(|| ServiceLocator {
    system_event_dispatcher: create_system_event_dispatcher(),
    system_info: Mutex::new(sysinfo::System::new()),
    components_info: Mutex::new(sysinfo::Components::new_with_refreshed_list()),
    system_dbus: LazyLock::new(connect_to_system_dbus as fn() -> zbus::blocking::Connection),
    db_conn_pool: db::DbConnPool::with_size(3),
    desktop: RwLock::new(desktop::Desktop::default()),
});

fn connect_to_system_dbus() -> zbus::blocking::Connection {
    zbus::blocking::Connection::system().expect("failed to connect to D-Bus system bus")
}

fn create_system_event_dispatcher() -> Arc<system_events::SystemEventDispatcher> {
    let (dispatcher, rx) = system_events::SystemEventDispatcher::new();
    let dispatcher = Arc::new(dispatcher);
    let dispatcher_listener = Arc::clone(&dispatcher);

    std::thread::Builder::new()
        .name("System Events Dispatcher".to_string())
        .spawn(move || dispatcher_listener.listen(rx))
        .expect("failed to create thread for system event dispatch");

    dispatcher
}
