// use glib::Object;
// use gtk4::glib;
// use std::sync::LazyLock;

// pub fn system_events() -> &'static SystemEvents {
//     static INSTANCE: LazyLock<SystemEvents> = LazyLock::new(|| {
//         let instance = SystemEvents::new();
//         instance.listen();
//         instance
//     });
//     &INSTANCE
// }

// glib::wrapper! {
//     pub struct SystemEvents(ObjectSubclass<imp::SystemEvents>);
// }

// impl SystemEvents {
//     #[inline]
//     fn new() -> Self {
//         Object::builder().build()
//     }

//     fn listen(&self) {

//     }

//     fn emit_event(&self, event: &stil_core::SystemEvent) {

//     }
// }

// mod imp {
//     use gtk4::glib;
//     use gtk4::subclass::prelude::*;
//     use glib::subclass::Signal;
//     use std::sync::OnceLock;

//     #[derive(Default)]
//     pub struct SystemEvents;

//     #[glib::object_subclass]
//     impl ObjectSubclass for SystemEvents {
//         const NAME: &'static str = "StilSystemEvents";
//         type Type = super::SystemEvents;
//     }

//     impl ObjectImpl for SystemEvents {
//         fn signals() -> &'static [Signal] {
//             static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
//             SIGNALS.get_or_init(|| vec![
//                 Signal::builder("window-opened")
//                     .param_types([usize::static_type()])
//                     .build(),
//                 Signal::builder("window-closed")
//                     .param_types([usize::static_type()])
//                     .build(),
//                 Signal::builder("window-focused")
//                     .param_types([usize::static_type()])
//                     .build(),
//                 Signal::builder("workspace-created")
//                     .param_types([i32::static_type()])
//                     .build(),
//                 // Add more signals...
//             ])
//         }
//     }
// }
