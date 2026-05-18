use super::Phase;
use super::*;
use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use rand::RngExt;

// /////////////////////////////////////////////////////////////
// DATA
// /////////////////////////////////////////////////////////////

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

// /////////////////////////////////////////////////////////////
// TYPE ALIASES
// /////////////////////////////////////////////////////////////

pub(crate) type CooldownBarQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Transform, &'static mut Sprite),
    (With<CooldownBar>, Without<Trix>),
>;

pub(crate) type WaveTransitionEntities<'w, 's> = Query<
    'w,
    's,
    Entity,
    Or<(
        With<PlayerBullet>,
        With<AlienBullet>,
        With<BulletSplash>,
        With<ExplosionParticle>,
        With<GameOverText>,
        With<Alien>,
    )>,
>;

// /////////////////////////////////////////////////////////////
// COLOR HELPERS
// /////////////////////////////////////////////////////////////

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

fn brighten_bullet(color: Color) -> Color {
    let hsla = Hsla::from(color);
    Color::from(Hsla {
        lightness: hsla.lightness.max(0.65),
        ..hsla
    })
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

// /////////////////////////////////////////////////////////////
// IMAGE HELPERS
// /////////////////////////////////////////////////////////////

pub(crate) fn make_alien_image(images: &mut Assets<Image>, pixel_data: Vec<u8>) -> Handle<Image> {
    let mut image = Image::new(
        Extent3d {
            width: 5,
            height: 5,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixel_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    images.add(image)
}

fn rows_for_wave(wave: u32) -> usize {
    if wave < 4 {
        return wave as usize;
    }
    let rand_factor: f32 = rand::rng().random_range(0.0..1.0);
    rows_for_wave_formula(wave, rand_factor)
}

// /////////////////////////////////////////////////////////////
// STARTUP
// /////////////////////////////////////////////////////////////

pub(crate) fn on_startup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, 3.0)),
            ..default()
        },
        Transform::from_xyz(0.0, BASELINE_Y, 0.0),
    ));

    #[rustfmt::skip]
    let ship: [bool; 225] = [
        false,false,false,false,false,false,false,true,false,false,false,false,false,false,false,
        false,false,false,false,false,false,true, true, true,false,false,false,false,false,false,
        false,false,false,false,false,true,true , true, true,true ,false,false,false,false,false,
        false,false,false,false,false,false,false,true,false,false,false,false,false,false,false,
        false,false,false,false,false,false,false,true,false,false,false,false,false,false,false,
        false,false,false,false,false,false,false,true,false,false,false,false,false,false,false,
        true ,false,false,false,false,false,true, true, true,false,false,false,false,false, true,
        true ,false,false,false,false,true, true, true, true,true,false,false,false,false, true,
        true, true ,true ,true ,true, true, true, true, true, true, true, true, true, true, true,
        true, true ,true ,true ,true, true, true, true, true, true, true, true, true, true, true,
        true ,false,false,false,false,false,false,true,false,false,false,false,false,false, true,
        false,false,false,false,false,false,false,true,false,false,false,false,false,false,false,
        false,false,false,false,false,false,true, true, true,false,false,false,false,false,false,
        false,false,false,false,false,true ,true, true, true,true ,false,false,false,false,false,
        false,false,false,false,false,false,true,false, true,false,false,false,false,false,false,
    ];
    let ship_data: Vec<u8> = ship
        .iter()
        .flat_map(|&f| {
            if f {
                [255u8, 255, 255, 255]
            } else {
                [0u8, 0, 0, 0]
            }
        })
        .collect();
    let mut ship_img = Image::new(
        Extent3d {
            width: 15,
            height: 15,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        ship_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    ship_img.sampler = ImageSampler::nearest();
    let ship_handle = images.add(ship_img);

    commands.spawn((
        Sprite {
            image: ship_handle,
            color: TRIX_COLOR,
            custom_size: Some(Vec2::splat(TRIX_RENDERED_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, TRIX_Y, 1.0),
        Trix,
    ));

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(TRIX_RENDERED_SIZE, 3.0)),
            ..default()
        },
        Transform::from_xyz(0.0, TRIX_Y - TRIX_RENDERED_SIZE / 2.0 - 5.0, 2.0),
        CooldownBar,
    ));

    let hud_y = WINDOW_HEIGHT / 2.0 - 22.0;
    let hud_font = TextFont {
        font_size: 16.0,
        ..default()
    };
    let hud_color = TextColor(Color::linear_rgb(0.75, 0.75, 0.75));

    commands.spawn((
        Text2d::new("WAVE\n1"),
        hud_font.clone(),
        hud_color,
        Transform::from_xyz(-130.0, hud_y, 5.0),
        WaveDisplay,
    ));

    commands.spawn((
        Text2d::new("SCORE\n0"),
        hud_font.clone(),
        hud_color,
        Transform::from_xyz(0.0, hud_y, 5.0),
        ScoreDisplay,
    ));

    commands.spawn((
        Text2d::new(format!("SPEED\n{}", BASE_GAME_SPEED as u32)),
        hud_font,
        hud_color,
        Transform::from_xyz(130.0, hud_y, 5.0),
        SpeedDisplay,
    ));

    println!("Trix Blasting ready.");
}

