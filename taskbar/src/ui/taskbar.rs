use glib::Object;
use gtk4::glib;

glib::wrapper! {
    pub struct Taskbar(ObjectSubclass<imp::Taskbar>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Taskbar {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use gtk4::glib;
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;

    #[derive(Default)]
    pub struct Taskbar;

    #[glib::object_subclass]
    impl ObjectSubclass for Taskbar {
        const NAME: &'static str = "StilTaskbar";
        type Type = super::Taskbar;
        type ParentType = gtk4::Box;
    }

    impl ObjectImpl for Taskbar {
        fn constructed(&self) {
            self.parent_constructed();

            let widgets = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .css_classes(["widgets"])
                .build();

            let system_tray = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .css_classes(["system-tray"])
                .build();

            let taskbar = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .valign(gtk4::Align::Center)
                .halign(gtk4::Align::Center)
                .css_classes(["app-icons"])
                .build();

            let host = self.obj();
            host.add_css_class("taskbar");
            host.set_homogeneous(true);
            host.set_height_request(50);

            host.append(&widgets);
            host.append(&taskbar);
            host.append(&system_tray);

            // For testing
            for _ in 0..10 {
                let item = crate::ui::TaskbarItem::new();
                taskbar.append(&item);
            }
        }
    }

    impl WidgetImpl for Taskbar {}

    impl BoxImpl for Taskbar {}
}
