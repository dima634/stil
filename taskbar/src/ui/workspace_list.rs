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
    use crate::events;
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
            host.set_spacing(2);
            host.add_css_class("workspace-list");

            let current_workspace = desktop().get_current_workspace_id();
            for workspace in desktop().get_workspaces() {
                let workspace_name_label = gtk4::Label::builder()
                    .label(workspace.name())
                    .valign(gtk4::Align::Center)
                    .halign(gtk4::Align::Center)
                    .build();
                let item = ui::TaskbarItem::new();
                item.set_content(&workspace_name_label);
                let workspace_id = workspace.id();
                item.set_highlighted(current_workspace == workspace_id);

                events().connect_workspace_opened(glib::clone!(
                    #[weak]
                    item,
                    move |opened_workspace| item.set_highlighted(opened_workspace == workspace_id)
                ));

                host.append(&item);
            }
        }
    }

    impl WidgetImpl for WorkspaceList {}

    impl BoxImpl for WorkspaceList {}
}