// /////////////////////////////////////////////////////////////
// SPAWN HELPERS
// /////////////////////////////////////////////////////////////

pub(crate) fn spawn_alien_wave(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    swarm: &Swarm,
    row_count: usize,
    fade_in: bool,
) {
    let mut rng = rand::rng();

    for row in 0..row_count {
        for col in 0..ALIEN_COLS {
            let x = alien_col_x(col, swarm.center_x);
            let y = alien_row_y(row, swarm.center_y);
            let color_bytes = pick_rainbow_color(&mut rng);
            let [r, g, b, _] = color_bytes;
            let alien_color =
                Color::srgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0);

            let special_roll = rng.random::<f32>();
            let is_machinegunner = special_roll < MACHINEGUNNER_PROBABILITY;
            let is_shielded = !is_machinegunner
                && special_roll < MACHINEGUNNER_PROBABILITY + SHIELDED_PROBABILITY;
            let is_speedster = !is_machinegunner
                && !is_shielded
                && special_roll
                    < MACHINEGUNNER_PROBABILITY + SHIELDED_PROBABILITY + SPEEDSTER_PROBABILITY;
            let is_special = is_machinegunner || is_shielded || is_speedster;
            let is_shooter = !is_special && rng.random::<f32>() < ALIEN_SHOOTER_PROBABILITY;

            let shape_idx = rng.random_range(0..ALIEN_SHAPES.len());
            let mut machinegunner_color: Option<Color> = None;
            let mut speedster_base_color: Option<Color> = None;
            let image_handle = if is_machinegunner {
                let (ca, cb) = pick_complementary_pair(&mut rng);
                let [r, g, b, _] = ca;
                machinegunner_color = Some(Color::srgba(
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    1.0,
                ));
                make_alien_image(
                    images,
                    special_alien_pixel_data(ca, cb, &ALIEN_SHAPES[shape_idx]),
                )
            } else if is_speedster {
                let sp_color = pick_rainbow_color(&mut rng);
                let [r, g, b, _] = sp_color;
                speedster_base_color = Some(Color::srgba(
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    1.0,
                ));
                make_alien_image(
                    images,
                    alien_pixel_data([255, 255, 255, 255], &ALIEN_SHAPES[shape_idx]),
                )
            } else if is_shielded {
                make_alien_image(
                    images,
                    alien_pixel_data_bg(
                        color_bytes,
                        [255, 255, 255, 180],
                        &ALIEN_SHAPES[shape_idx],
                    ),
                )
            } else {
                make_alien_image(
                    images,
                    alien_pixel_data(color_bytes, &ALIEN_SHAPES[shape_idx]),
                )
            };

            let tint = if fade_in {
                Color::WHITE.with_alpha(0.0)
            } else {
                Color::WHITE
            };

            let mut entity = commands.spawn((
                Sprite {
                    image: image_handle,
                    color: tint,
                    custom_size: Some(Vec2::splat(ALIEN_RENDERED_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 1.0),
                Alien {
                    col,
                    row,
                    color: machinegunner_color
                        .or(speedster_base_color)
                        .unwrap_or(alien_color),
                },
            ));

            if fade_in {
                entity.insert(AlphaFadeIn {
                    timer: Timer::from_seconds(ALIEN_FADE_DURATION_SECS, TimerMode::Once),
                });
            }
            if is_machinegunner {
                let idle_interval =
                    rng.random_range(MACHINEGUNNER_IDLE_MIN..MACHINEGUNNER_IDLE_MAX);
                let burst_count =
                    rng.random_range(MACHINEGUNNER_BURST_MIN..=MACHINEGUNNER_BURST_MAX);
                entity.insert(Machinegunner {
                    burst_count,
                    remaining: 0,
                    burst_elapsed: 0.0,
                    idle_elapsed: 0.0,
                    idle_interval,
                });
            }
            if is_shielded {
                let health = rng.random_range(SHIELDED_HEALTH_MIN..=SHIELDED_HEALTH_MAX);
                entity.insert(Shielded { health });
            }
            if is_speedster {
                let multiplier =
                    rng.random_range(SPEEDSTER_MULTIPLIER_MIN..=SPEEDSTER_MULTIPLIER_MAX);
                entity.insert(Speedster {
                    multiplier,
                    base_color: speedster_base_color.unwrap_or(Color::WHITE),
                    flash_elapsed: 0.0,
                });
            }
            if is_shooter {
                let interval = rng.random_range(ALIEN_SHOOT_INTERVAL_MIN..ALIEN_SHOOT_INTERVAL_MAX);
                entity.insert(AlienShooter {
                    timer: Timer::from_seconds(interval, TimerMode::Repeating),
                });
            }
        }
    }
}

pub(crate) fn spawn_explosion(commands: &mut Commands, pos: Vec2, color: Color) {
    let dirs: [Vec2; 8] = [
        Vec2::new(1.0, 0.0),
        Vec2::new(0.707, 0.707),
        Vec2::new(0.0, 1.0),
        Vec2::new(-0.707, 0.707),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.707, -0.707),
        Vec2::new(0.0, -1.0),
        Vec2::new(0.707, -0.707),
    ];
    for dir in dirs {
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::splat(4.0)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 3.0),
            ExplosionParticle {
                velocity: dir * 110.0,
                timer: Timer::from_seconds(0.4, TimerMode::Once),
                lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            },
        ));
    }
}

