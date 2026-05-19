use super::GameWindow;
use bevy::prelude::*;

// /////////////////////////////////////////////////////////////
// CONSTANTS
// /////////////////////////////////////////////////////////////

const SWARM_START_Y: f32 = GameWindow::HEIGHT / 2.0 - 60.0;

// /////////////////////////////////////////////////////////////
// GAME MECHANICS
// /////////////////////////////////////////////////////////////

#[derive(Resource, Default)]
pub(crate) struct Score {
    pub(crate) value: u32,
}

#[derive(Resource)]
pub(crate) struct Speed {
    pub(crate) current: f32,
}

impl Speed {
    pub(crate) const BASE: f32 = 50.0;

    pub(crate) fn new() -> Self {
        Self {
            current: Self::BASE,
        }
    }
}

#[derive(Resource)]
pub(crate) struct Wave {
    pub(crate) number: u32,
    pub(crate) spawn_count: usize,
}

#[derive(Resource)]
pub(crate) struct SpeedsterBoost {
    pub(crate) multiplier: f32,
}

impl SpeedsterBoost {
    pub(crate) fn new() -> Self {
        Self { multiplier: 1.0 }
    }
}

// /////////////////////////////////////////////////////////////
// SWARM
// /////////////////////////////////////////////////////////////

#[derive(Resource)]
pub(crate) struct Swarm {
    pub(crate) center_x: f32,
    pub(crate) center_y: f32,
    pub(crate) direction: f32,
}

impl Swarm {
    pub(crate) fn new() -> Self {
        Self {
            center_x: 0.0,
            center_y: SWARM_START_Y,
            direction: 1.0,
        }
    }
}

// /////////////////////////////////////////////////////////////
// PLAYER STATE
// /////////////////////////////////////////////////////////////

#[derive(Resource)]
pub(crate) struct PlayerShootCooldown(pub(crate) f32);

#[derive(Resource, Default)]
pub(crate) struct RestartPending(pub(crate) bool);

// /////////////////////////////////////////////////////////////
// TIMERS / EFFECTS
// /////////////////////////////////////////////////////////////

#[derive(Resource)]
pub(crate) struct WaveSplashTimer(pub(crate) Timer);

impl WaveSplashTimer {
    const DURATION: f32 = 1.5;

    pub(crate) fn new() -> Self {
        Self(Timer::from_seconds(Self::DURATION, TimerMode::Once))
    }
}

#[derive(Resource)]
pub(crate) struct CameraShake {
    pub(crate) elapsed: f32,
}

impl CameraShake {
    pub(crate) fn inactive() -> Self {
        Self { elapsed: f32::MAX }
    }
}
