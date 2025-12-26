use super::notification_model::NotificationModel;
use crate::events;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4_layer_shell::*;

pub fn create_notifications_window() -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::builder()
        .css_classes(["notifications-window"])
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Top, true);
    window.set_margin(Edge::Top, 128);

    window.set_child(Some(&create_notifications_list()));

    window
}

struct Notification {
    window_pool: Vec<gtk4::ApplicationWindow>,
    list_store: gtk4::gio::ListStore,
}

impl Notification {
    fn with_max_notifications(max: usize) -> Self {
        let list_store = gtk4::gio::ListStore::new::<NotificationModel>();
        let mut window_pool = Vec::with_capacity(max);
        
        for _ in 0..max {
            let window = create_notifications_window1();
            window_pool.push(window);
        }



        Self { window_pool, list_store  }
    }
}

fn create_notifications_window1() -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::builder()
        .css_classes(["notifications-window"])
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Top, true);

    window
}

fn create_notifications_list() -> gtk4::ScrolledWindow {
    let model = gtk4::gio::ListStore::new::<NotificationModel>();

    let factory = gtk4::SignalListItemFactory::new();
    factory.connect_setup(|_, list_item| {
        let list_item = list_item.downcast_ref::<gtk4::ListItem>().expect("should be ListItem");
        let notification_widget = create_notification_widget(list_item);
        list_item.set_child(Some(&notification_widget));
    });

    events().connect_notification(glib::clone!(
        #[weak]
        model,
        move |summary, body, icon| {
            let notification = NotificationModel::new(&summary, body.as_deref(), icon.as_deref());
            model.append(&notification);
        }
    ));

    let list_view = gtk4::ListView::new(Some(gtk4::NoSelection::new(Some(model))), Some(factory));
    
    let scrolled_window = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .max_content_height(400)
        .propagate_natural_height(true)
        .child(&list_view)
        .build();

    scrolled_window
}

fn create_notification_widget(list_item: &gtk4::ListItem) -> gtk4::Box {
    let header = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .css_classes(["header"])
        .build();
    list_item
        .property_expression("item")
        .chain_property::<NotificationModel>("summary")
        .bind(&header, "label", gtk4::Widget::NONE);

    let body = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .css_classes(["body"])
        .build();
    list_item
        .property_expression("item")
        .chain_property::<NotificationModel>("body")
        .bind(&body, "label", gtk4::Widget::NONE);

    let v_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .halign(gtk4::Align::Center)
        .build();
    v_box.append(&header);
    v_box.append(&body);

    let icon = gtk4::Image::builder()
        .valign(gtk4::Align::Center)
        .pixel_size(32)
        .build();
    icon.bind_property("icon-name", &icon, "visible")
        .transform_to(|_, icon_name: Option<&str>| Some(icon_name.map_or(false, |name| !name.is_empty())))
        .sync_create()
        .build();
    list_item
        .property_expression("item")
        .chain_property::<NotificationModel>("app_icon")
        .bind(&icon, "icon-name", gtk4::Widget::NONE);

    let h_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .halign(gtk4::Align::Center)
        .css_classes(["notification"])
        .spacing(6)
        .build();
    h_box.append(&icon);
    h_box.append(&v_box);

    h_box
}