// /////////////////////////////////////////////////////////////
// MENU
// /////////////////////////////////////////////////////////////

pub(crate) fn on_menu_enter(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::NONE),
            StartButton,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(120.0),
                        border: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BackgroundColor(START_BUTTON_COLOR),
                    StartButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("TRIX BLASTING\n\nPLAY"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                });
        });
}

pub(crate) fn on_menu_exit(
    mut commands: Commands,
    menu_entities: Query<Entity, With<StartButton>>,
) {
    for entity in menu_entities.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    start_btn: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<Phase>>,
) {
    let button_clicked = start_btn.iter().any(|i| *i == Interaction::Pressed);
    if button_clicked || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(Phase::WaveSplash);
    }
}

pub(crate) fn start_button_feedback(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), With<StartButton>>,
) {
    for (interaction, mut color) in &mut buttons {
        *color = match *interaction {
            Interaction::Pressed | Interaction::None => START_BUTTON_COLOR.into(),
            Interaction::Hovered => START_BUTTON_HOVER.into(),
        };
    }
}

// /////////////////////////////////////////////////////////////
// WAVE
// /////////////////////////////////////////////////////////////

pub(crate) fn on_wave_splash_enter(
    mut commands: Commands,
    mut swarm: ResMut<Swarm>,
    mut wave: ResMut<Wave>,
    mut splash_timer: ResMut<WaveSplashTimer>,
    all_ephemeral: WaveTransitionEntities,
    mut images: ResMut<Assets<Image>>,
) {
    splash_timer.0 = Timer::from_seconds(WAVE_SPLASH_DURATION_SECS, TimerMode::Once);
    *swarm = Swarm::new();

    let to_despawn: Vec<Entity> = all_ephemeral.iter().collect();
    for entity in to_despawn {
        commands.entity(entity).despawn();
    }

    let row_count = rows_for_wave(wave.number);
    wave.spawn_count = row_count * ALIEN_COLS;
    spawn_alien_wave(&mut commands, &mut images, &swarm, row_count, true);

    commands.spawn((
        Text2d::new(format!("Wave {}", wave.number)),
        TextFont {
            font_size: 56.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 50.0, 10.0),
        SplashText,
    ));
}

