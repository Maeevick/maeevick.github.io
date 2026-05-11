use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PresentMode,
};
use rand::RngExt;

pub const WINDOW_WIDTH: f32 = 400.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
const WINDOW_WIDTH_PX: u32 = 400;
const WINDOW_HEIGHT_PX: u32 = 800;

const TRIX_RENDERED_SIZE: f32 = 30.0;
const TRIX_COLOR: Color = Color::linear_rgb(0.0, 0.63, 0.87);
const BASELINE_Y: f32 = -WINDOW_HEIGHT / 2.0 + 40.0;
const TRIX_Y: f32 = BASELINE_Y + TRIX_RENDERED_SIZE / 2.0 + 5.0;
const TRIX_BASE_SPEED: f32 = 150.0;

const ALIEN_COLS: usize = 10;
const ALIEN_RENDERED_SIZE: f32 = 25.0;
const ALIEN_GAP: f32 = 5.0;
const ALIEN_DROP_DISTANCE: f32 = ALIEN_RENDERED_SIZE + ALIEN_GAP;
const WALL_MARGIN: f32 = 5.0;
const SWARM_START_Y: f32 = WINDOW_HEIGHT / 2.0 - 60.0;

const BASE_GAME_SPEED: f32 = 50.0;
const SPEED_PER_HIT: f32 = -1.0;
const SPEED_PER_PLAYER_MISS: f32 = 1.0;

const PLAYER_BULLET_WIDTH: f32 = 4.0;
const PLAYER_BULLET_HEIGHT: f32 = 12.0;
const PLAYER_BULLET_BASE_SPEED: f32 = 200.0;
const PLAYER_SHOOT_COOLDOWN_SECS: f32 = 0.25;

const ALIEN_BULLET_WIDTH: f32 = 4.0;
const ALIEN_BULLET_HEIGHT: f32 = 10.0;
const ALIEN_BULLET_BASE_SPEED: f32 = 150.0;
const ALIEN_SHOOT_INTERVAL_MIN: f32 = 1.5;
const ALIEN_SHOOT_INTERVAL_MAX: f32 = 3.0;
const ALIEN_SHOOTER_PROBABILITY: f32 = 0.3;

const WAVE_SPLASH_DURATION_SECS: f32 = 1.5;
const ALIEN_FADE_DURATION_SECS: f32 = 0.3;

const MACHINEGUNNER_PROBABILITY: f32 = 0.12;
const MACHINEGUNNER_BURST_MIN: u8 = 3;
const MACHINEGUNNER_BURST_MAX: u8 = 8;
const MACHINEGUNNER_IDLE_MIN: f32 = 2.0;
const MACHINEGUNNER_IDLE_MAX: f32 = 5.0;

const SHIELDED_PROBABILITY: f32 = 0.10;
const SHIELDED_HEALTH_MIN: u8 = 3;
const SHIELDED_HEALTH_MAX: u8 = 8;

const SPEEDSTER_PROBABILITY: f32 = 0.08;
const SPEEDSTER_MULTIPLIER_MIN: f32 = 1.2;
const SPEEDSTER_MULTIPLIER_MAX: f32 = 2.0;
const SPEEDSTER_ACTIVATION_THRESHOLD: usize = 5;


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Running,
    WaveSplash,
    GameOver,
}

#[derive(Component)]
struct Trix;

#[derive(Component)]
struct Alien {
    col: usize,
    row: usize,
    color: Color,
}

#[derive(Component)]
struct AlienShooter {
    timer: Timer,
}

#[derive(Component)]
struct AlphaFadeIn {
    timer: Timer,
}

#[derive(Component)]
struct AlienBullet;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct CooldownBar;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct SpeedDisplay;

#[derive(Resource, Default)]
struct Score {
    value: u32,
}

#[derive(Component)]
struct WaveDisplay;

#[derive(Component)]
struct SplashText;

#[derive(Resource)]
struct Speed {
    current: f32,
}

impl Speed {
    fn new() -> Self {
        Self {
            current: BASE_GAME_SPEED,
        }
    }
}

#[derive(Resource)]
struct PlayerShootCooldown(f32);

#[derive(Resource)]
struct Swarm {
    center_x: f32,
    center_y: f32,
    direction: f32,
}

impl Swarm {
    fn new() -> Self {
        Self {
            center_x: 0.0,
            center_y: SWARM_START_Y,
            direction: 1.0,
        }
    }
}

#[derive(Resource)]
struct Wave {
    number: u32,
    spawn_count: usize,
}

