use crate::ui;
use gtk4::prelude::*;
use gtk4_layer_shell::*;

pub fn create_power_ctl_flyout() -> gtk4::ApplicationWindow {
    let list = gtk4::ListBox::builder()
        .selection_mode(gtk4::SelectionMode::None)
        .halign(gtk4::Align::Start)
        .build();

    list.append(&create_list_item("Restart", "system-reboot-symbolic"));
    list.append(&create_list_item("Shutdown", "system-shutdown-symbolic"));

    let window = ui::create_flyout_window();
    window.set_anchor(Edge::Right, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_margin_end(4);
    window.set_child(Some(&list));
    window
}

fn create_list_item(label: &str, icon: &str) -> gtk4::Box {
    let hbox = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .halign(gtk4::Align::Start)
        .spacing(4)
        .build();
    hbox.append(&gtk4::Image::from_icon_name(icon));
    hbox.append(&gtk4::Label::new(Some(label)));
    hbox
}