pub(crate) fn tick_wave_splash(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<WaveSplashTimer>,
    mut next_state: ResMut<NextState<Phase>>,
    splash_query: Query<Entity, With<SplashText>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        for entity in splash_query.iter() {
            commands.entity(entity).despawn();
        }
        next_state.set(Phase::Running);
    }
}

pub(crate) fn check_wave_cleared(
    aliens: Query<Entity, With<Alien>>,
    mut speed: ResMut<Speed>,
    mut wave: ResMut<Wave>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<Phase>>,
) {
    if aliens.is_empty() {
        let bonus = wave.spawn_count;
        speed.current = speed_after_wave(speed.current, bonus);
        score.value += bonus as u32;
        wave.number += 1;
        next_state.set(Phase::WaveSplash);
    }
}

pub(crate) fn reset_trix_color(mut trix_query: Query<&mut Sprite, With<Trix>>) {
    if let Ok(mut sprite) = trix_query.single_mut() {
        sprite.color = TRIX_COLOR;
    }
}

pub(crate) fn reset_camera(mut camera: Query<&mut Transform, With<Camera2d>>) {
    if let Ok(mut transform) = camera.single_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }
}

// /////////////////////////////////////////////////////////////
// PLAYER
// /////////////////////////////////////////////////////////////

pub(crate) fn move_trix(
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut trix_query: Query<&mut Transform, With<Trix>>,
    speed: Res<Speed>,
    time: Res<Time>,
) {
    let Ok(mut trix_transform) = trix_query.single_mut() else {
        return;
    };

    let mut direction = 0.0f32;

    if keyboard.pressed(KeyCode::ArrowLeft)
        || keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::KeyQ)
    {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let trix_wx = trix_transform.translation.x + WINDOW_WIDTH / 2.0;
    for touch in touches.iter() {
        let pos = touch.position();
        if pos.y > WINDOW_HEIGHT - 40.0 {
            if pos.x < trix_wx {
                direction -= 1.0;
            } else if pos.x > trix_wx {
                direction += 1.0;
            }
        }
    }

    direction = direction.clamp(-1.0, 1.0);

    if direction == 0.0 {
        return;
    }

    let trix_speed = TRIX_BASE_SPEED + speed.current;
    let left_boundary = -WINDOW_WIDTH / 2.0 + TRIX_RENDERED_SIZE / 2.0;
    let right_boundary = WINDOW_WIDTH / 2.0 - TRIX_RENDERED_SIZE / 2.0;

    trix_transform.translation.x = (trix_transform.translation.x
        + direction * trix_speed * time.delta_secs())
    .clamp(left_boundary, right_boundary);
}

pub(crate) fn handle_trix_shooting(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut commands: Commands,
    trix_query: Query<&Transform, With<Trix>>,
    mut cooldown: ResMut<PlayerShootCooldown>,
    time: Res<Time>,
) {
    cooldown.0 = (cooldown.0 - time.delta_secs()).max(0.0);
    if cooldown.0 > 0.0 {
        return;
    }

    let Ok(trix_transform) = trix_query.single() else {
        return;
    };

    let touch_shoot = touches.iter_just_released().count() > 0;

    let wants_to_shoot = keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touch_shoot;

    if !wants_to_shoot {
        return;
    }

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PLAYER_BULLET_WIDTH, PLAYER_BULLET_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(
            trix_transform.translation.x,
            trix_transform.translation.y + TRIX_RENDERED_SIZE / 2.0 + PLAYER_BULLET_HEIGHT / 2.0,
            2.0,
        ),
        PlayerBullet,
    ));
    cooldown.0 = PLAYER_SHOOT_COOLDOWN_SECS;
}

pub(crate) fn move_player_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<PlayerBullet>>,
    mut speed: ResMut<Speed>,
    time: Res<Time>,
) {
    let bullet_speed = PLAYER_BULLET_BASE_SPEED + speed.current;

    for (entity, mut transform) in bullets.iter_mut() {
        transform.translation.y += bullet_speed * time.delta_secs();

        if transform.translation.y > WINDOW_HEIGHT / 2.0 + PLAYER_BULLET_HEIGHT {
            commands.entity(entity).despawn();
            speed.current = speed_after_miss(speed.current);
            println!("Miss! Speed: {:.1}", speed.current);
        }
    }
}

