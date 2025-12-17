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
            glib::closure_local!(move |_: Self, workspace_id: i32| {
                f(workspace_id);
            }),
        )
    }

    fn handle_event(&self, event: SystemEvent) {
        match event {
            SystemEvent::WorkspaceCreated(_) => todo!(),
            SystemEvent::WorkspaceDestroyed(_) => todo!(),
            SystemEvent::WorkspaceOpened(workspace_id) => self.emit_by_name::<()>("workspace-opened", &[&workspace_id]),
            SystemEvent::WindowOpened(_) => todo!(),
            SystemEvent::WindowClosed(_) => todo!(),
            SystemEvent::WindowFocused(_) => todo!(),
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
                    // Add more signals...
                ]
            });
            &SIGNALS
        }
    }
}
