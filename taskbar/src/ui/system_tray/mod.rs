mod wallclock;

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
    use crate::ui::create_power_ctl_flyout;

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

            let power_ctl = ui::TaskbarItem::new();
            power_ctl.add_css_class("power-ctl");

            let flyout = create_power_ctl_flyout();

            power_ctl.connect_clicked(glib::clone!(
                #[weak]
                flyout,
                move |_| {
                    if flyout.is_visible() {
                        flyout.close();
                    } else {
                        flyout.present();
                    }
                }
            ));

            flyout
                .bind_property("visible", &power_ctl, "highlighted")
                .sync_create()
                .build();

            let host = self.obj();
            host.set_halign(gtk4::Align::End);
            host.add_css_class("system-tray");
            host.append(&super::wallclock::Wallclock::new());
            host.append(&power_ctl);
        }
    }

    impl WidgetImpl for SystemTray {}

    impl BoxImpl for SystemTray {}
}
