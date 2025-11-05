mod hyprland;
mod daemon;
mod desktop;

fn main() {
    setup_logging();
    daemon::start_daemon();
}

fn setup_logging() {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
