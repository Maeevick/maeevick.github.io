use bevy::prelude::*;

// /////////////////////////////////////////////////////////////
// PLAYER
// /////////////////////////////////////////////////////////////

#[derive(Component)]
pub(crate) struct Trix;

#[derive(Component)]
pub(crate) struct PlayerBullet;

#[derive(Component)]
pub(crate) struct CooldownBar;

// /////////////////////////////////////////////////////////////
// ALIENS
// /////////////////////////////////////////////////////////////

#[derive(Component)]
pub(crate) struct Alien {
    pub(crate) col: usize,
    pub(crate) row: usize,
    pub(crate) color: Color,
}

impl Alien {
    pub(crate) const COLS: usize = 10;
    pub(crate) const SIZE: f32 = 25.0;
    pub(crate) const GAP: f32 = 5.0;
    pub(crate) const DROP: f32 = Self::SIZE + Self::GAP;
}

#[derive(Component)]
pub(crate) struct AlienBullet;

#[derive(Component)]
pub(crate) struct AlienShooter {
    pub(crate) timer: Timer,
}

#[derive(Component)]
pub(crate) struct Machinegunner {
    pub(crate) burst_count: u8,
    pub(crate) remaining: u8,
    pub(crate) burst_elapsed: f32,
    pub(crate) idle_elapsed: f32,
    pub(crate) idle_interval: f32,
}

#[derive(Component)]
pub(crate) struct Shielded {
    pub(crate) health: u8,
}

#[derive(Component)]
pub(crate) struct Speedster {
    pub(crate) multiplier: f32,
    pub(crate) base_color: Color,
    pub(crate) flash_elapsed: f32,
}

// /////////////////////////////////////////////////////////////
// VISUAL / ANIMATION
// /////////////////////////////////////////////////////////////

#[derive(Component)]
pub(crate) struct AlphaFadeIn {
    pub(crate) timer: Timer,
}

#[derive(Component)]
pub(crate) struct ExplosionParticle {
    pub(crate) velocity: Vec2,
    pub(crate) timer: Timer,
    pub(crate) lifetime: Timer,
}

#[derive(Component)]
pub(crate) struct BulletSplash {
    pub(crate) timer: Timer,
}

// /////////////////////////////////////////////////////////////
// UI ELEMENTS
// /////////////////////////////////////////////////////////////

#[derive(Component)]
pub(crate) struct GameOverText;

#[derive(Component)]
pub(crate) struct StartButton;

#[derive(Component)]
pub(crate) struct RestartButton;

#[derive(Component)]
pub(crate) struct WaveDisplay;

#[derive(Component)]
pub(crate) struct SplashText;
