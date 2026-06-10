use bevy::prelude::*;

fn hue_to_rgb(h: f32) -> [f32; 3] {
    let h = h.rem_euclid(360.0);
    let x = 1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs();
    match (h / 60.0) as u32 {
        0 => [1.0, x, 0.0],
        1 => [x, 1.0, 0.0],
        2 => [0.0, 1.0, x],
        3 => [0.0, x, 1.0],
        4 => [x, 0.0, 1.0],
        _ => [1.0, 0.0, x],
    }
}

pub(crate) fn pick_rainbow_color(rng: &mut impl rand::RngExt) -> [u8; 4] {
    let hue: f32 = rng.random_range(0.0..360.0);
    let brightness: f32 = rng.random_range(0.7..1.0);
    let [r, g, b] = hue_to_rgb(hue);
    [
        (r * brightness * 255.0) as u8,
        (g * brightness * 255.0) as u8,
        (b * brightness * 255.0) as u8,
        255,
    ]
}

pub(crate) fn pick_complementary_pair(rng: &mut impl rand::RngExt) -> ([u8; 4], [u8; 4]) {
    let hue: f32 = rng.random_range(0.0..360.0);
    let brightness: f32 = rng.random_range(0.7..1.0);
    let [r1, g1, b1] = hue_to_rgb(hue);
    let [r2, g2, b2] = hue_to_rgb((hue + 180.0).rem_euclid(360.0));
    (
        [
            (r1 * brightness * 255.0) as u8,
            (g1 * brightness * 255.0) as u8,
            (b1 * brightness * 255.0) as u8,
            255,
        ],
        [
            (r2 * brightness * 255.0) as u8,
            (g2 * brightness * 255.0) as u8,
            (b2 * brightness * 255.0) as u8,
            255,
        ],
    )
}

pub(crate) fn brighten_bullet(color: Color) -> Color {
    let hsla = Hsla::from(color);
    Color::from(Hsla {
        lightness: hsla.lightness.max(0.65),
        ..hsla
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_hue_zero_when_converting_to_rgb_then_returns_pure_red() {
        let [r, g, b] = hue_to_rgb(0.0);
        assert!((r - 1.0).abs() < 0.001);
        assert!(g.abs() < 0.001);
        assert!(b.abs() < 0.001);
    }

    #[test]
    fn given_hue_120_when_converting_to_rgb_then_returns_pure_green() {
        let [r, g, b] = hue_to_rgb(120.0);
        assert!(r.abs() < 0.001);
        assert!((g - 1.0).abs() < 0.001);
        assert!(b.abs() < 0.001);
    }

    #[test]
    fn given_hue_240_when_converting_to_rgb_then_returns_pure_blue() {
        let [r, g, b] = hue_to_rgb(240.0);
        assert!(r.abs() < 0.001);
        assert!(g.abs() < 0.001);
        assert!((b - 1.0).abs() < 0.001);
    }

    #[test]
    fn given_negative_hue_when_converting_to_rgb_then_wraps_around() {
        let from_negative = hue_to_rgb(-60.0);
        let from_positive = hue_to_rgb(300.0);
        for i in 0..3 {
            assert!((from_negative[i] - from_positive[i]).abs() < 0.001);
        }
    }

    #[test]
    fn given_dark_color_when_brightening_then_lightness_clamps_to_floor() {
        let dark = Color::from(Hsla {
            hue: 200.0,
            saturation: 1.0,
            lightness: 0.2,
            alpha: 1.0,
        });
        let brightened = Hsla::from(brighten_bullet(dark));
        assert!((brightened.lightness - 0.65).abs() < 0.001);
    }

    #[test]
    fn given_already_bright_color_when_brightening_then_lightness_is_preserved() {
        let bright = Color::from(Hsla {
            hue: 200.0,
            saturation: 1.0,
            lightness: 0.9,
            alpha: 1.0,
        });
        let result = Hsla::from(brighten_bullet(bright));
        assert!((result.lightness - 0.9).abs() < 0.001);
    }
}
