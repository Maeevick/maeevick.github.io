use super::*;
use bevy::math::Vec2;

// /////////////////////////////////////////////////////////////
// POSITIONS
// /////////////////////////////////////////////////////////////

pub fn alien_col_x(col: usize, swarm_center_x: f32) -> f32 {
    let total_grid_width =
        ALIEN_COLS as f32 * ALIEN_RENDERED_SIZE + (ALIEN_COLS - 1) as f32 * ALIEN_GAP;
    swarm_center_x + col as f32 * (ALIEN_RENDERED_SIZE + ALIEN_GAP) - total_grid_width / 2.0
        + ALIEN_RENDERED_SIZE / 2.0
}

pub fn alien_row_y(row: usize, swarm_center_y: f32) -> f32 {
    swarm_center_y - row as f32 * ALIEN_DROP_DISTANCE
}

// /////////////////////////////////////////////////////////////
// SPEED
// /////////////////////////////////////////////////////////////

pub fn speed_after_hit(current: f32) -> f32 {
    (current + SPEED_PER_HIT).max(BASE_GAME_SPEED)
}

pub fn speed_after_miss(current: f32) -> f32 {
    current + SPEED_PER_PLAYER_MISS
}

pub fn speed_after_wave(current: f32, alien_count: usize) -> f32 {
    current + alien_count as f32
}

pub fn burst_delay_secs(current_speed: f32) -> f32 {
    0.1 * (BASE_GAME_SPEED / current_speed)
}

// /////////////////////////////////////////////////////////////
// COLLISION
// /////////////////////////////////////////////////////////////

pub fn aabb_overlaps(pos_a: Vec2, half_a: Vec2, pos_b: Vec2, half_b: Vec2) -> bool {
    (pos_a.x - pos_b.x).abs() < half_a.x + half_b.x
        && (pos_a.y - pos_b.y).abs() < half_a.y + half_b.y
}

// /////////////////////////////////////////////////////////////
// WAVES
// /////////////////////////////////////////////////////////////

pub fn rows_for_wave_formula(wave: u32, rand_factor: f32) -> usize {
    match wave {
        1 => 1,
        2 => 2,
        3 => 3,
        n => (((n - 3) as f32 * rand_factor).floor() as usize).clamp(1, 12),
    }
}

// /////////////////////////////////////////////////////////////
// PIXEL DATA
// /////////////////////////////////////////////////////////////

pub fn alien_pixel_data(color: [u8; 4], shape: &[bool; 25]) -> Vec<u8> {
    shape
        .iter()
        .flat_map(|&filled| if filled { color } else { [0, 0, 0, 0] })
        .collect()
}

pub fn alien_pixel_data_bg(color: [u8; 4], bg: [u8; 4], shape: &[bool; 25]) -> Vec<u8> {
    shape
        .iter()
        .flat_map(|&filled| if filled { color } else { bg })
        .collect()
}

