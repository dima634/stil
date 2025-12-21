use glib::Object;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

glib::wrapper! {
    pub struct WindowModel(ObjectSubclass<imp::WindowModel>);
}

impl WindowModel {
    pub fn new(id: u64, icon: String, is_focused: bool) -> Self {
        Object::builder()
            .property("address", id)
            .property("icon", icon)
            .property("is-focused", is_focused)
            .build()
    }
}

mod imp {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::WindowModel)]
    pub struct WindowModel {
        #[property(get, set, construct_only)]
        address: Cell<u64>,
        #[property(get, set, construct_only)]
        icon: RefCell<String>,
        #[property(get, set)]
        is_focused: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WindowModel {
        const NAME: &'static str = "StilWindowModel";
        type Type = super::WindowModel;
    }

    #[glib::derived_properties]
    impl ObjectImpl for WindowModel {}
}
