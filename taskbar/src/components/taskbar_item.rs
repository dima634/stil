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
    use std::cell::Cell;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::TaskbarItem)]
    pub struct TaskbarItem {
        #[property(get, set, default)]
        status_bar: Cell<StatusBar>,
        #[property(get, set)]
        highlighted: Cell<bool>,
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
            let status_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            status_box.add_css_class("status");
            status_box.set_size_request(status_box_active_width, 3);
            status_box.set_halign(gtk4::Align::Center);

            let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
            vbox.set_valign(gtk4::Align::End);
            vbox.append(&status_box);

            let obj = self.obj();
            obj.add_css_class("taskbar-item");
            obj.set_size_request(40, 40);
            obj.set_child(Some(&vbox));

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

    impl ButtonImpl for TaskbarItem {
        fn clicked(&self) {
            let obj = self.obj();
            match obj.status_bar() {
                StatusBar::Hidden => obj.set_status_bar(StatusBar::Active),
                StatusBar::Active => obj.set_status_bar(StatusBar::Focused),
                StatusBar::Focused => obj.set_status_bar(StatusBar::Hidden),
            }
        }
    }
}
