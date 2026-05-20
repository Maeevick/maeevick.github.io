use bevy::prelude::*;

const SPEED_PER_HIT: f32 = -1.0;
const SPEED_PER_MISS: f32 = 1.0;

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

#[derive(Component)]
pub(crate) struct SpeedDisplay;

pub(crate) fn decelerate_on_kill(current: f32) -> f32 {
    (current + SPEED_PER_HIT).max(Speed::BASE)
}

pub(crate) fn accelerate_on_miss(current: f32) -> f32 {
    current + SPEED_PER_MISS
}

pub(crate) fn accelerate_on_wave(current: f32, alien_count: usize) -> f32 {
    current + alien_count as f32
}

pub(crate) fn burst_delay_secs(current_speed: f32) -> f32 {
    0.1 * (Speed::BASE / current_speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_base_speed_when_player_misses_then_speed_increases_by_penalty() {
        let result = accelerate_on_miss(Speed::BASE);
        assert!((result - (Speed::BASE + SPEED_PER_MISS)).abs() < 0.01);
    }

    #[test]
    fn given_any_speed_when_player_misses_then_speed_increases_by_fixed_penalty() {
        let result = accelerate_on_miss(120.0);
        assert!((result - (120.0 + SPEED_PER_MISS)).abs() < 0.01);
    }

    #[test]
    fn given_base_speed_when_bullet_hits_alien_then_speed_stays_at_floor() {
        let result = decelerate_on_kill(Speed::BASE);
        assert!((result - Speed::BASE).abs() < 0.01);
    }

    #[test]
    fn given_any_speed_when_bullet_hits_alien_then_speed_changes_by_fixed_delta() {
        let result = decelerate_on_kill(120.0);
        assert!((result - (120.0 + SPEED_PER_HIT)).abs() < 0.01);
    }

    #[test]
    fn given_base_speed_when_wave_clears_then_speed_increases_by_alien_count() {
        let alien_count = 10usize;
        let result = accelerate_on_wave(Speed::BASE, alien_count);
        assert!((result - (Speed::BASE + alien_count as f32)).abs() < 0.01);
    }

    #[test]
    fn given_base_speed_when_computing_burst_delay_then_equals_0_1() {
        let delay = burst_delay_secs(Speed::BASE);
        assert!((delay - 0.1).abs() < 0.001);
    }

    #[test]
    fn given_double_speed_when_computing_burst_delay_then_delay_halves() {
        let delay = burst_delay_secs(Speed::BASE * 2.0);
        assert!((delay - 0.05).abs() < 0.001);
    }
}
