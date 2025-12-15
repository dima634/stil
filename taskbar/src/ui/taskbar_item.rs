use glib::Object;
use gtk4::glib;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "StilStatusBar")]
pub enum StatusBar {
    #[default]
    Hidden,
    Focused,
    Active,
}

glib::wrapper! {
    pub struct TaskbarItem(ObjectSubclass<imp::TaskbarItem>)
        @extends gtk4::Button, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl TaskbarItem {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use super::StatusBar;
    use glib::Properties;
    use gtk4::glib;
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;
    use std::cell::{Cell, RefCell};

    #[derive(Properties)]
    #[properties(wrapper_type = super::TaskbarItem)]
    pub struct TaskbarItem {
        #[property(get, set, default)]
        status_bar: Cell<StatusBar>,
        #[property(get, set)]
        highlighted: Cell<bool>,
        #[property(get, set)]
        icon: RefCell<String>,
    }

    impl Default for TaskbarItem {
        fn default() -> Self {
            Self {
                status_bar: StatusBar::Hidden.into(),
                highlighted: false.into(),
                icon: String::from("unknown").into(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TaskbarItem {
        const NAME: &'static str = "StilTaskbarItem";
        type Type = super::TaskbarItem;
        type ParentType = gtk4::Button;
    }

    #[glib::derived_properties]
    impl ObjectImpl for TaskbarItem {
        fn constructed(&self) {
            self.parent_constructed();

            let status_box_focused_width = 20;
            let status_box_active_width = 12;
            let status_box = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .css_classes(["status"])
                .width_request(status_box_active_width)
                .height_request(3)
                .halign(gtk4::Align::Center)
                .build();

            let status_box_at_bottom = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Vertical)
                .valign(gtk4::Align::End)
                .build();
            status_box_at_bottom.append(&status_box);

            let icon = gtk4::Image::builder().pixel_size(30).build();
            let status_box_over_icon = gtk4::Overlay::builder().child(&icon).build();
            status_box_over_icon.add_overlay(&status_box_at_bottom);

            let obj = self.obj();
            obj.add_css_class("taskbar-item");
            obj.set_size_request(40, 40);
            obj.set_child(Some(&status_box_over_icon));

            obj.bind_property("icon", &icon, "icon-name").sync_create().build();

            obj.connect_status_bar_notify(glib::clone!(
                #[weak]
                status_box,
                move |item| match item.status_bar() {
                    StatusBar::Hidden => status_box.set_css_classes(&["status"]),
                    StatusBar::Focused => {
                        status_box.set_css_classes(&["status", "focused"]);
                        status_box.set_width_request(status_box_focused_width);
                    }
                    StatusBar::Active => {
                        status_box.set_css_classes(&["status", "active"]);
                        status_box.set_width_request(status_box_active_width);
                    }
                }
            ));

            obj.connect_highlighted_notify(|item| {
                if item.highlighted() {
                    item.add_css_class("highlighted");
                } else {
                    item.remove_css_class("highlighted");
                }
            });
        }
    }

    impl WidgetImpl for TaskbarItem {}

    impl ButtonImpl for TaskbarItem {}
}
