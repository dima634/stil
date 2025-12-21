use glib::Object;
use gtk4::glib::{self, object::ObjectExt};
use std::{
    sync::mpsc::{Receiver, TryRecvError},
    time::Duration,
};
use stil_core::SystemEvent;

glib::wrapper! {
    pub struct Events(ObjectSubclass<imp::Events>);
}

impl Events {
    pub fn new(receiver: Receiver<SystemEvent>) -> Self {
        let obj: Self = Object::builder().build();

        let obj_clone = obj.clone();
        glib::timeout_add_local(Duration::from_millis(15), move || {
            loop {
                match receiver.try_recv() {
                    Ok(event) => obj_clone.handle_event(event),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return glib::ControlFlow::Break,
                }
            }
            glib::ControlFlow::Continue
        });

        obj
    }

    pub fn connect_workspace_opened<F: Fn(i32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "workspace-opened",
            false,
            glib::closure_local!(move |_: Self, workspace_id: i32| f(workspace_id)),
        )
    }

    pub fn connect_workspace_destroyed<F: Fn(i32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "workspace-destroyed",
            false,
            glib::closure_local!(move |_: Self, workspace_id: i32| f(workspace_id)),
        )
    }

    /// Emits `(workspace_id, workspace_name)` when new workspace is created
    pub fn connect_workspace_created<F: Fn(i32, String) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "workspace-created",
            false,
            glib::closure_local!(move |_: Self, id: i32, name: String| f(id, name)),
        )
    }

    pub fn connect_window_focused<F: Fn(u64) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "window-focused",
            false,
            glib::closure_local!(move |_: Self, addr: u64| f(addr)),
        )
    }

    pub fn connect_window_opened<F: Fn(u64, Option<String>, i32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "window-opened",
            false,
            glib::closure_local!(move |_: Self, addr: u64, app_id: Option<String>, workspace_id: i32| {
                f(addr, app_id, workspace_id)
            }),
        )
    }

    fn handle_event(&self, event: SystemEvent) {
        match event {
            SystemEvent::WorkspaceCreated(workspace) => {
                self.emit_by_name("workspace-created", &[&workspace.id, &workspace.name])
            }
            SystemEvent::WorkspaceDestroyed(workspace_id) => self.emit_by_name("workspace-destroyed", &[&workspace_id]),
            SystemEvent::WorkspaceOpened(workspace_id) => self.emit_by_name("workspace-opened", &[&workspace_id]),
            SystemEvent::WindowOpened(window) => self.emit_by_name(
                "window-opened",
                &[
                    &u64::try_from(window.address.0).unwrap_or(0),
                    &window.app_id,
                    &window.workspace_id,
                ],
            ),
            SystemEvent::WindowClosed(_) => todo!(),
            SystemEvent::WindowFocused(addr) => {
                self.emit_by_name("window-focused", &[&u64::try_from(addr.0).unwrap_or(0)])
            }
            SystemEvent::WindowMoved(_) => todo!(),
            SystemEvent::KeyboardLayoutChanged(_) => todo!(),
            SystemEvent::Empty => todo!(),
        }
    }
}

mod imp {
    use glib::subclass::Signal;
    use gtk4::glib;
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;
    use std::sync::LazyLock;

    #[derive(Default)]
    pub struct Events;

    #[glib::object_subclass]
    impl ObjectSubclass for Events {
        const NAME: &'static str = "StilEvents";
        type Type = super::Events;
    }

    impl ObjectImpl for Events {
        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
                vec![
                    Signal::builder("workspace-opened")
                        .param_types([i32::static_type()])
                        .build(),
                    Signal::builder("workspace-destroyed")
                        .param_types([i32::static_type()])
                        .build(),
                    Signal::builder("workspace-created")
                        .param_types([i32::static_type(), String::static_type()])
                        .build(),
                    Signal::builder("window-focused")
                        .param_types([u64::static_type()])
                        .build(),
                    Signal::builder("window-opened")
                        .param_types([u64::static_type(), Option::<String>::static_type(), i32::static_type()])
                        .build(),
                ]
            });
            &SIGNALS
        }
    }
}
