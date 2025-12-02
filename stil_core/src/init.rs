use crate::{
    db,
    hyprland::{Event, HyprEvents},
    service_locator::ServiceLocator,
    system_events::{SystemEvent, WindowMoved, WindowOpened, WorkspaceCreated},
};
use std::sync::Once;
use tracing::{error, info, warn};

static INIT: Once = Once::new();
pub fn init() {
    INIT.call_once(|| {
        setup_logging();
        db::migrate_up().expect("failed to migrate database up");
        ServiceLocator::desktop_mut().init();
        listen_for_hyprland_events();
        info!("Stil Core initialized");
    });
}

fn listen_for_hyprland_events() {
    let res = std::thread::Builder::new()
        .name("Hyprland Events Listener".to_string())
        .spawn(|| {
            let systems_events = ServiceLocator::system_event_dispatcher();
            let tx = systems_events.tx();

            HyprEvents::listen(|event| {
                // TODO: system event from Hyprland event
                let system_event = match event {
                    Event::OpenWindow(open_window) => SystemEvent::WindowOpened(WindowOpened {
                        address: open_window.window_address.0,
                        workspace_name: open_window.workspace_name,
                        class_name: open_window.window_class,
                    }),
                    Event::CloseWindow(close_window) => SystemEvent::WindowClosed(close_window.window_address.0),
                    Event::ActiveWindowV2(active_window) => SystemEvent::WindowFocused(active_window.address.0),
                    Event::MoveWindowV2(move_window_v2) => SystemEvent::WindowMoved(WindowMoved {
                        address: move_window_v2.window_address.0,
                        workspace_id: move_window_v2.workspace_id,
                        workspace_name: move_window_v2.workspace_name,
                    }),
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
