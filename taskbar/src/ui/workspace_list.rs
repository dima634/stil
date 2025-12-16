use glib::Object;
use gtk4::glib;

glib::wrapper! {
    pub struct WorkspaceList(ObjectSubclass<imp::WorkspaceList>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl WorkspaceList {
    #[inline]
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use crate::{desktop, ui};
    use gtk4::glib;
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;

    #[derive(Default)]
    pub struct WorkspaceList;

    #[glib::object_subclass]
    impl ObjectSubclass for WorkspaceList {
        const NAME: &'static str = "StilWorkspaceList";
        type Type = super::WorkspaceList;
        type ParentType = gtk4::Box;
    }

    impl ObjectImpl for WorkspaceList {
        fn constructed(&self) {
            self.parent_constructed();

            let host = self.obj();
            host.set_orientation(gtk4::Orientation::Horizontal);
            host.add_css_class("workspace-list");

            for workspace in desktop().get_workspaces() {
                let item = ui::TaskbarItem::new();
            }
        }
    }

    impl WidgetImpl for WorkspaceList {}

    impl BoxImpl for WorkspaceList {}
}
