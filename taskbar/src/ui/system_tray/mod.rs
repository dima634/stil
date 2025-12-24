use crate::{desktop, events, ui};
use gtk4::glib;
use gtk4::prelude::*;

mod wallclock;

pub fn create_system_tray() -> gtk4::Box {
    let system_tray = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .css_classes(["system-tray"])
        .halign(gtk4::Align::End)
        .spacing(2)
        .build();
    system_tray.append(&create_keyboard_layout_item());
    system_tray.append(&wallclock::create_wallclock());
    system_tray.append(&create_power_ctl_item());

    system_tray
}

fn create_power_ctl_item() -> ui::TaskbarItem {
    let power_ctl = ui::TaskbarItem::new();
    power_ctl.add_css_class("power-ctl");

    let flyout = ui::create_power_ctl_flyout();
    flyout
        .bind_property("visible", &power_ctl, "highlighted")
        .sync_create()
        .build();

    power_ctl.connect_clicked(move |_| {
        if flyout.is_visible() {
            flyout.close();
        } else {
            flyout.present();
        }
    });

    power_ctl
}

fn create_keyboard_layout_item() -> ui::TaskbarItem {
    let current_layout = desktop().get_current_keyboard_layout_code().unwrap_or_else(|| "??");
    let label = gtk4::Label::new(Some(current_layout));
    label.add_css_class("keyboard-layout");

    events().connect_keyboard_layout_changed(glib::clone!(
        #[weak]
        label,
        move |new_layout: String| label.set_label(&new_layout)
    ));

    let keyboard_layout = ui::TaskbarItem::new();
    keyboard_layout.set_content(&label);

    keyboard_layout
}
