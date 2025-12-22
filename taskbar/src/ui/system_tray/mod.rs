mod wallclock;

pub use wallclock::Wallclock;

use glib::Object;
use gtk4::glib;

glib::wrapper! {
    pub struct SystemTray(ObjectSubclass<imp::SystemTray>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl SystemTray {
    #[inline]
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use gtk4::glib;
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;

    use crate::ui;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::SystemTray)]
    pub struct SystemTray;

    #[glib::object_subclass]
    impl ObjectSubclass for SystemTray {
        const NAME: &'static str = "StilSystemTray";
        type Type = super::SystemTray;
        type ParentType = gtk4::Box;
    }

    #[glib::derived_properties]
    impl ObjectImpl for SystemTray {
        fn constructed(&self) {
            self.parent_constructed();

            let host = self.obj();
            host.set_halign(gtk4::Align::End);
            host.add_css_class("system-tray");
            host.append(&ui::Wallclock::new());
        }
    }

    impl WidgetImpl for SystemTray {}

    impl BoxImpl for SystemTray {}
}
