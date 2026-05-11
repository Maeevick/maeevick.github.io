use bevy::{prelude::*, window::PresentMode};
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

const ALIEN_COLORS: [Color; 6] = [
    Color::linear_rgb(1.0, 0.2, 0.2),
    Color::linear_rgb(1.0, 0.6, 0.1),
    Color::linear_rgb(0.9, 0.9, 0.1),
    Color::linear_rgb(0.2, 0.9, 0.2),
    Color::linear_rgb(0.1, 0.5, 1.0),
    Color::linear_rgb(0.7, 0.2, 0.9),
];

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

    spawn_alien_wave(&mut commands, &swarm, 1, false);

    println!("Trix Blasting ready.");
}

fn spawn_alien_wave(commands: &mut Commands, swarm: &Swarm, row_count: usize, fade_in: bool) {
    let mut rng = rand::rng();

    for row in 0..row_count {
        for col in 0..ALIEN_COLS {
            let x = alien_col_x(col, swarm.center_x);
            let y = alien_row_y(row, swarm.center_y);
            let color = ALIEN_COLORS[rng.random_range(0..ALIEN_COLORS.len())];
            let is_shooter = rng.random::<f32>() < ALIEN_SHOOTER_PROBABILITY;
            let sprite_color = if fade_in {
                color.with_alpha(0.0)
            } else {
                color
            };

            let mut entity = commands.spawn((
                Sprite {
                    color: sprite_color,
                    custom_size: Some(Vec2::splat(ALIEN_RENDERED_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 1.0),
                Alien { col, row, color },
            ));

            if fade_in {
                entity.insert(AlphaFadeIn {
                    timer: Timer::from_seconds(ALIEN_FADE_DURATION_SECS, TimerMode::Once),
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
) {
    splash_timer.0 = Timer::from_seconds(WAVE_SPLASH_DURATION_SECS, TimerMode::Once);
    *swarm = Swarm::new();

    let to_despawn: Vec<Entity> = all_ephemeral.iter().collect();
    for entity in to_despawn {
        commands.entity(entity).despawn();
    }

    let row_count = rows_for_wave(wave.number);
    wave.spawn_count = row_count * ALIEN_COLS;
    spawn_alien_wave(&mut commands, &swarm, row_count, true);

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
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::KeyR) {
        speed.current = BASE_GAME_SPEED;
        wave.number = 1;
        wave.spawn_count = ALIEN_COLS;
        cooldown.0 = 0.0;
        score.value = 0;
        next_state.set(GameState::WaveSplash);
    }
}

fn check_bullet_alien_collisions(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<PlayerBullet>>,
    aliens: Query<(Entity, &Transform), With<Alien>>,
    mut speed: ResMut<Speed>,
    mut score: ResMut<Score>,
) {
    let half_bullet = Vec2::new(PLAYER_BULLET_WIDTH / 2.0, PLAYER_BULLET_HEIGHT / 2.0);
    let half_alien = Vec2::splat(ALIEN_RENDERED_SIZE / 2.0);

    let mut hit_bullets = std::collections::HashSet::new();
    let mut hit_aliens = std::collections::HashSet::new();

    for (bullet_entity, bullet_transform) in bullets.iter() {
        if hit_bullets.contains(&bullet_entity) {
            continue;
        }
        for (alien_entity, alien_transform) in aliens.iter() {
            if hit_aliens.contains(&alien_entity) {
                continue;
            }
            if aabb_overlaps(
                bullet_transform.translation.truncate(),
                half_bullet,
                alien_transform.translation.truncate(),
                half_alien,
            ) {
                hit_bullets.insert(bullet_entity);
                hit_aliens.insert(alien_entity);
                break;
            }
        }
    }

    let hit_count = hit_aliens.len();
    for entity in hit_bullets.iter().chain(hit_aliens.iter()) {
        commands.entity(*entity).despawn();
    }
    score.value += hit_count as u32;
    for _ in 0..hit_count {
        speed.current = speed_after_hit(speed.current);
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
    mut commands: Commands,
    trix_query: Query<&Transform, With<Trix>>,
    mut cooldown: ResMut<PlayerShootCooldown>,
    time: Res<Time>,
) {
    cooldown.0 = (cooldown.0 - time.delta_secs()).max(0.0);

    if cooldown.0 > 0.0 {
        return;
    }

    let wants_to_shoot =
        keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left);

    if !wants_to_shoot {
        return;
    }

    for trix_transform in trix_query.iter() {
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PLAYER_BULLET_WIDTH, PLAYER_BULLET_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(
                trix_transform.translation.x,
                trix_transform.translation.y
                    + TRIX_RENDERED_SIZE / 2.0
                    + PLAYER_BULLET_HEIGHT / 2.0,
                2.0,
            ),
            PlayerBullet,
        ));
        cooldown.0 = PLAYER_SHOOT_COOLDOWN_SECS;
    }
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

    swarm.center_x += swarm.direction * speed.current * time.delta_secs();

    for (alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x = alien_col_x(alien.col, swarm.center_x);
        transform.translation.y = alien_row_y(alien.row, swarm.center_y);
    }
}

fn move_trix(
    keyboard: Res<ButtonInput<KeyCode>>,
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
}