pub(crate) fn update_cooldown_bar(
    cooldown: Res<PlayerShootCooldown>,
    trix_query: Query<&Transform, With<Trix>>,
    mut bar_query: CooldownBarQuery,
) {
    let fraction_ready = 1.0 - (cooldown.0 / PLAYER_SHOOT_COOLDOWN_SECS).clamp(0.0, 1.0);
    let bar_width = TRIX_RENDERED_SIZE * fraction_ready;

    for trix_transform in trix_query.iter() {
        for (mut bar_transform, mut sprite) in bar_query.iter_mut() {
            bar_transform.translation.x = trix_transform.translation.x;
            bar_transform.translation.y =
                trix_transform.translation.y - TRIX_RENDERED_SIZE / 2.0 - 5.0;
            sprite.custom_size = Some(Vec2::new(bar_width.max(0.01), 3.0));
        }
    }
}

// /////////////////////////////////////////////////////////////
// ALIENS
// /////////////////////////////////////////////////////////////

pub(crate) fn move_swarm(
    time: Res<Time>,
    mut swarm: ResMut<Swarm>,
    speed: Res<Speed>,
    boost: Res<SpeedsterBoost>,
    mut alien_query: Query<(&Alien, &mut Transform)>,
) {
    let mut leftmost_col = ALIEN_COLS;
    let mut rightmost_col = 0usize;
    let mut has_aliens = false;

    for (alien, _) in alien_query.iter() {
        leftmost_col = leftmost_col.min(alien.col);
        rightmost_col = rightmost_col.max(alien.col);
        has_aliens = true;
    }

    if !has_aliens {
        return;
    }

    let rightmost_x = alien_col_x(rightmost_col, swarm.center_x);
    let leftmost_x = alien_col_x(leftmost_col, swarm.center_x);
    let half = ALIEN_RENDERED_SIZE / 2.0;
    let right_wall = WINDOW_WIDTH / 2.0 - WALL_MARGIN;
    let left_wall = -WINDOW_WIDTH / 2.0 + WALL_MARGIN;

    if swarm.direction > 0.0 && rightmost_x + half >= right_wall {
        swarm.direction = -1.0;
        swarm.center_y -= ALIEN_DROP_DISTANCE;
    } else if swarm.direction < 0.0 && leftmost_x - half <= left_wall {
        swarm.direction = 1.0;
        swarm.center_y -= ALIEN_DROP_DISTANCE;
    }

    swarm.center_x += swarm.direction * speed.current * boost.multiplier * time.delta_secs();

    for (alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x = alien_col_x(alien.col, swarm.center_x);
        transform.translation.y = alien_row_y(alien.row, swarm.center_y);
    }
}

pub(crate) fn handle_alien_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut shooter_query: Query<(&Transform, &Alien, &mut AlienShooter)>,
) {
    for (transform, alien, mut shooter) in shooter_query.iter_mut() {
        shooter.timer.tick(time.delta());
        if shooter.timer.just_finished() {
            commands.spawn((
                Sprite {
                    color: brighten_bullet(alien.color),
                    custom_size: Some(Vec2::new(ALIEN_BULLET_WIDTH, ALIEN_BULLET_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y - ALIEN_RENDERED_SIZE / 2.0 - ALIEN_BULLET_HEIGHT / 2.0,
                    2.0,
                ),
                AlienBullet,
            ));
            let new_interval = shooter.timer.duration().as_secs_f32();
            shooter
                .timer
                .set_duration(std::time::Duration::from_secs_f32(new_interval.max(0.2)));
        }
    }
}

pub(crate) fn handle_machinegunner_shooting(
    mut commands: Commands,
    time: Res<Time>,
    speed: Res<Speed>,
    mut query: Query<(&Transform, &Alien, &mut Machinegunner)>,
) {
    let burst_delay = burst_delay_secs(speed.current);
    let dt = time.delta_secs();

    for (transform, alien, mut gunner) in query.iter_mut() {
        if gunner.remaining > 0 {
            gunner.burst_elapsed += dt;
            while gunner.burst_elapsed >= burst_delay && gunner.remaining > 0 {
                gunner.burst_elapsed -= burst_delay;
                gunner.remaining -= 1;
                commands.spawn((
                    Sprite {
                        color: alien.color,
                        custom_size: Some(Vec2::new(ALIEN_BULLET_WIDTH, ALIEN_BULLET_HEIGHT)),
                        ..default()
                    },
                    Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y
                            - ALIEN_RENDERED_SIZE / 2.0
                            - ALIEN_BULLET_HEIGHT / 2.0,
                        2.0,
                    ),
                    AlienBullet,
                ));
            }
        } else {
            gunner.idle_elapsed += dt;
            if gunner.idle_elapsed >= gunner.idle_interval {
                gunner.idle_elapsed = 0.0;
                gunner.burst_elapsed = 0.0;
                gunner.remaining = gunner.burst_count;
            }
        }
    }
}

pub(crate) fn handle_speedsters(
    aliens: Query<&Alien>,
    speedsters: Query<(&Alien, &Speedster)>,
    mut boost: ResMut<SpeedsterBoost>,
) {
    let occupied: Vec<(usize, usize)> = aliens.iter().map(|a| (a.col, a.row)).collect();

    let active_boost = speedsters
        .iter()
        .filter(|(alien, _)| {
            let (c, r) = (alien.col, alien.row);
            let side_neighbors = [(c.wrapping_sub(1), r), (c + 1, r)];
            !side_neighbors.iter().any(|n| occupied.contains(n))
        })
        .map(|(_, s)| s.multiplier)
        .fold(1.0f32, f32::max);

    boost.multiplier = active_boost;
}

pub(crate) fn handle_speedster_flash(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Speedster), Without<AlphaFadeIn>>,
) {
    for (mut sprite, mut speedster) in query.iter_mut() {
        speedster.flash_elapsed += time.delta_secs();
        let flash_on = ((speedster.flash_elapsed * 8.0) as u32).is_multiple_of(2);
        sprite.color = if flash_on {
            speedster.base_color
        } else {
            Color::WHITE
        };
    }
}

