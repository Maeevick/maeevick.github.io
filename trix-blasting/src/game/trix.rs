use super::GameWindow;
use bevy::prelude::*;

pub(crate) const TRIX_RENDERED_SIZE: f32 = 30.0;
pub(crate) const TRIX_COLOR: Color = Color::linear_rgb(0.0, 0.63, 0.87);
pub(crate) const TRIX_Y: f32 = GameWindow::BASELINE_Y + TRIX_RENDERED_SIZE / 2.0 + 5.0;
pub(crate) const TRIX_BASE_SPEED: f32 = 150.0;

pub(crate) const TRIX_BULLET_WIDTH: f32 = 4.0;
pub(crate) const TRIX_BULLET_HEIGHT: f32 = 12.0;
pub(crate) const TRIX_BULLET_SPEED: f32 = 200.0;
pub(crate) const TRIX_SHOOT_COOLDOWN: f32 = 0.25;

#[derive(Component)]
pub(crate) struct Trix;

#[derive(Component)]
pub(crate) struct TrixBullet;

#[derive(Component)]
pub(crate) struct ReloadBar;

#[derive(Resource)]
pub(crate) struct TrixShootCooldown(pub(crate) f32);

#[rustfmt::skip]
pub(crate) const SHIP_SHAPE: [bool; 225] = [
    false,false,false,false,false,false,false,true, false,false,false,false,false,false,false,
    false,false,false,false,false,false,true, true, true, false,false,false,false,false,false,
    false,false,false,false,false,true, true, true, true, true, false,false,false,false,false,
    false,false,false,false,false,false,false,true, false,false,false,false,false,false,false,
    false,false,false,false,false,false,false,true, false,false,false,false,false,false,false,
    false,false,false,false,false,false,false,true, false,false,false,false,false,false,false,
    true, false,false,false,false,false,true, true, true, false,false,false,false,false,true,
    true, false,false,false,false,true, true, true, true, true, false,false,false,false,true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, false,false,false,false,false,false,true, false,false,false,false,false,false,true,
    false,false,false,false,false,false,false,true, false,false,false,false,false,false,false,
    false,false,false,false,false,false,true, true, true, false,false,false,false,false,false,
    false,false,false,false,false,true, true, true, true, true, false,false,false,false,false,
    false,false,false,false,false,false,true, false,true, false,false,false,false,false,false,
];

#[rustfmt::skip]
pub(crate) const DESTROYED_SHIP_SHAPE: [bool; 25] = [
    false, true,  true,  true,  false,
    true,  false, true,  false, true,
    true,  true,  true,  true,  true,
    false, true,  false, true,  false,
    false, false, true,  false, false,
];

pub(crate) fn tick_reload(current: f32, delta: f32) -> f32 {
    (current - delta).max(0.0)
}

pub(crate) fn compute_reload_bar_width(reload: f32) -> f32 {
    TRIX_RENDERED_SIZE * (1.0 - (reload / TRIX_SHOOT_COOLDOWN).clamp(0.0, 1.0))
}

pub(crate) fn move_trix(current_x: f32, direction: f32, trix_speed: f32, delta: f32) -> f32 {
    let left = -GameWindow::WIDTH / 2.0 + TRIX_RENDERED_SIZE / 2.0;
    let right = GameWindow::WIDTH / 2.0 - TRIX_RENDERED_SIZE / 2.0;
    (current_x + direction * trix_speed * delta).clamp(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_zero_reload_when_tick_then_stays_at_zero() {
        assert!((tick_reload(0.0, 0.016) - 0.0).abs() < 0.001);
    }

    #[test]
    fn given_positive_reload_when_tick_then_decreases() {
        let result = tick_reload(0.25, 0.016);
        assert!((result - (0.25 - 0.016)).abs() < 0.001);
    }

    #[test]
    fn given_reload_less_than_delta_when_tick_then_clamps_to_zero() {
        let result = tick_reload(0.01, 0.016);
        assert!((result - 0.0).abs() < 0.001);
    }

    #[test]
    fn given_full_reload_when_compute_reload_bar_width_then_width_is_zero() {
        let result = compute_reload_bar_width(TRIX_SHOOT_COOLDOWN);
        assert!(result < 0.001);
    }

    #[test]
    fn given_zero_reload_when_compute_reload_bar_width_then_width_equals_rendered_size() {
        let result = compute_reload_bar_width(0.0);
        assert!((result - TRIX_RENDERED_SIZE).abs() < 0.001);
    }

    #[test]
    fn given_half_reload_when_compute_reload_bar_width_then_width_is_half_rendered_size() {
        let result = compute_reload_bar_width(TRIX_SHOOT_COOLDOWN / 2.0);
        assert!((result - TRIX_RENDERED_SIZE / 2.0).abs() < 0.001);
    }

    #[test]
    fn given_center_position_with_right_direction_when_move_trix_then_moves_right() {
        let result = move_trix(0.0, 1.0, 100.0, 0.016);
        assert!(result > 0.0);
    }

    #[test]
    fn given_far_right_position_with_right_direction_when_move_trix_then_clamps_to_boundary() {
        let right_boundary = GameWindow::WIDTH / 2.0 - TRIX_RENDERED_SIZE / 2.0;
        let result = move_trix(right_boundary, 1.0, 500.0, 1.0);
        assert!((result - right_boundary).abs() < 0.001);
    }
}
