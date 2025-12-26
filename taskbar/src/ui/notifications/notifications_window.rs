use crate::events;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4_layer_shell::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Once;
use std::time::Duration;

pub fn init_notifications(max_notifications: usize) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let pool = Rc::new(NotificationsPool {
            free_widgets: RefCell::new(Vec::from_iter(
                (0..max_notifications).map(|_| create_notification_window()),
            )),
            pending: RefCell::new(VecDeque::new()),
            displayed: RefCell::new(Vec::new()),
        });

        events().connect_notification(move |summary, body, app_icon| {
            handle_new_notification(summary, body, app_icon, pool.clone());
        });
    });
}

struct Notification {
    summary: String,
    body: Option<String>,
    app_icon: Option<String>,
}

struct NotificationsPool {
    pending: RefCell<VecDeque<Notification>>,
    free_widgets: RefCell<Vec<NotificationWidget>>,
    displayed: RefCell<Vec<NotificationWidget>>,
}

fn handle_new_notification(
    summary: String,
    body: Option<String>,
    app_icon: Option<String>,
    pool: Rc<NotificationsPool>,
) {
    pool.pending.borrow_mut().push_back(Notification {
        summary,
        body,
        app_icon,
    });

    display_next_notification(pool);
}

fn display_next_notification(pool: Rc<NotificationsPool>) {
    {
        let mut free_widgets = pool.free_widgets.borrow_mut();
        if free_widgets.is_empty() {
            return;
        }

        let mut pending = pool.pending.borrow_mut();
        let mut displayed = pool.displayed.borrow_mut();

        while !pending.is_empty() && !free_widgets.is_empty() {
            let notification = pending.pop_front().unwrap();
            let widget = free_widgets.pop().unwrap();
            let window = widget.window.clone();
            widget.set_from_notification(&notification);
            widget.window.present();
            displayed.push(widget);

            schedule_dismiss(pool.clone(), window);
        }
    }

    update_notifications_position(pool);
}

fn update_notifications_position(pool: Rc<NotificationsPool>) {
    const BASE_OFFSET_FROM_TOP: i32 = 128;
    const GAP: i32 = 6;
    const NOTIFICATION_HEIGHT: i32 = 48;

    for (i, widget) in pool.displayed.borrow().iter().enumerate() {
        widget.window.set_margin(
            Edge::Top,
            BASE_OFFSET_FROM_TOP + (GAP + NOTIFICATION_HEIGHT) * (i as i32),
        );
    }
}

fn schedule_dismiss(pool: Rc<NotificationsPool>, notification_window: gtk4::ApplicationWindow) {
    glib::timeout_add_local_once(Duration::from_secs(3), move || {
        let idx = pool
            .displayed
            .borrow()
            .iter()
            .position(|w| w.window == notification_window);

        if let Some(idx) = idx {
            let widget = pool.displayed.borrow_mut().remove(idx);
            widget.window.close();
            pool.free_widgets.borrow_mut().push(widget);
            display_next_notification(pool);
        }
    });
}

fn create_notification_window() -> NotificationWidget {
    let header = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .css_classes(["header"])
        .build();

    let body = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .css_classes(["body"])
        .build();

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

    let h_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .halign(gtk4::Align::Center)
        .css_classes(["notification"])
        .spacing(6)
        .build();
    h_box.append(&icon);
    h_box.append(&v_box);

    let window = gtk4::ApplicationWindow::builder()
        .css_classes(["notifications-window"])
        .hide_on_close(true)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Top, true);
    window.set_child(Some(&h_box));

    NotificationWidget {
        window,
        header,
        body,
        icon,
    }
}

#[derive(Debug, Clone)]
struct NotificationWidget {
    window: gtk4::ApplicationWindow,
    header: gtk4::Label,
    body: gtk4::Label,
    icon: gtk4::Image,
}

impl NotificationWidget {
    fn set_from_notification(&self, notification: &Notification) {
        self.header.set_label(&notification.summary);

        self.body.set_visible(notification.body.is_some());
        if let Some(body) = &notification.body {
            self.body.set_label(body);
        }

        self.icon.set_visible(notification.app_icon.is_some());
        if let Some(app_icon) = &notification.app_icon {
            self.icon.set_icon_name(Some(app_icon));
        }
    }
}
