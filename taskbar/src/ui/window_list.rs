use glib::Object;
use gtk4::glib;

glib::wrapper! {
    pub struct WindowList(ObjectSubclass<imp::WindowList>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl WindowList {
    #[inline]
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use crate::{desktop, ui};
    use gtk4::gio;
    use gtk4::glib;
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;

    #[derive(Default)]
    pub struct WindowList;

    #[glib::object_subclass]
    impl ObjectSubclass for WindowList {
        const NAME: &'static str = "StilWindowList";
        type Type = super::WindowList;
        type ParentType = gtk4::Box;
    }

    impl ObjectImpl for WindowList {
        fn constructed(&self) {
            self.parent_constructed();

            let host = self.obj();
            host.add_css_class("window-list");

            let flowbox = gtk4::FlowBox::builder()
                .column_spacing(2)
                .orientation(gtk4::Orientation::Horizontal)
                .selection_mode(gtk4::SelectionMode::None)
                .build();

            host.append(&flowbox);

            let windows = desktop().get_current_workspace_windows();
            let list_store = gio::ListStore::new::<ui::WindowModel>();

            for window in windows {
                let mut icon = "unknown".to_string();

                if let Some(app_id) = window.app_id()
                    && let Some(app) = desktop().get_app(app_id)
                    && let Some(app_icon) = app.icon()
                {
                    icon = app_icon.clone();
                }

                let Ok(addr) = window.address().0.try_into() else {
                    continue;
                };

                let model = ui::WindowModel::new(addr, icon, window.is_focused());
                list_store.append(&model);
            }

            flowbox.bind_model(Some(&list_store), |item| {
                let window = item.downcast_ref::<ui::WindowModel>().expect("WindowModel expected");
                let item = make_window_taskbar_item(window);
                item.upcast()
            });
        }
    }

    impl WidgetImpl for WindowList {}

    impl BoxImpl for WindowList {}

    fn make_window_taskbar_item(window_model: &ui::WindowModel) -> ui::TaskbarItem {
        let icon = gtk4::Image::builder()
            .icon_name(&window_model.icon())
            .pixel_size(30)
            .build();
        let taskbar_item = ui::TaskbarItem::new();
        taskbar_item.set_content(&icon);

        window_model
            .bind_property("is-focused", &taskbar_item, "highlighted")
            .sync_create()
            .build();

        window_model
            .bind_property("is-focused", &taskbar_item, "status-bar")
            .transform_to(|_, focused| {
                Some(if focused {
                    ui::StatusBar::Focused
                } else {
                    ui::StatusBar::Hidden
                })
            })
            .sync_create()
            .build();

        taskbar_item
    }
}
