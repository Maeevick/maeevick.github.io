use bevy::prelude::*;

#[derive(Resource, Default)]
pub(crate) struct RestartPending(pub(crate) bool);

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