pub fn special_alien_pixel_data(color_a: [u8; 4], color_b: [u8; 4], shape: &[bool; 25]) -> Vec<u8> {
    let mut data = Vec::with_capacity(100);
    for row in 0..5usize {
        for col in 0..5usize {
            let t = ((row + col) as f32 / 8.0).clamp(0.0, 1.0);
            if shape[row * 5 + col] {
                for ch in 0..4 {
                    data.push((color_a[ch] as f32 * (1.0 - t) + color_b[ch] as f32 * t) as u8);
                }
            } else {
                data.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    data
}

// /////////////////////////////////////////////////////////////
// TESTS
// /////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_column_zero_when_computing_alien_x_then_aligns_with_left_grid_edge() {
        let x = alien_col_x(0, 0.0);
        let total_width =
            ALIEN_COLS as f32 * ALIEN_RENDERED_SIZE + (ALIEN_COLS - 1) as f32 * ALIEN_GAP;
        let expected = -total_width / 2.0 + ALIEN_RENDERED_SIZE / 2.0;
        assert!((x - expected).abs() < 0.01, "got {x}, expected {expected}");
    }

    #[test]
    fn given_last_column_when_computing_alien_x_then_mirrors_first_column() {
        let x_first = alien_col_x(0, 0.0);
        let x_last = alien_col_x(ALIEN_COLS - 1, 0.0);
        assert!(
            (x_first + x_last).abs() < 0.01,
            "expected symmetry: {x_first} + {x_last} ≈ 0"
        );
    }

    #[test]
    fn given_row_zero_when_computing_alien_y_then_equals_swarm_center_y() {
        let y = alien_row_y(0, 100.0);
        assert!((y - 100.0).abs() < 0.01);
    }

    #[test]
    fn given_increasing_rows_when_computing_alien_y_then_each_row_is_lower() {
        let y0 = alien_row_y(0, 100.0);
        let y1 = alien_row_y(1, 100.0);
        let y2 = alien_row_y(2, 100.0);
        assert!(y0 > y1, "row 0 should be above row 1");
        assert!(y1 > y2, "row 1 should be above row 2");
    }

    #[test]
    fn given_base_speed_when_player_misses_then_speed_increases_by_penalty() {
        let result = speed_after_miss(BASE_GAME_SPEED);
        assert!((result - (BASE_GAME_SPEED + SPEED_PER_PLAYER_MISS)).abs() < 0.01);
    }

    #[test]
    fn given_any_speed_when_player_misses_then_speed_increases_by_fixed_penalty() {
        let result = speed_after_miss(120.0);
        assert!((result - (120.0 + SPEED_PER_PLAYER_MISS)).abs() < 0.01);
    }

    #[test]
    fn given_base_speed_when_bullet_hits_alien_then_speed_stays_at_floor() {
        let result = speed_after_hit(BASE_GAME_SPEED);
        assert!((result - BASE_GAME_SPEED).abs() < 0.01);
    }

    #[test]
    fn given_any_speed_when_bullet_hits_alien_then_speed_changes_by_fixed_delta() {
        let result = speed_after_hit(120.0);
        assert!((result - (120.0 + SPEED_PER_HIT)).abs() < 0.01);
    }

    #[test]
    fn given_base_speed_when_wave_clears_then_speed_increases_by_alien_count() {
        let alien_count = 10usize;
        let result = speed_after_wave(BASE_GAME_SPEED, alien_count);
        assert!((result - (BASE_GAME_SPEED + alien_count as f32)).abs() < 0.01);
    }

    #[test]
    fn given_overlapping_boxes_when_checking_aabb_then_returns_true() {
        assert!(aabb_overlaps(
            Vec2::ZERO,
            Vec2::splat(10.0),
            Vec2::new(5.0, 5.0),
            Vec2::splat(10.0)
        ));
    }

    #[test]
    fn given_separated_boxes_when_checking_aabb_then_returns_false() {
        assert!(!aabb_overlaps(
            Vec2::ZERO,
            Vec2::splat(5.0),
            Vec2::new(20.0, 0.0),
            Vec2::splat(5.0)
        ));
    }

    #[test]
    fn given_touching_edges_when_checking_aabb_then_returns_false() {
        assert!(!aabb_overlaps(
            Vec2::ZERO,
            Vec2::splat(5.0),
            Vec2::new(10.0, 0.0),
            Vec2::splat(5.0)
        ));
    }

    #[test]
    fn given_waves_1_to_3_when_computing_rows_then_matches_wave_number() {
        assert_eq!(rows_for_wave_formula(1, 0.5), 1);
        assert_eq!(rows_for_wave_formula(2, 0.5), 2);
        assert_eq!(rows_for_wave_formula(3, 0.5), 3);
    }

    #[test]
    fn given_wave_4_with_zero_factor_when_computing_rows_then_clamps_to_1() {
        assert_eq!(rows_for_wave_formula(4, 0.0), 1);
    }

    #[test]
    fn given_large_wave_with_full_factor_when_computing_rows_then_caps_at_12() {
        assert_eq!(rows_for_wave_formula(100, 1.0), 12);
    }

    #[test]
    fn given_full_shape_when_creating_alien_pixel_data_then_all_25_pixels_match_color() {
        let color = [255u8, 128, 0, 255];
        let data = alien_pixel_data(color, &[true; 25]);
        assert_eq!(data.len(), 100);
        for i in 0..25 {
            assert_eq!(&data[i * 4..(i + 1) * 4], &color);
        }
    }

    #[test]
    fn given_false_pixel_in_shape_when_creating_alien_pixel_data_then_pixel_is_transparent() {
        let color = [255u8, 0, 0, 255];
        let mut shape = [true; 25];
        shape[0] = false;
        let data = alien_pixel_data(color, &shape);
        assert_eq!(&data[0..4], &[0u8, 0, 0, 0]);
        assert_eq!(&data[4..8], &color);
    }

    #[test]
    fn given_two_colors_when_creating_special_pixel_data_then_corners_match() {
        let black = [0u8, 0, 0, 255];
        let white = [255u8, 255, 255, 255];
        let data = special_alien_pixel_data(black, white, &[true; 25]);
        assert_eq!(data.len(), 100);
        assert_eq!(&data[0..4], &black);
        assert_eq!(&data[96..100], &white);
    }

    #[test]
    fn given_base_speed_when_computing_burst_delay_then_equals_0_1() {
        let delay = burst_delay_secs(BASE_GAME_SPEED);
        assert!((delay - 0.1).abs() < 0.001);
    }

    #[test]
    fn given_double_speed_when_computing_burst_delay_then_delay_halves() {
        let delay = burst_delay_secs(BASE_GAME_SPEED * 2.0);
        assert!((delay - 0.05).abs() < 0.001);
    }
}
