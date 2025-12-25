use glib::Object;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

glib::wrapper! {
    pub struct NotificationModel(ObjectSubclass<imp::NotificationModel>);
}

impl NotificationModel {
    pub fn new(summary: &str, body: Option<&str>, app_icon: Option<&str>) -> Self {
        Object::builder()
            .property("summary", summary)
            .property("body", body)
            .property("app_icon", app_icon)
            .build()
    }
}

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::NotificationModel)]
    pub struct NotificationModel {
        #[property(get, set, construct_only)]
        summary: RefCell<String>,
        #[property(get, set, construct_only)]
        body: RefCell<Option<String>>,
        #[property(get, set, construct_only)]
        app_icon: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NotificationModel {
        const NAME: &'static str = "StilNotificationModel";
        type Type = super::NotificationModel;
    }

    #[glib::derived_properties]
    impl ObjectImpl for NotificationModel {}
}
