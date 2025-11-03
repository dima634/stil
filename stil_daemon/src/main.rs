mod hyprland;

fn main() {
    setup_logging();

    // hyprland::HyprEvents::listen(|event| match event {
    //     hyprland::Event::OpenWindow(open_window) => {
    //         println!("Window opened: {}", open_window.window_address);
    //     }
    //     hyprland::Event::CloseWindow(close_window) => {
    //         println!("Window closed: {}", close_window.window_address);
    //     }
    //     hyprland::Event::ActiveWindow(active_window) => {
    //         println!("Active window: {}", active_window.window_class);
    //     }
    // });

    let mut ctl = hyprland::HyprCtl::default();
    let clients = ctl.run(hyprland::ClientsCmd);
    println!("Clients: {:?}", clients);
    let clients = ctl.run(hyprland::ClientsCmd);
    println!("Clients: {:?}", clients);
}

fn setup_logging() {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
