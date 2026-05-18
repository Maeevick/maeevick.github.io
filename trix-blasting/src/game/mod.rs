use bevy::{prelude::*, window::PresentMode};

mod components;
use components::*;

mod resources;
use resources::*;

mod core;
use core::*;

mod systems;
use systems::*;

// /////////////////////////////////////////////////////////////
// CONSTANTS
// /////////////////////////////////////////////////////////////

pub(crate) const WINDOW_WIDTH: f32 = 400.0;
pub(crate) const WINDOW_HEIGHT: f32 = 600.0;
pub(crate) const WINDOW_WIDTH_PX: u32 = 400;
pub(crate) const WINDOW_HEIGHT_PX: u32 = 600;

pub(crate) const TRIX_RENDERED_SIZE: f32 = 30.0;
pub(crate) const TRIX_COLOR: Color = Color::linear_rgb(0.0, 0.63, 0.87);
pub(crate) const BASELINE_Y: f32 = -WINDOW_HEIGHT / 2.0 + 40.0;
pub(crate) const TRIX_Y: f32 = BASELINE_Y + TRIX_RENDERED_SIZE / 2.0 + 5.0;
pub(crate) const TRIX_BASE_SPEED: f32 = 150.0;

pub(crate) const ALIEN_COLS: usize = 10;
pub(crate) const ALIEN_RENDERED_SIZE: f32 = 25.0;
pub(crate) const ALIEN_GAP: f32 = 5.0;
pub(crate) const ALIEN_DROP_DISTANCE: f32 = ALIEN_RENDERED_SIZE + ALIEN_GAP;
pub(crate) const WALL_MARGIN: f32 = 5.0;
pub(crate) const SWARM_START_Y: f32 = WINDOW_HEIGHT / 2.0 - 60.0;

pub(crate) const BASE_GAME_SPEED: f32 = 50.0;
pub(crate) const SPEED_PER_HIT: f32 = -1.0;
pub(crate) const SPEED_PER_PLAYER_MISS: f32 = 1.0;

pub(crate) const PLAYER_BULLET_WIDTH: f32 = 4.0;
pub(crate) const PLAYER_BULLET_HEIGHT: f32 = 12.0;
pub(crate) const PLAYER_BULLET_BASE_SPEED: f32 = 200.0;
pub(crate) const PLAYER_SHOOT_COOLDOWN_SECS: f32 = 0.25;

pub(crate) const ALIEN_BULLET_WIDTH: f32 = 4.0;
pub(crate) const ALIEN_BULLET_HEIGHT: f32 = 10.0;
pub(crate) const ALIEN_BULLET_BASE_SPEED: f32 = 150.0;
pub(crate) const ALIEN_SHOOT_INTERVAL_MIN: f32 = 1.5;
pub(crate) const ALIEN_SHOOT_INTERVAL_MAX: f32 = 3.0;
pub(crate) const ALIEN_SHOOTER_PROBABILITY: f32 = 0.3;

pub(crate) const WAVE_SPLASH_DURATION_SECS: f32 = 1.5;
pub(crate) const ALIEN_FADE_DURATION_SECS: f32 = 0.3;

pub(crate) const MACHINEGUNNER_PROBABILITY: f32 = 0.12;
pub(crate) const MACHINEGUNNER_BURST_MIN: u8 = 3;
pub(crate) const MACHINEGUNNER_BURST_MAX: u8 = 8;
pub(crate) const MACHINEGUNNER_IDLE_MIN: f32 = 2.0;
pub(crate) const MACHINEGUNNER_IDLE_MAX: f32 = 5.0;

pub(crate) const SHIELDED_PROBABILITY: f32 = 0.10;
pub(crate) const SHIELDED_HEALTH_MIN: u8 = 2;
pub(crate) const SHIELDED_HEALTH_MAX: u8 = 5;

pub(crate) const SPEEDSTER_PROBABILITY: f32 = 0.08;
pub(crate) const SPEEDSTER_MULTIPLIER_MIN: f32 = 1.2;
pub(crate) const SPEEDSTER_MULTIPLIER_MAX: f32 = 2.0;

pub(crate) const START_BUTTON_COLOR: Color = Color::linear_rgb(0.89, 0.13, 0.74);
pub(crate) const START_BUTTON_HOVER: Color = Color::linear_rgb(0.71, 0.09, 0.58);
pub(crate) const RESTART_BUTTON_COLOR: Color = Color::linear_rgb(0.0, 0.63, 0.87);
pub(crate) const RESTART_BUTTON_HOVER: Color = Color::linear_rgb(0.10, 0.76, 1.0);

pub(crate) const CAMERA_SHAKE_DURATION: f32 = 1.5;
pub(crate) const CAMERA_SHAKE_AMPLITUDE: f32 = 6.0;

// /////////////////////////////////////////////////////////////
// STATE
// /////////////////////////////////////////////////////////////

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Phase {
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
    .insert_resource(Speed::new())
    .insert_resource(Swarm::new())
    .insert_resource(PlayerShootCooldown(0.0))
    .insert_resource(Wave {
        number: 1,
        spawn_count: ALIEN_COLS,
    })
    .insert_resource(Score::default())
    .insert_resource(SpeedsterBoost::new())
    .insert_resource(RestartPending::default())
    .insert_resource(CameraShake::inactive())
    .insert_resource(WaveSplashTimer(Timer::from_seconds(
        WAVE_SPLASH_DURATION_SECS,
        TimerMode::Once,
    )))
    .init_state::<Phase>()
    .add_systems(Startup, on_startup)
    .add_systems(OnEnter(Phase::Menu), on_menu_enter)
    .add_systems(OnExit(Phase::Menu), on_menu_exit)
    .add_systems(
        Update,
        (handle_menu_input, start_button_feedback).run_if(in_state(Phase::Menu)),
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
            .run_if(in_state(Phase::Running)),
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
            .run_if(in_state(Phase::WaveSplash)),
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
            .run_if(in_state(Phase::GameOver)),
    )
    .add_systems(
        OnEnter(Phase::WaveSplash),
        (on_wave_splash_enter, reset_trix_color, reset_camera),
    )
    .add_systems(OnEnter(Phase::GameOver), on_game_over_enter);
    app
}