pub(crate) fn move_alien_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &Sprite), With<AlienBullet>>,
    speed: Res<Speed>,
    time: Res<Time>,
) {
    let bullet_speed = ALIEN_BULLET_BASE_SPEED + speed.current;

    for (entity, mut transform, sprite) in bullets.iter_mut() {
        transform.translation.y -= bullet_speed * time.delta_secs();

        if transform.translation.y <= BASELINE_Y {
            transform.translation.y = BASELINE_Y;
            let color = sprite.color;
            commands
                .entity(entity)
                .remove::<AlienBullet>()
                .insert(BulletSplash {
                    timer: Timer::from_seconds(0.25, TimerMode::Once),
                })
                .insert(Sprite {
                    color,
                    custom_size: Some(Vec2::new(ALIEN_BULLET_WIDTH, ALIEN_BULLET_HEIGHT)),
                    ..default()
                });
        }
    }
}

pub(crate) fn check_bullet_alien_collisions(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &Sprite), With<PlayerBullet>>,
    aliens: Query<(Entity, &Transform), With<Alien>>,
    mut shielded: Query<&mut Shielded>,
    mut speed: ResMut<Speed>,
    mut score: ResMut<Score>,
) {
    let half_bullet = Vec2::new(PLAYER_BULLET_WIDTH / 2.0, PLAYER_BULLET_HEIGHT / 2.0);
    let half_alien = Vec2::splat(ALIEN_RENDERED_SIZE / 2.0);

    let mut used_bullets = std::collections::HashSet::new();
    let mut used_aliens = std::collections::HashSet::new();
    let mut hit_pairs: Vec<(Entity, Entity, Vec2, Color)> = Vec::new();

    for (bullet_entity, bullet_transform, bullet_sprite) in bullets.iter() {
        if used_bullets.contains(&bullet_entity) {
            continue;
        }
        for (alien_entity, alien_transform) in aliens.iter() {
            if used_aliens.contains(&alien_entity) {
                continue;
            }
            if aabb_overlaps(
                bullet_transform.translation.truncate(),
                half_bullet,
                alien_transform.translation.truncate(),
                half_alien,
            ) {
                used_bullets.insert(bullet_entity);
                used_aliens.insert(alien_entity);
                hit_pairs.push((
                    bullet_entity,
                    alien_entity,
                    alien_transform.translation.truncate(),
                    bullet_sprite.color,
                ));
                break;
            }
        }
    }

    for (bullet_entity, alien_entity, alien_pos, bullet_color) in &hit_pairs {
        commands.entity(*bullet_entity).despawn();
        score.value += 1;
        speed.current = speed_after_hit(speed.current);
        if let Ok(mut s) = shielded.get_mut(*alien_entity) {
            if s.health > 1 {
                s.health -= 1;
            } else {
                spawn_explosion(&mut commands, *alien_pos, *bullet_color);
                commands.entity(*alien_entity).despawn();
            }
        } else {
            spawn_explosion(&mut commands, *alien_pos, *bullet_color);
            commands.entity(*alien_entity).despawn();
        }
    }
}

