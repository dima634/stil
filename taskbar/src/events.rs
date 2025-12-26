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

    /// Emits `(window_address, workspace_id)` when a window is closed
    pub fn connect_window_closed<F: Fn(u64, i32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "window-closed",
            false,
            glib::closure_local!(move |_: Self, addr: u64, workspace_id: i32| { f(addr, workspace_id) }),
        )
    }

    /// Emits `(window_address, from_workspace, to_workspace)` when a window is moved between workspaces
    pub fn connect_window_moved<F: Fn(u64, i32, i32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "window-moved",
            false,
            glib::closure_local!(move |_: Self, addr: u64, from_workspace: i32, to_workspace: i32| {
                f(addr, from_workspace, to_workspace)
            }),
        )
    }

    pub fn connect_keyboard_layout_changed<F: Fn(String) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "keyboard-layout-changed",
            false,
            glib::closure_local!(move |_: Self, new_layout: String| f(new_layout)),
        )
    }

    /// Emits `(summary, body, icon)` when a notification is received
    pub fn connect_notification<F: Fn(String, Option<String>, Option<String>, Option<gtk4::gdk::Texture>) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_closure(
            "notification",
            false,
            glib::closure_local!(move |_: Self,
                                       summary: String,
                                       body: Option<String>,
                                       icon: Option<String>,
                                       image: Option<gtk4::gdk::Texture>| {
                f(summary, body, icon, image)
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
                &[&addr_to_u64(window.address), &window.app_id, &window.workspace_id],
            ),
            SystemEvent::WindowClosed(window_closed) => self.emit_by_name(
                "window-closed",
                &[&addr_to_u64(window_closed.address), &window_closed.workspace_id],
            ),
            SystemEvent::WindowFocused(addr) => self.emit_by_name("window-focused", &[&addr_to_u64(addr)]),
            SystemEvent::WindowMoved(window_moved) => self.emit_by_name(
                "window-moved",
                &[
                    &addr_to_u64(window_moved.window_address),
                    &window_moved.from_workspace,
                    &window_moved.to_workspace,
                ],
            ),
            SystemEvent::KeyboardLayoutChanged(new_layout) => {
                self.emit_by_name("keyboard-layout-changed", &[&new_layout])
            }
            SystemEvent::Notification(notification) => {
                let image_data = notification.image.map(|image| {
                    let bytes = glib::Bytes::from_owned(image.data);
                    gtk4::gdk::MemoryTextureBuilder::new()
                        .set_bytes(Some(&bytes))
                        .set_height(image.height)
                        .set_width(image.width)
                        .set_stride(image.row_stride as usize)
                        .set_format(if image.has_alpha {
                            gtk4::gdk::MemoryFormat::R8g8b8a8
                        } else {
                            gtk4::gdk::MemoryFormat::R8g8b8
                        })
                        .build()
                });
                self.emit_by_name(
                    "notification",
                    &[
                        &notification.summary,
                        &notification.body,
                        &notification.icon,
                        &image_data,
                    ],
                )
            }
            SystemEvent::Empty => todo!(),
        }
    }
}

fn addr_to_u64(address: stil_core::Address) -> u64 {
    u64::try_from(address.0).unwrap_or(0)
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
                    // Workspace-related signals
                    Signal::builder("workspace-opened")
                        .param_types([i32::static_type()])
                        .build(),
                    Signal::builder("workspace-destroyed")
                        .param_types([i32::static_type()])
                        .build(),
                    Signal::builder("workspace-created")
                        .param_types([i32::static_type(), String::static_type()])
                        .build(),
                    // Windows-related signals
                    Signal::builder("window-focused")
                        .param_types([u64::static_type()])
                        .build(),
                    Signal::builder("window-opened")
                        .param_types([u64::static_type(), Option::<String>::static_type(), i32::static_type()])
                        .build(),
                    Signal::builder("window-closed")
                        .param_types([u64::static_type(), i32::static_type()])
                        .build(),
                    Signal::builder("window-moved")
                        .param_types([u64::static_type(), i32::static_type(), i32::static_type()])
                        .build(),
                    // Keyboard-related signals
                    Signal::builder("keyboard-layout-changed")
                        .param_types([String::static_type()])
                        .build(),
                    // Notifications
                    Signal::builder("notification")
                        .param_types([
                            String::static_type(),
                            Option::<String>::static_type(),
                            Option::<String>::static_type(),
                            Option::<gtk4::gdk::Texture>::static_type(),
                        ])
                        .build(),
                ]
            });
            &SIGNALS
        }
    }
}