#[derive(Resource)]
struct WaveSplashTimer(Timer);

#[derive(Component)]
struct Machinegunner {
    burst_count: u8,
    remaining: u8,
    burst_elapsed: f32,
    idle_elapsed: f32,
    idle_interval: f32,
}

#[derive(Component)]
struct Shielded {
    health: u8,
}

#[derive(Component)]
struct Speedster {
    multiplier: f32,
    base_color: Color,
    flash_elapsed: f32,
}

#[derive(Resource)]
struct SpeedsterBoost {
    multiplier: f32,
}

impl SpeedsterBoost {
    fn new() -> Self { Self { multiplier: 1.0 } }
}

type CooldownBarQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Transform, &'static mut Sprite),
    (With<CooldownBar>, Without<Trix>),
>;

type WaveTransitionEntities<'w, 's> = Query<
    'w,
    's,
    Entity,
    Or<(
        With<PlayerBullet>,
        With<AlienBullet>,
        With<GameOverText>,
        With<Alien>,
    )>,
>;

pub fn alien_col_x(col: usize, swarm_center_x: f32) -> f32 {
    let total_grid_width =
        ALIEN_COLS as f32 * ALIEN_RENDERED_SIZE + (ALIEN_COLS - 1) as f32 * ALIEN_GAP;
    swarm_center_x + col as f32 * (ALIEN_RENDERED_SIZE + ALIEN_GAP) - total_grid_width / 2.0
        + ALIEN_RENDERED_SIZE / 2.0
}

pub fn alien_row_y(row: usize, swarm_center_y: f32) -> f32 {
    swarm_center_y - row as f32 * ALIEN_DROP_DISTANCE
}

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

pub fn aabb_overlaps(pos_a: Vec2, half_a: Vec2, pos_b: Vec2, half_b: Vec2) -> bool {
    (pos_a.x - pos_b.x).abs() < half_a.x + half_b.x
        && (pos_a.y - pos_b.y).abs() < half_a.y + half_b.y
}

pub fn rows_for_wave_formula(wave: u32, rand_factor: f32) -> usize {
    match wave {
        1 => 1,
        2 => 2,
        3 => 3,
        n => (((n - 3) as f32 * rand_factor).floor() as usize).clamp(1, 12),
    }
}

fn rows_for_wave(wave: u32) -> usize {
    if wave < 4 {
        return wave as usize;
    }
    let rand_factor: f32 = rand::rng().random_range(0.0..1.0);
    rows_for_wave_formula(wave, rand_factor)
}

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

