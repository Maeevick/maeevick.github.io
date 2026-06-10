use super::GameWindow;
use bevy::prelude::*;
use rand::RngExt;

pub(crate) const SWARM_START_Y: f32 = GameWindow::HEIGHT / 2.0 - 60.0;

pub(crate) const ALIEN_BULLET_WIDTH: f32 = 4.0;
pub(crate) const ALIEN_BULLET_HEIGHT: f32 = 10.0;
pub(crate) const ALIEN_BULLET_BASE_SPEED: f32 = 150.0;
pub(crate) const ALIEN_SHOOT_INTERVAL_MIN: f32 = 1.5;
pub(crate) const ALIEN_SHOOT_INTERVAL_MAX: f32 = 3.0;
pub(crate) const ALIEN_SHOOTER_PROBABILITY: f32 = 0.3;

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

#[derive(Component)]
pub(crate) struct AlphaFadeIn {
    pub(crate) timer: Timer,
}

#[derive(Resource)]
pub(crate) struct Swarm {
    pub(crate) wave: u32,
    pub(crate) spawn_count: usize,
    pub(crate) center_x: f32,
    pub(crate) center_y: f32,
    pub(crate) direction: f32,
}

impl Swarm {
    pub(crate) fn new() -> Self {
        Self {
            wave: 1,
            spawn_count: Alien::COLS,
            center_x: 0.0,
            center_y: SWARM_START_Y,
            direction: 1.0,
        }
    }

    pub(crate) fn reset_position(&mut self) {
        self.center_x = 0.0;
        self.center_y = SWARM_START_Y;
        self.direction = 1.0;
    }
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

pub(crate) const ALIEN_SHAPES: [[bool; 25]; 5] = [
    // crab
    [
        false, true, false, true, false, true, true, true, true, true, true, true, false, true,
        true, false, true, true, true, false, true, false, false, false, true,
    ],
    // squid
    [
        false, false, true, false, false, false, true, true, true, false, true, true, false, true,
        true, true, false, true, false, true, false, true, false, true, false,
    ],
    // octopus
    [
        false, true, true, true, false, true, true, true, true, true, true, false, true, false,
        true, true, true, true, true, true, false, true, false, true, false,
    ],
    // bat
    [
        true, false, false, false, true, true, true, false, true, true, true, true, true, true,
        true, false, false, true, false, false, false, true, true, true, false,
    ],
    // star
    [
        true, false, true, false, true, false, true, true, true, false, true, true, false, true,
        true, false, true, true, true, false, true, false, true, false, true,
    ],
];

pub(crate) fn alien_col_x(col: usize, swarm_center_x: f32) -> f32 {
    let total_grid_width = Alien::COLS as f32 * Alien::SIZE + (Alien::COLS - 1) as f32 * Alien::GAP;
    swarm_center_x + col as f32 * (Alien::SIZE + Alien::GAP) - total_grid_width / 2.0
        + Alien::SIZE / 2.0
}

pub(crate) fn alien_row_y(row: usize, swarm_center_y: f32) -> f32 {
    swarm_center_y - row as f32 * Alien::DROP
}

pub(crate) fn rows_for_wave(wave: u32) -> usize {
    match wave {
        1 => 1,
        2 => 2,
        3 => 3,
        n => {
            let rand_factor: f32 = rand::rng().random_range(0.0..1.0);
            (((n - 3) as f32 * rand_factor).floor() as usize).clamp(1, 12)
        }
    }
}

pub(crate) fn alien_pixel_data(color: [u8; 4], shape: &[bool; 25]) -> Vec<u8> {
    shape
        .iter()
        .flat_map(|&filled| if filled { color } else { [0, 0, 0, 0] })
        .collect()
}

pub(crate) fn alien_pixel_data_bg(color: [u8; 4], bg: [u8; 4], shape: &[bool; 25]) -> Vec<u8> {
    shape
        .iter()
        .flat_map(|&filled| if filled { color } else { bg })
        .collect()
}

pub(crate) fn special_alien_pixel_data(
    color_a: [u8; 4],
    color_b: [u8; 4],
    shape: &[bool; 25],
) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_column_zero_when_computing_alien_x_then_aligns_with_left_grid_edge() {
        let x = alien_col_x(0, 0.0);
        let total_width = Alien::COLS as f32 * Alien::SIZE + (Alien::COLS - 1) as f32 * Alien::GAP;
        let expected = -total_width / 2.0 + Alien::SIZE / 2.0;
        assert!((x - expected).abs() < 0.01, "got {x}, expected {expected}");
    }

    #[test]
    fn given_last_column_when_computing_alien_x_then_mirrors_first_column() {
        let x_first = alien_col_x(0, 0.0);
        let x_last = alien_col_x(Alien::COLS - 1, 0.0);
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
    fn given_waves_1_to_3_when_computing_rows_then_matches_wave_number() {
        assert_eq!(rows_for_wave(1), 1);
        assert_eq!(rows_for_wave(2), 2);
        assert_eq!(rows_for_wave(3), 3);
    }

    #[test]
    fn given_wave_above_3_when_computing_rows_then_stays_within_bounds() {
        for _ in 0..200 {
            let result = rows_for_wave(50);
            assert!((1..=12).contains(&result), "expected 1..=12, got {result}");
        }
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
}
