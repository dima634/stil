mod components;

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn main() {
    let application = gtk4::Application::new(Some("com.stil"), Default::default());
    application.connect_startup(|_| load_css());
    application.connect_activate(activate);
    application.run();
}

fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(include_str!("styles.css"));

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_USER,
    );
}

fn activate(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.auto_exclusive_zone_enable();

    window.set_margin(Edge::Left, 2);
    window.set_margin(Edge::Right, 2);
    window.set_margin(Edge::Top, 2);
    window.set_margin(Edge::Bottom, 2);

    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_anchor(Edge::Top, false);
    window.set_anchor(Edge::Bottom, true);

    window.add_css_class("taskbar");
    window.set_height_request(50);

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 3);
    hbox.set_halign(gtk4::Align::Center);
    hbox.set_valign(gtk4::Align::Center);

    for _ in 0..10 {
        let item = components::TaskbarItem::new();
        hbox.append(&item);
    }

    window.set_child(Some(&hbox));

    window.present();
}
