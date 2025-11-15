use crate::{
    hyprland::{Event, HyprEvents},
    system_events::{SystemEvent, SystemEventDispatcher, WindowOpened, WorkspaceCreated},
};
use std::sync::{Arc, LazyLock, Once};
use tracing::{error, warn};

static INIT: Once = Once::new();
pub fn global_init() {
    INIT.call_once(|| {
        setup_logging();
        listen_for_hyprland_events();
    });
}

static SYSTEM_EVENT_DISPATCHER: LazyLock<Arc<SystemEventDispatcher>> = LazyLock::new(|| {
    let (dispatcher, rx) = SystemEventDispatcher::new();
    let dispatcher = Arc::new(dispatcher);
    let dispatcher_listener = Arc::clone(&dispatcher);

    std::thread::Builder::new()
        .name("System Events Dispatcher".to_string())
        .spawn(move || {
            dispatcher_listener.listen(rx);
        })
        .expect("should create thread for system event dispatch");

    dispatcher
});

#[inline]
pub fn system_event_dispatcher() -> Arc<SystemEventDispatcher> {
    SYSTEM_EVENT_DISPATCHER.clone()
}

fn listen_for_hyprland_events() {
    let res = std::thread::Builder::new()
        .name("Hyprland Events Listener".to_string())
        .spawn(|| {
            let systems_events = system_event_dispatcher();
            let tx = systems_events.tx();

            HyprEvents::listen(|event| {
                let system_event = match event {
                    Event::OpenWindow(open_window) => SystemEvent::WindowOpened(WindowOpened {
                        address: open_window.window_address,
                        workspace: -1, // TODO: get workspace id
                        class: open_window.window_class,
                    }),
                    Event::CloseWindow(close_window) => SystemEvent::WindowClosed(close_window.window_address),
                    Event::ActiveWindow(active_window) => SystemEvent::WindowFocused(active_window.address),
                    Event::CreateWorkspace(workspace_v2) => SystemEvent::WorkspaceCreated(WorkspaceCreated {
                        id: workspace_v2.id,
                        name: workspace_v2.name,
                    }),
                    Event::DestroyWorkspace(workspace_v2) => SystemEvent::WorkspaceDestroyed(workspace_v2.id),
                    Event::FocusWorkspace(workspace_v2) => SystemEvent::WorkspaceFocused(workspace_v2.id),
                };

                if tx.send(system_event).is_err() {
                    warn!("Cannot send Hyprland event to System Event Dispatcher. Aborting listener.");
                    return false;
                }

                true
            });
        });

    if res.is_err() {
        error!("Failed to spawn Hyprland Events Listener thread. Workspaces API may not function correctly.");
    }
}

fn setup_logging() {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
