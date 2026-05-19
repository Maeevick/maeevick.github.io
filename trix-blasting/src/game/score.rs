use bevy::prelude::*;

#[derive(Resource, Default)]
pub(crate) struct Score {
    pub(crate) value: u32,
}

#[derive(Component)]
pub(crate) struct ScoreDisplay;

pub(crate) fn score_a_kill_point(current: u32) -> u32 {
    current + 1
}

pub(crate) fn score_wave_bonus(current: u32, spawn_count: usize) -> u32 {
    current + spawn_count as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_zero_score_when_kill_then_score_is_one() {
        assert_eq!(score_a_kill_point(0), 1);
    }

    #[test]
    fn given_any_score_when_kill_then_increments_by_one() {
        assert_eq!(score_a_kill_point(42), 43);
    }

    #[test]
    fn given_zero_score_when_wave_bonus_then_adds_spawn_count() {
        assert_eq!(score_wave_bonus(0, 10), 10);
    }

    #[test]
    fn given_score_when_wave_bonus_with_zero_count_then_unchanged() {
        assert_eq!(score_wave_bonus(15, 0), 15);
    }
}
