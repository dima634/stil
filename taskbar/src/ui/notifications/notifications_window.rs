use super::notification_model::NotificationModel;
use crate::events;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4_layer_shell::*;

pub fn create_notifications_window() -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::builder().build();
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_margin(Edge::Top, 256);
    window.add_css_class("notifications-window");
    window.set_child(Some(&create_notifications_list()));
    window.set_size_request(512, 512);
    window
}

fn create_notifications_list() -> gtk4::ListView {
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

    gtk4::ListView::new(Some(gtk4::NoSelection::new(Some(model))), Some(factory))
}

fn create_notification_widget(list_item: &gtk4::ListItem) -> gtk4::Box {
    let header = gtk4::Label::builder().halign(gtk4::Align::Start).build();
    list_item
        .property_expression("item")
        .chain_property::<NotificationModel>("summary")
        .bind(&header, "label", gtk4::Widget::NONE);

    let body = gtk4::Label::builder().halign(gtk4::Align::Start).build();
    list_item
        .property_expression("item")
        .chain_property::<NotificationModel>("body")
        .bind(&body, "label", gtk4::Widget::NONE);

    let v_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .halign(gtk4::Align::Center)
        .spacing(6)
        .build();
    v_box.append(&header);
    v_box.append(&body);

    let icon = gtk4::Image::builder().halign(gtk4::Align::Start).pixel_size(32).build();
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
