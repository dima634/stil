mod events;
mod ui;

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::sync::{Arc, LazyLock};

static DESKTOP: LazyLock<(Arc<stil_core::Desktop>, events::Events)> = LazyLock::new(|| {
    let (desktop, system_event_rx) = stil_core::Desktop::new();
    let events = events::Events::new(system_event_rx);
    (desktop, events)
});

fn desktop() -> &'static stil_core::Desktop {
    &DESKTOP.0
}

fn events() -> &'static events::Events {
    &DESKTOP.1
}

fn main() {
    let application = gtk4::Application::new(Some("com.dmytro.volovyk.stil"), Default::default());
    application.connect_startup(|_| load_css());
    application.connect_activate(create_taskbar_window);
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

fn create_taskbar_window(application: &gtk4::Application) {
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

    window.set_child(Some(&ui::Taskbar::new()));

    window.present();
}
