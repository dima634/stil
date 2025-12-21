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
    use gtk4::gio;
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
            host.add_css_class("workspace-list");

            let flowbox = gtk4::FlowBox::builder()
                .column_spacing(2)
                .orientation(gtk4::Orientation::Horizontal)
                .selection_mode(gtk4::SelectionMode::None)
                .build();

            host.append(&flowbox);

            let model = gio::ListStore::new::<ui::WorkspaceModel>();
            let current_workspace = desktop().get_current_workspace_id();

            let mut workspaces = desktop().get_workspaces();
            workspaces.sort_by_key(|ws| ws.id());

            for workspace in workspaces {
                let item =
                    ui::WorkspaceModel::new(workspace.id(), workspace.name(), workspace.id() == current_workspace);
                model.append(&item);
            }

            flowbox.bind_model(Some(&model), |item| {
                let workspace_item = item
                    .downcast_ref::<ui::WorkspaceModel>()
                    .expect("WorkspaceModel expected");
                make_workspace_taskbar_item(workspace_item).upcast()
            });

            events().connect_workspace_created(glib::clone!(
                #[weak]
                model,
                move |id, name| {
                    let item = ui::WorkspaceModel::new(id, &name, false);
                    model.insert_sorted(&item, |a, b| {
                        let a_id = a.downcast_ref::<ui::WorkspaceModel>().map(|ws| ws.id()).unwrap_or(0);
                        let b_id = b.downcast_ref::<ui::WorkspaceModel>().map(|ws| ws.id()).unwrap_or(0);
                        a_id.cmp(&b_id)
                    });
                }
            ));

            events().connect_workspace_opened(glib::clone!(
                #[weak]
                model,
                move |opened_workspace_id| {
                    for i in 0..model.n_items() {
                        if let Some(item) = model.item(i).and_downcast::<ui::WorkspaceModel>() {
                            item.set_is_current(item.id() == opened_workspace_id);
                        }
                    }
                }
            ));

            events().connect_workspace_destroyed(glib::clone!(
                #[weak]
                model,
                move |id| model.retain(|el| el.downcast_ref::<ui::WorkspaceModel>().is_some_and(|ws| ws.id() != id))
            ));
        }
    }

    impl WidgetImpl for WorkspaceList {}

    impl BoxImpl for WorkspaceList {}

    fn make_workspace_taskbar_item(workspace_model: &ui::WorkspaceModel) -> ui::TaskbarItem {
        let label = gtk4::Label::builder()
            .label(workspace_model.name())
            .valign(gtk4::Align::Center)
            .halign(gtk4::Align::Center)
            .build();

        let taskbar_item = ui::TaskbarItem::new();
        taskbar_item.set_content(&label);
        taskbar_item.set_highlighted(workspace_model.is_current());

        workspace_model
            .bind_property("is-current", &taskbar_item, "highlighted")
            .sync_create()
            .build();

        taskbar_item
    }
}