pub fn pick_rainbow_color(rng: &mut impl rand::RngExt) -> [u8; 4] {
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

pub fn pick_complementary_pair(rng: &mut impl rand::RngExt) -> ([u8; 4], [u8; 4]) {
    let hue: f32 = rng.random_range(0.0..360.0);
    let brightness: f32 = rng.random_range(0.7..1.0);
    let [r1, g1, b1] = hue_to_rgb(hue);
    let [r2, g2, b2] = hue_to_rgb((hue + 180.0).rem_euclid(360.0));
    (
        [(r1 * brightness * 255.0) as u8, (g1 * brightness * 255.0) as u8, (b1 * brightness * 255.0) as u8, 255],
        [(r2 * brightness * 255.0) as u8, (g2 * brightness * 255.0) as u8, (b2 * brightness * 255.0) as u8, 255],
    )
}

const ALIEN_SHAPES: [[bool; 25]; 5] = [
    // crab
    [
        false, true,  false, true,  false,
        true,  true,  true,  true,  true,
        true,  true,  false, true,  true,
        false, true,  true,  true,  false,
        true,  false, false, false, true,
    ],
    // squid
    [
        false, false, true,  false, false,
        false, true,  true,  true,  false,
        true,  true,  false, true,  true,
        true,  false, true,  false, true,
        false, true,  false, true,  false,
    ],
    // octopus
    [
        false, true,  true,  true,  false,
        true,  true,  true,  true,  true,
        true,  false, true,  false, true,
        true,  true,  true,  true,  true,
        false, true,  false, true,  false,
    ],
    // bat
    [
        true,  false, false, false, true,
        true,  true,  false, true,  true,
        true,  true,  true,  true,  true,
        false, false, true,  false, false,
        false, true,  true,  true,  false,
    ],
    // star
    [
        true,  false, true,  false, true,
        false, true,  true,  true,  false,
        true,  true,  false, true,  true,
        false, true,  true,  true,  false,
        true,  false, true,  false, true,
    ],
];

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

fn make_alien_image(images: &mut Assets<Image>, pixel_data: Vec<u8>) -> Handle<Image> {
    let mut image = Image::new(
        Extent3d { width: 5, height: 5, depth_or_array_layers: 1 },
        TextureDimension::D2,
        pixel_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    images.add(image)
}

pub fn create_app(for_wasm: bool) -> App {
    let window = if for_wasm {
        Window {
            title: "Trix Blasting".into(),
            resolution: (WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX).into(),
            canvas: Some("#game-canvas".to_string()),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }
    } else {
        Window {
            title: "Trix Blasting".into(),
            resolution: (WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX).into(),
            ..default()
        }
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }))
    .insert_resource(ClearColor(Color::linear_rgb(0.05, 0.05, 0.1)))
    .insert_resource(Speed::new())
    .insert_resource(Swarm::new())
    .insert_resource(PlayerShootCooldown(0.0))
    .insert_resource(Wave {
        number: 1,
        spawn_count: ALIEN_COLS,
    })
    .insert_resource(Score::default())
    .insert_resource(SpeedsterBoost::new())
    .insert_resource(WaveSplashTimer(Timer::from_seconds(
        WAVE_SPLASH_DURATION_SECS,
        TimerMode::Once,
    )))
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            move_trix,
            handle_trix_shooting,
            move_player_bullets,
            update_cooldown_bar,
            move_swarm,
            handle_alien_shooting,
            handle_machinegunner_shooting,
            handle_speedsters,
            handle_speedster_flash,
            move_alien_bullets,
            fade_in_aliens,
            check_bullet_alien_collisions,
            check_game_over_conditions,
            check_wave_cleared,
            update_score_display,
            update_wave_display,
            update_speed_display,
        )
            .run_if(in_state(GameState::Running)),
    )
    .add_systems(
        Update,
        (
            tick_wave_splash,
            fade_in_aliens,
            update_score_display,
            update_wave_display,
            update_speed_display,
        )
            .run_if(in_state(GameState::WaveSplash)),
    )
    .add_systems(Update, handle_restart.run_if(in_state(GameState::GameOver)))
    .add_systems(OnEnter(GameState::WaveSplash), on_wave_splash_enter)
    .add_systems(OnEnter(GameState::GameOver), on_game_over_enter);
    app
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    swarm: Res<Swarm>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, 3.0)),
            ..default()
        },
        Transform::from_xyz(0.0, BASELINE_Y, 0.0),
    ));

    let triangle = Triangle2d::new(
        Vec2::new(0.0, TRIX_RENDERED_SIZE / 2.0),
        Vec2::new(-TRIX_RENDERED_SIZE / 2.0, -TRIX_RENDERED_SIZE / 2.0),
        Vec2::new(TRIX_RENDERED_SIZE / 2.0, -TRIX_RENDERED_SIZE / 2.0),
    );
    commands.spawn((
        Mesh2d(meshes.add(triangle)),
        MeshMaterial2d(materials.add(TRIX_COLOR)),
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

    spawn_alien_wave(&mut commands, &mut images, &swarm, 1, false);

    println!("Trix Blasting ready.");
}

fn spawn_alien_wave(
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
                && special_roll < MACHINEGUNNER_PROBABILITY + SHIELDED_PROBABILITY + SPEEDSTER_PROBABILITY;
            let is_special = is_machinegunner || is_shielded || is_speedster;
            let is_shooter = !is_special && rng.random::<f32>() < ALIEN_SHOOTER_PROBABILITY;

            let shape_idx = rng.random_range(0..ALIEN_SHAPES.len());
            let mut speedster_base_color: Option<Color> = None;
            let image_handle = if is_machinegunner {
                let (ca, cb) = pick_complementary_pair(&mut rng);
                make_alien_image(images, special_alien_pixel_data(ca, cb, &ALIEN_SHAPES[shape_idx]))
            } else if is_speedster {
                let sp_color = pick_rainbow_color(&mut rng);
                let [r, g, b, _] = sp_color;
                speedster_base_color = Some(Color::srgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0));
                make_alien_image(images, alien_pixel_data([255, 255, 255, 255], &ALIEN_SHAPES[shape_idx]))
            } else if is_shielded {
                make_alien_image(images, alien_pixel_data_bg(color_bytes, [255, 255, 255, 180], &ALIEN_SHAPES[shape_idx]))
            } else {
                make_alien_image(images, alien_pixel_data(color_bytes, &ALIEN_SHAPES[shape_idx]))
            };

            let tint = if fade_in { Color::WHITE.with_alpha(0.0) } else { Color::WHITE };

            let mut entity = commands.spawn((
                Sprite {
                    image: image_handle,
                    color: tint,
                    custom_size: Some(Vec2::splat(ALIEN_RENDERED_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 1.0),
                Alien { col, row, color: alien_color },
            ));

            if fade_in {
                entity.insert(AlphaFadeIn {
                    timer: Timer::from_seconds(ALIEN_FADE_DURATION_SECS, TimerMode::Once),
                });
            }
            if is_machinegunner {
                let idle_interval =
                    rng.random_range(MACHINEGUNNER_IDLE_MIN..MACHINEGUNNER_IDLE_MAX);
                let burst_count = rng.random_range(MACHINEGUNNER_BURST_MIN..=MACHINEGUNNER_BURST_MAX);
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

fn check_wave_cleared(
    aliens: Query<Entity, With<Alien>>,
    mut speed: ResMut<Speed>,
    mut wave: ResMut<Wave>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if aliens.is_empty() {
        let bonus = wave.spawn_count;
        speed.current = speed_after_wave(speed.current, bonus);
        score.value += bonus as u32;
        wave.number += 1;
        next_state.set(GameState::WaveSplash);
    }
}

fn on_wave_splash_enter(
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

fn tick_wave_splash(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<WaveSplashTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    splash_query: Query<Entity, With<SplashText>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        for entity in splash_query.iter() {
            commands.entity(entity).despawn();
        }
        next_state.set(GameState::Running);
    }
}

fn fade_in_aliens(
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

fn update_score_display(score: Res<Score>, mut query: Query<&mut Text2d, With<ScoreDisplay>>) {
    for mut text in query.iter_mut() {
        text.0 = format!("SCORE\n{}", score.value);
    }
}

fn update_wave_display(wave: Res<Wave>, mut query: Query<&mut Text2d, With<WaveDisplay>>) {
    for mut text in query.iter_mut() {
        text.0 = format!("WAVE\n{}", wave.number);
    }
}

fn update_speed_display(speed: Res<Speed>, mut query: Query<&mut Text2d, With<SpeedDisplay>>) {
    for mut text in query.iter_mut() {
        text.0 = format!("SPEED\n{}", speed.current as u32);
    }
}

fn on_game_over_enter(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        Text2d::new(format!(
            "GAME OVER\nScore: {}\n\nENTER to restart",
            score.value
        )),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 10.0),
        GameOverText,
    ));
    println!("Game Over! Score: {}", score.value);
}

fn handle_restart(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut speed: ResMut<Speed>,
    mut wave: ResMut<Wave>,
    mut cooldown: ResMut<PlayerShootCooldown>,
    mut score: ResMut<Score>,
    mut boost: ResMut<SpeedsterBoost>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::KeyR) {
        speed.current = BASE_GAME_SPEED;
        wave.number = 1;
        wave.spawn_count = ALIEN_COLS;
        cooldown.0 = 0.0;
        score.value = 0;
        boost.multiplier = 1.0;
        next_state.set(GameState::WaveSplash);
    }
}

fn check_bullet_alien_collisions(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<PlayerBullet>>,
    aliens: Query<(Entity, &Transform), With<Alien>>,
    mut shielded: Query<&mut Shielded>,
    mut speed: ResMut<Speed>,
    mut score: ResMut<Score>,
) {
    let half_bullet = Vec2::new(PLAYER_BULLET_WIDTH / 2.0, PLAYER_BULLET_HEIGHT / 2.0);
    let half_alien = Vec2::splat(ALIEN_RENDERED_SIZE / 2.0);

    let mut used_bullets = std::collections::HashSet::new();
    let mut used_aliens = std::collections::HashSet::new();
    let mut hit_pairs: Vec<(Entity, Entity)> = Vec::new();

    for (bullet_entity, bullet_transform) in bullets.iter() {
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
                hit_pairs.push((bullet_entity, alien_entity));
                break;
            }
        }
    }

    for (bullet_entity, alien_entity) in &hit_pairs {
        commands.entity(*bullet_entity).despawn();
        score.value += 1;
        speed.current = speed_after_hit(speed.current);
        if let Ok(mut s) = shielded.get_mut(*alien_entity) {
            if s.health > 1 {
                s.health -= 1;
            } else {
                commands.entity(*alien_entity).despawn();
            }
        } else {
            commands.entity(*alien_entity).despawn();
        }
    }
}

