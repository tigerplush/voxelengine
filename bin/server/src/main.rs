use bevy::{log::LogPlugin, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_systems(Startup, setup)
        .run()
}

fn setup() {
    info!("Hello from server");
}
