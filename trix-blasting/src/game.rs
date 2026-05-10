use bevy::{prelude::*, window::PresentMode};

pub const WINDOW_WIDTH: f32 = 400.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
const WINDOW_WIDTH_PX: u32 = 400;
const WINDOW_HEIGHT_PX: u32 = 800;

pub fn create_app(for_wasm: bool) -> App {
    let window = if for_wasm {
        Window {
            title: "Trix Blasting".into(),
            resolution: (WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX).into(),
            canvas: Some("#game-canvas".to_string()),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }
    } else {
        Window {
            title: "Trix Blasting".into(),
            resolution: (WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX).into(),
            ..default()
        }
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }))
    .insert_resource(ClearColor(Color::linear_rgb(0.05, 0.05, 0.1)))
    .add_systems(Startup, setup_camera);
    app
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
    println!("Trix Blasting ready.");
}