fn check_game_over_conditions(
    aliens: Query<&Transform, With<Alien>>,
    alien_bullets: Query<&Transform, With<AlienBullet>>,
    trix_query: Query<&Transform, With<Trix>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(trix_transform) = trix_query.single() else {
        return;
    };

    let trix_pos = trix_transform.translation.truncate();
    let half_trix = Vec2::splat(TRIX_RENDERED_SIZE / 2.0);
    let half_alien_bullet = Vec2::new(ALIEN_BULLET_WIDTH / 2.0, ALIEN_BULLET_HEIGHT / 2.0);
    let half_alien = Vec2::splat(ALIEN_RENDERED_SIZE / 2.0);

    for bullet_transform in alien_bullets.iter() {
        if aabb_overlaps(
            bullet_transform.translation.truncate(),
            half_alien_bullet,
            trix_pos,
            half_trix,
        ) {
            next_state.set(GameState::GameOver);
            return;
        }
    }

    for alien_transform in aliens.iter() {
        let alien_pos = alien_transform.translation.truncate();
        if aabb_overlaps(alien_pos, half_alien, trix_pos, half_trix) {
            next_state.set(GameState::GameOver);
            return;
        }
        if alien_transform.translation.y - ALIEN_RENDERED_SIZE / 2.0 <= BASELINE_Y {
            next_state.set(GameState::GameOver);
            return;
        }
    }
}

