use glib::Object;
use gtk4::glib;

glib::wrapper! {
    pub struct Wallclock(ObjectSubclass<imp::Wallclock>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Wallclock {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use std::time::Duration;

    use crate::ui;
    use glib::Properties;
    use gtk4::glib;
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Wallclock)]
    pub struct Wallclock;

    #[glib::object_subclass]
    impl ObjectSubclass for Wallclock {
        const NAME: &'static str = "StilWallclock";
        type Type = super::Wallclock;
        type ParentType = gtk4::Box;
    }

    impl ObjectImpl for Wallclock {
        fn constructed(&self) {
            self.parent_constructed();

            let date_label = gtk4::Label::new(Some("--/--/--"));
            let time_label = gtk4::Label::new(Some("--:--"));

            let vbox = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Vertical)
                .valign(gtk4::Align::Center)
                .spacing(0)
                .build();
            vbox.append(&time_label);
            vbox.append(&date_label);

            let taskbar_item = ui::TaskbarItem::new();
            taskbar_item.set_content(&vbox);

            let host = self.obj();
            host.add_css_class("wallclock");
            host.append(&taskbar_item);

            update_clock(time_label.downgrade(), date_label.downgrade());
        }
    }

    impl WidgetImpl for Wallclock {}

    impl BoxImpl for Wallclock {}

    fn update_clock(time: glib::WeakRef<gtk4::Label>, date: glib::WeakRef<gtk4::Label>) {
        let Some(time) = time.upgrade() else { return };
        let Some(date) = date.upgrade() else { return };

        let Ok(now) = glib::DateTime::now_local() else {
            // Retry in 30 seconds if we fail to get the time
            glib::timeout_add_seconds_local_once(30, move || update_clock(time.downgrade(), date.downgrade()));
            return;
        };

        let date_str = now.format("%d/%m/%Y").unwrap_or_else(|_| "--/--/--".into());
        date.set_text(&date_str);

        let time_str = now.format("%H:%M").unwrap_or_else(|_| "--:--".into());
        time.set_text(&time_str);

        let diff_ms = 60_000.0 - now.seconds() * 1000.0;
        glib::timeout_add_local_once(Duration::from_millis(diff_ms as u64), move || {
            update_clock(time.downgrade(), date.downgrade())
        });
    }
}