pub(crate) fn check_game_over_conditions(
    mut commands: Commands,
    aliens: Query<&Transform, With<Alien>>,
    alien_bullets: Query<(Entity, &Transform, &Sprite), With<AlienBullet>>,
    trix_query: Query<&Transform, With<Trix>>,
    mut next_state: ResMut<NextState<Phase>>,
) {
    let Ok(trix_transform) = trix_query.single() else {
        return;
    };

    let trix_pos = trix_transform.translation.truncate();
    let half_trix_shrunk = Vec2::splat(TRIX_RENDERED_SIZE / 2.0 - 1.0);
    let half_alien = Vec2::splat(ALIEN_RENDERED_SIZE / 2.0);
    let half_bullet = Vec2::new(ALIEN_BULLET_WIDTH / 2.0, ALIEN_BULLET_HEIGHT / 2.0);

    for (bullet_entity, bullet_transform, bullet_sprite) in alien_bullets.iter() {
        let bullet_pos = bullet_transform.translation.truncate();
        if aabb_overlaps(trix_pos, half_trix_shrunk, bullet_pos, half_bullet) {
            spawn_explosion(&mut commands, bullet_pos, bullet_sprite.color);
            commands.entity(bullet_entity).despawn();
            next_state.set(Phase::GameOver);
            return;
        }
    }

    for alien_transform in aliens.iter() {
        let alien_pos = alien_transform.translation.truncate();
        if aabb_overlaps(alien_pos, half_alien, trix_pos, half_trix_shrunk) {
            next_state.set(Phase::GameOver);
            return;
        }
        if alien_transform.translation.y - ALIEN_RENDERED_SIZE / 2.0 <= BASELINE_Y {
            next_state.set(Phase::GameOver);
            return;
        }
    }
}

// /////////////////////////////////////////////////////////////
// ANIMATION
// /////////////////////////////////////////////////////////////

pub(crate) fn fade_in_aliens(
    mut commands: Commands,
    mut aliens: Query<(Entity, &mut Sprite, &mut AlphaFadeIn)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut fade) in aliens.iter_mut() {
        fade.timer.tick(time.delta());
        sprite.color = sprite.color.with_alpha(fade.timer.fraction());
        if fade.timer.just_finished() {
            sprite.color = sprite.color.with_alpha(1.0);
            commands.entity(entity).remove::<AlphaFadeIn>();
        }
    }
}

pub(crate) fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut ExplosionParticle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut transform, mut sprite) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.just_finished() {
            commands.entity(entity).despawn();
            continue;
        }
        if particle.timer.is_finished() {
            continue;
        }
        particle.timer.tick(time.delta());
        let t = particle.timer.fraction();
        transform.translation += (particle.velocity * time.delta_secs()).extend(0.0);
        sprite.color = sprite.color.with_alpha(1.0 - t * 0.65);
        sprite.custom_size = Some(Vec2::splat(4.0 * (1.0 - t * 0.5)));
    }
}

pub(crate) fn animate_bullet_splash(
    mut commands: Commands,
    time: Res<Time>,
    mut splashes: Query<(Entity, &mut BulletSplash, &mut Sprite)>,
) {
    for (entity, mut splash, mut sprite) in splashes.iter_mut() {
        splash.timer.tick(time.delta());
        let t = splash.timer.fraction();

        let size = Vec2::new(
            ALIEN_BULLET_WIDTH + t * 14.0,
            ALIEN_BULLET_HEIGHT * (1.0 - t * 0.5),
        );
        sprite.custom_size = Some(size);

        let alpha = 1.0 - t;
        sprite.color = sprite.color.with_alpha(alpha);

        if splash.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// /////////////////////////////////////////////////////////////
// HUD
// /////////////////////////////////////////////////////////////

pub(crate) fn update_score_display(
    score: Res<Score>,
    mut query: Query<&mut Text2d, With<ScoreDisplay>>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("SCORE\n{}", score.value);
    }
}

pub(crate) fn update_wave_display(
    wave: Res<Wave>,
    mut query: Query<&mut Text2d, With<WaveDisplay>>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("WAVE\n{}", wave.number);
    }
}