fn handle_alien_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut shooter_query: Query<(&Transform, &Alien, &mut AlienShooter)>,
) {
    for (transform, alien, mut shooter) in shooter_query.iter_mut() {
        shooter.timer.tick(time.delta());
        if shooter.timer.just_finished() {
            commands.spawn((
                Sprite {
                    color: alien.color,
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

fn handle_machinegunner_shooting(
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

fn handle_speedsters(
    aliens: Query<Entity, With<Alien>>,
    speedsters: Query<&Speedster>,
    mut boost: ResMut<SpeedsterBoost>,
) {
    if aliens.iter().count() <= SPEEDSTER_ACTIVATION_THRESHOLD && !speedsters.is_empty() {
        boost.multiplier = speedsters
            .iter()
            .map(|s| s.multiplier)
            .fold(1.0f32, f32::max);
    } else {
        boost.multiplier = 1.0;
    }
}

fn handle_speedster_flash(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Speedster), Without<AlphaFadeIn>>,
) {
    for (mut sprite, mut speedster) in query.iter_mut() {
        speedster.flash_elapsed += time.delta_secs();
        let flash_on = ((speedster.flash_elapsed * 8.0) as u32).is_multiple_of(2);
        sprite.color = if flash_on { speedster.base_color } else { Color::WHITE };
    }
}

fn move_alien_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<AlienBullet>>,
    speed: Res<Speed>,
    time: Res<Time>,
) {
    let bullet_speed = ALIEN_BULLET_BASE_SPEED + speed.current;

    for (entity, mut transform) in bullets.iter_mut() {
        transform.translation.y -= bullet_speed * time.delta_secs();

        if transform.translation.y < -WINDOW_HEIGHT / 2.0 - ALIEN_BULLET_HEIGHT {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_trix_shooting(
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

    let trix_wx = trix_transform.translation.x + WINDOW_WIDTH / 2.0;
    let trix_wy = WINDOW_HEIGHT / 2.0 - trix_transform.translation.y;
    let half = TRIX_RENDERED_SIZE / 2.0;

    let touch_shoot = touches.iter_just_pressed().any(|touch| {
        let pos = touch.position();
        if pos.y > WINDOW_HEIGHT - 40.0
            && pos.x >= WINDOW_WIDTH / 3.0
            && pos.x <= 2.0 * WINDOW_WIDTH / 3.0
        {
            return true;
        }
        (pos.x - trix_wx).abs() < half && (pos.y - trix_wy).abs() < half
    });

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

fn move_player_bullets(
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

fn update_cooldown_bar(
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

fn move_swarm(
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

fn move_trix(
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut trix_query: Query<&mut Transform, With<Trix>>,
    speed: Res<Speed>,
    time: Res<Time>,
) {
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

    for touch in touches.iter() {
        let pos = touch.position();
        if pos.y > WINDOW_HEIGHT - 40.0 {
            if pos.x < WINDOW_WIDTH / 3.0 {
                direction -= 1.0;
            } else if pos.x > 2.0 * WINDOW_WIDTH / 3.0 {
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

    for mut transform in trix_query.iter_mut() {
        transform.translation.x = (transform.translation.x
            + direction * trix_speed * time.delta_secs())
        .clamp(left_boundary, right_boundary);
    }
}

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
