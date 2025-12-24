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
    use crate::ui;
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

            let workspace_list = ui::WorkspaceList::new();
            workspace_list.set_valign(gtk4::Align::Center);
            workspace_list.set_halign(gtk4::Align::Start);

            let window_list = ui::WindowList::new();
            window_list.set_valign(gtk4::Align::Center);
            window_list.set_halign(gtk4::Align::Center);

            let host = self.obj();
            host.add_css_class("taskbar");
            host.set_homogeneous(true);
            host.set_height_request(50);

            host.append(&workspace_list);
            host.append(&window_list);
            host.append(&ui::create_system_tray());
        }
    }

    impl WidgetImpl for Taskbar {}

    impl BoxImpl for Taskbar {}
}
