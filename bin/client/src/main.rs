use bevy::prelude::*;
use std::process::{Child, Command};

#[derive(Resource)]
struct ServerProcess(Child);

impl Drop for ServerProcess {
    fn drop(&mut self) {
        self.0.kill().ok();
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start_server)
        .run()
}

fn start_server(mut commands: Commands) {
    let child = Command::new(server_binary_name())
        .arg("--port=7777")
        .spawn()
        .expect("Failed to start server");

    commands.insert_resource(ServerProcess(child));
}

fn server_binary_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "server.exe"
    } else {
        "server"
    }
}
