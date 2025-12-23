mod power_ctl_flyout;

pub use power_ctl_flyout::create_power_ctl_flyout;

use gtk4::prelude::*;
use gtk4_layer_shell::*;

pub fn create_flyout_window() -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::builder().build();
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_hide_on_close(true);
    window.add_css_class("flyout-window");
    window
}