pub(crate) fn update_speed_display(
    speed: Res<Speed>,
    mut query: Query<&mut Text2d, With<SpeedDisplay>>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("SPEED\n{}", speed.current as u32);
    }
}

// /////////////////////////////////////////////////////////////
// GAME OVER
// /////////////////////////////////////////////////////////////

pub(crate) fn on_game_over_enter(
    mut commands: Commands,
    score: Res<Score>,
    mut shake: ResMut<CameraShake>,
    mut trix_query: Query<(&Transform, &mut Sprite), With<Trix>>,
    mut images: ResMut<Assets<Image>>,
) {
    shake.elapsed = 0.0;

    if let Ok((trix_transform, mut trix_sprite)) = trix_query.single_mut() {
        trix_sprite.color = Color::linear_rgb(0.9, 0.1, 0.1);

        #[rustfmt::skip]
        let skull: [bool; 25] = [
            false, true,  true,  true,  false,
            true,  false, true,  false, true,
            true,  true,  true,  true,  true,
            false, true,  false, true,  false,
            false, false, true,  false, false,
        ];
        let data: Vec<u8> = skull
            .iter()
            .flat_map(|&f| {
                if f {
                    [255u8, 255, 255, 255]
                } else {
                    [0u8, 0, 0, 0]
                }
            })
            .collect();
        let mut img = Image::new(
            Extent3d {
                width: 5,
                height: 5,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        img.sampler = ImageSampler::nearest();
        let skull_handle = images.add(img);

        commands.spawn((
            Sprite {
                image: skull_handle,
                custom_size: Some(Vec2::splat(10.0)),
                ..default()
            },
            Transform::from_xyz(
                trix_transform.translation.x,
                trix_transform.translation.y - TRIX_RENDERED_SIZE / 6.0,
                2.0,
            ),
            GameOverText,
        ));
    }
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::NONE),
            GameOverText,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(120.0),
                        border: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BackgroundColor(RESTART_BUTTON_COLOR),
                    RestartButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(format!("GAME OVER\nScore: {}\n\nRESTART", score.value)),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                });
        });
    println!("Game Over! Score: {}", score.value);
}

pub(crate) fn update_camera_shake(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    if shake.elapsed >= CAMERA_SHAKE_DURATION {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        return;
    }
    shake.elapsed += time.delta_secs();
    let t = shake.elapsed / CAMERA_SHAKE_DURATION;
    let amplitude = CAMERA_SHAKE_AMPLITUDE * (1.0 - t);
    transform.translation.x = amplitude * (shake.elapsed * 47.0).sin();
    transform.translation.y = amplitude * (shake.elapsed * 31.0).cos();
}

pub(crate) fn detect_restart(
    keyboard: Res<ButtonInput<KeyCode>>,
    restart_btn: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut pending: ResMut<RestartPending>,
) {
    let button_pressed = restart_btn.iter().any(|i| *i == Interaction::Pressed);
    if button_pressed
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::KeyR)
    {
        pending.0 = true;
    }
}

pub(crate) fn apply_restart(
    mut pending: ResMut<RestartPending>,
    mut speed: ResMut<Speed>,
    mut wave: ResMut<Wave>,
    mut cooldown: ResMut<PlayerShootCooldown>,
    mut score: ResMut<Score>,
    mut boost: ResMut<SpeedsterBoost>,
    mut next_state: ResMut<NextState<Phase>>,
) {
    if !pending.0 {
        return;
    }
    pending.0 = false;
    speed.current = BASE_GAME_SPEED;
    wave.number = 1;
    wave.spawn_count = ALIEN_COLS;
    cooldown.0 = 0.0;
    score.value = 0;
    boost.multiplier = 1.0;
    next_state.set(Phase::WaveSplash);
}

pub(crate) fn restart_button_feedback(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), With<RestartButton>>,
) {
    for (interaction, mut color) in &mut buttons {
        *color = match *interaction {
            Interaction::Pressed | Interaction::None => RESTART_BUTTON_COLOR.into(),
            Interaction::Hovered => RESTART_BUTTON_HOVER.into(),
        };
    }
}
