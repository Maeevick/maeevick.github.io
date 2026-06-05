use bevy::prelude::*;

// /////////////////////////////////////////////////////////////
// VISUAL / ANIMATION
// /////////////////////////////////////////////////////////////

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
