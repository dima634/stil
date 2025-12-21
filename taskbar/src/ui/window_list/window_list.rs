use crate::desktop;
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
        Object::builder()
            .property("current-workspace-id", desktop().get_current_workspace_id())
            .build()
    }
}

mod imp {
    use crate::{desktop, events, ui};
    use gtk4::gio;
    use gtk4::glib;
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;
    use std::cell::Cell;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::WindowList)]
    pub struct WindowList {
        #[property(get, set)]
        current_workspace_id: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WindowList {
        const NAME: &'static str = "StilWindowList";
        type Type = super::WindowList;
        type ParentType = gtk4::Box;
    }

    #[glib::derived_properties]
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

            let list_store = gio::ListStore::new::<ui::WindowModel>();

            flowbox.bind_model(Some(&list_store), |item| {
                let window = item.downcast_ref::<ui::WindowModel>().expect("WindowModel expected");
                let item = make_window_taskbar_item(window);
                item.upcast()
            });

            host.connect_current_workspace_id_notify(glib::clone!(
                #[weak]
                list_store,
                move |window_list| {
                    fill_window_list_store(&list_store, window_list.current_workspace_id());
                }
            ));

            events().connect_workspace_opened(glib::clone!(
                #[weak]
                host,
                move |workspace_id| host.set_current_workspace_id(workspace_id)
            ));

            events().connect_window_focused(glib::clone!(
                #[weak]
                list_store,
                move |addr: u64| {
                    for i in 0..list_store.n_items() {
                        if let Some(item) = list_store.item(i).and_downcast::<ui::WindowModel>() {
                            item.set_is_focused(item.address() == addr);
                        }
                    }
                }
            ));

            events().connect_window_opened(glib::clone!(
                #[weak]
                list_store,
                #[weak]
                host,
                move |addr: u64, app_id, workspace_id: i32| {
                    if host.current_workspace_id() != workspace_id {
                        return;
                    }

                    let icon = find_app_icon(app_id.as_ref());
                    let model = ui::WindowModel::new(addr, icon, false);
                    list_store.append(&model);
                }
            ));

            events().connect_window_closed(glib::clone!(
                #[weak]
                list_store,
                #[weak]
                host,
                move |addr: u64, workspace_id: i32| {
                    if host.current_workspace_id() != workspace_id {
                        return;
                    }

                    list_store.retain(|el| {
                        el.downcast_ref::<ui::WindowModel>()
                            .is_some_and(|window| window.address() != addr)
                    });
                }
            ));
        }
    }

    impl WidgetImpl for WindowList {}

    impl BoxImpl for WindowList {}

    fn fill_window_list_store(list_store: &gio::ListStore, workspace_id: i32) {
        list_store.remove_all();
        let windows = desktop().get_workspace_windows(workspace_id);

        for window in windows {
            let Ok(addr) = window.address().0.try_into() else {
                continue;
            };

            let icon = find_app_icon(window.app_id());
            let model = ui::WindowModel::new(addr, icon, window.is_focused());
            list_store.append(&model);
        }
    }

    fn find_app_icon(app_id: Option<&String>) -> String {
        if let Some(app_id) = app_id
            && let Some(app) = desktop().get_app(app_id)
            && let Some(app_icon) = app.icon()
        {
            app_icon.clone()
        } else {
            "unknown".to_string()
        }
    }

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
