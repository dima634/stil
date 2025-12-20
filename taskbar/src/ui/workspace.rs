use glib::Object;
use gtk4::glib::{self, Properties};
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

glib::wrapper! {
    pub struct Workspace(ObjectSubclass<imp::Workspace>);
}

impl Workspace {
    pub fn new(id: i32, name: &str, is_current: bool) -> Self {
        Object::builder()
            .property("id", id)
            .property("name", name)
            .property("is-current", is_current)
            .build()
    }
}

mod imp {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Workspace)]
    pub struct Workspace {
        #[property(get, set, construct_only)]
        id: Cell<i32>,
        #[property(get, set, construct_only)]
        name: RefCell<String>,
        #[property(get, set)]
        is_current: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Workspace {
        const NAME: &'static str = "StilWorkspace";
        type Type = super::Workspace;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Workspace {}
}
