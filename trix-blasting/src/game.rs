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
const SPEED_PER_HIT: f32 = 1.0;
const SPEED_PER_PLAYER_MISS: f32 = 5.0;

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
struct AlienBullet;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct CooldownBar;

#[derive(Component)]
struct GameOverText;

#[derive(Resource)]
struct GameSpeed {
    current: f32,
}

impl GameSpeed {
    fn new() -> Self {
        Self {
            current: BASE_GAME_SPEED,
        }
    }
}

#[derive(Resource)]
struct PlayerShootCooldown(f32);

#[derive(Resource)]
struct SwarmState {
    center_x: f32,
    center_y: f32,
    horizontal_direction: f32,
}

impl SwarmState {
    fn new() -> Self {
        Self {
            center_x: 0.0,
            center_y: SWARM_START_Y,
            horizontal_direction: 1.0,
        }
    }
}

type CooldownBarQuery<'w, 's> = Query<
    'w,
    's,
    (&'static mut Transform, &'static mut Sprite),
    (With<CooldownBar>, Without<Trix>),
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
    current + SPEED_PER_HIT
}

pub fn speed_after_miss(current: f32) -> f32 {
    current + SPEED_PER_PLAYER_MISS
}

pub fn aabb_overlaps(pos_a: Vec2, half_a: Vec2, pos_b: Vec2, half_b: Vec2) -> bool {
    (pos_a.x - pos_b.x).abs() < half_a.x + half_b.x
        && (pos_a.y - pos_b.y).abs() < half_a.y + half_b.y
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
    .insert_resource(GameSpeed::new())
    .insert_resource(SwarmState::new())
    .insert_resource(PlayerShootCooldown(0.0))
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
            check_bullet_alien_collisions,
            check_game_over_conditions,
        )
            .run_if(in_state(GameState::Running)),
    )
    .add_systems(OnEnter(GameState::GameOver), on_game_over_enter);
    app
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    swarm: Res<SwarmState>,
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

    spawn_alien_wave(&mut commands, &swarm, 1);

    println!("Trix Blasting ready.");
}

fn spawn_alien_wave(commands: &mut Commands, swarm: &SwarmState, row_count: usize) {
    let mut rng = rand::rng();

    for row in 0..row_count {
        for col in 0..ALIEN_COLS {
            let x = alien_col_x(col, swarm.center_x);
            let y = alien_row_y(row, swarm.center_y);
            let color = ALIEN_COLORS[rng.random_range(0..ALIEN_COLORS.len())];
            let is_shooter = rng.random::<f32>() < ALIEN_SHOOTER_PROBABILITY;

            let mut entity = commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(ALIEN_RENDERED_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 1.0),
                Alien { col, row, color },
            ));

            if is_shooter {
                let interval =
                    rng.random_range(ALIEN_SHOOT_INTERVAL_MIN..ALIEN_SHOOT_INTERVAL_MAX);
                entity.insert(AlienShooter {
                    timer: Timer::from_seconds(interval, TimerMode::Repeating),
                });
            }
        }
    }
}

fn check_bullet_alien_collisions(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<PlayerBullet>>,
    aliens: Query<(Entity, &Transform), With<Alien>>,
    mut game_speed: ResMut<GameSpeed>,
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
    for _ in 0..hit_count {
        game_speed.current = speed_after_hit(game_speed.current);
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

fn on_game_over_enter(mut commands: Commands) {
    commands.spawn((
        Text2d::new("GAME OVER"),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 10.0),
        GameOverText,
    ));
    println!("Game Over!");
}

fn handle_alien_shooting(
    mut commands: Commands,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut shooter_query: Query<(&Transform, &Alien, &mut AlienShooter)>,
) {
    let speed_factor = BASE_GAME_SPEED / game_speed.current;

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
                    transform.translation.y
                        - ALIEN_RENDERED_SIZE / 2.0
                        - ALIEN_BULLET_HEIGHT / 2.0,
                    2.0,
                ),
                AlienBullet,
            ));
            let new_interval = shooter.timer.duration().as_secs_f32() * speed_factor;
            shooter
                .timer
                .set_duration(std::time::Duration::from_secs_f32(new_interval.max(0.2)));
        }
    }
}

fn move_alien_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<AlienBullet>>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    let bullet_speed = ALIEN_BULLET_BASE_SPEED + game_speed.current;

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
                trix_transform.translation.y + TRIX_RENDERED_SIZE / 2.0 + PLAYER_BULLET_HEIGHT / 2.0,
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
    mut game_speed: ResMut<GameSpeed>,
    time: Res<Time>,
) {
    let bullet_speed = PLAYER_BULLET_BASE_SPEED + game_speed.current;

    for (entity, mut transform) in bullets.iter_mut() {
        transform.translation.y += bullet_speed * time.delta_secs();

        if transform.translation.y > WINDOW_HEIGHT / 2.0 + PLAYER_BULLET_HEIGHT {
            commands.entity(entity).despawn();
            game_speed.current = speed_after_miss(game_speed.current);
            println!("Miss! Speed: {:.1}", game_speed.current);
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
    mut swarm: ResMut<SwarmState>,
    game_speed: Res<GameSpeed>,
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

    if swarm.horizontal_direction > 0.0 && rightmost_x + half >= right_wall {
        swarm.horizontal_direction = -1.0;
        swarm.center_y -= ALIEN_DROP_DISTANCE;
    } else if swarm.horizontal_direction < 0.0 && leftmost_x - half <= left_wall {
        swarm.horizontal_direction = 1.0;
        swarm.center_y -= ALIEN_DROP_DISTANCE;
    }

    swarm.center_x += swarm.horizontal_direction * game_speed.current * time.delta_secs();

    for (alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x = alien_col_x(alien.col, swarm.center_x);
        transform.translation.y = alien_row_y(alien.row, swarm.center_y);
    }
}

fn move_trix(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut trix_query: Query<&mut Transform, With<Trix>>,
    game_speed: Res<GameSpeed>,
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

    let trix_speed = TRIX_BASE_SPEED + game_speed.current;
    let left_boundary = -WINDOW_WIDTH / 2.0 + TRIX_RENDERED_SIZE / 2.0;
    let right_boundary = WINDOW_WIDTH / 2.0 - TRIX_RENDERED_SIZE / 2.0;

    for mut transform in trix_query.iter_mut() {
        transform.translation.x =
            (transform.translation.x + direction * trix_speed * time.delta_secs())
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
        assert!((result - 125.0).abs() < 0.01);
    }

    #[test]
    fn given_base_speed_when_bullet_hits_alien_then_speed_increases_by_hit_penalty() {
        let result = speed_after_hit(BASE_GAME_SPEED);
        assert!((result - (BASE_GAME_SPEED + SPEED_PER_HIT)).abs() < 0.01);
    }

    #[test]
    fn given_any_speed_when_bullet_hits_alien_then_speed_increases_by_fixed_penalty() {
        let result = speed_after_hit(120.0);
        assert!((result - 121.0).abs() < 0.01);
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
}
