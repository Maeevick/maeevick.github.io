use bevy::{prelude::*, window::PresentMode};

mod components;
use components::*;

mod resources;
use resources::*;

mod score;
use score::*;

mod speed;
use speed::*;

mod core;
use core::*;

mod systems;
use systems::*;

// /////////////////////////////////////////////////////////////
// GAME WINDOW
// /////////////////////////////////////////////////////////////

pub(crate) struct GameWindow;

impl GameWindow {
    pub(crate) const WIDTH: f32 = 400.0;
    pub(crate) const HEIGHT: f32 = 600.0;
}

// /////////////////////////////////////////////////////////////
// STATE
// /////////////////////////////////////////////////////////////

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameState {
    #[default]
    Menu,
    Running,
    WaveSplash,
    GameOver,
}

// /////////////////////////////////////////////////////////////
// APP BOOTSTRAP
// /////////////////////////////////////////////////////////////

pub(crate) fn create_app(for_wasm: bool) -> App {
    let window = if for_wasm {
        Window {
            title: "Trix Blasting".into(),
            resolution: (GameWindow::WIDTH as u32, GameWindow::HEIGHT as u32).into(),
            canvas: Some("#game-canvas".to_string()),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }
    } else {
        Window {
            title: "Trix Blasting".into(),
            resolution: (GameWindow::WIDTH as u32, GameWindow::HEIGHT as u32).into(),
            ..default()
        }
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }))
    .insert_resource(ClearColor(Color::linear_rgb(0.05, 0.05, 0.1)))
    .insert_resource(Speed::new())
    .insert_resource(Swarm::new())
    .insert_resource(PlayerShootCooldown(0.0))
    .insert_resource(Wave {
        number: 1,
        spawn_count: Alien::COLS,
    })
    .insert_resource(Score::default())
    .insert_resource(SpeedsterBoost::new())
    .insert_resource(RestartPending::default())
    .insert_resource(CameraShake::inactive())
    .insert_resource(WaveSplashTimer::new())
    .init_state::<GameState>()
    .add_systems(Startup, on_startup)
    .add_systems(OnEnter(GameState::Menu), on_menu_enter)
    .add_systems(OnExit(GameState::Menu), on_menu_exit)
    .add_systems(
        Update,
        (handle_menu_input, start_button_feedback).run_if(in_state(GameState::Menu)),
    )
    .add_systems(
        Update,
        (
            move_trix,
            handle_trix_shooting,
            move_player_bullets,
            update_cooldown_bar,
            move_swarm,
            handle_alien_shooting,
            handle_machinegunner_shooting,
            handle_speedsters,
            handle_speedster_flash,
            move_alien_bullets,
            animate_bullet_splash,
            animate_explosion,
            fade_in_aliens,
            check_bullet_alien_collisions,
            check_game_over_conditions,
            check_wave_cleared,
            update_score_display,
            update_wave_display,
            update_speed_display,
        )
            .run_if(in_state(GameState::Running)),
    )
    .add_systems(
        Update,
        (
            tick_wave_splash,
            fade_in_aliens,
            update_score_display,
            update_wave_display,
            update_speed_display,
        )
            .run_if(in_state(GameState::WaveSplash)),
    )
    .add_systems(
        Update,
        (
            move_alien_bullets,
            check_game_over_conditions,
            animate_bullet_splash,
            animate_explosion,
            update_camera_shake,
            detect_restart,
            apply_restart,
            restart_button_feedback,
        )
            .run_if(in_state(GameState::GameOver)),
    )
    .add_systems(
        OnEnter(GameState::WaveSplash),
        (on_wave_splash_enter, reset_trix_color, reset_camera),
    )
    .add_systems(OnEnter(GameState::GameOver), on_game_over_enter);
    app
}
