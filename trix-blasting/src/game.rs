use bevy::{prelude::*, window::PresentMode};

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
const SPEED_PER_PLAYER_MISS: f32 = 5.0;

const PLAYER_BULLET_WIDTH: f32 = 4.0;
const PLAYER_BULLET_HEIGHT: f32 = 12.0;
const PLAYER_BULLET_BASE_SPEED: f32 = 200.0;
const PLAYER_SHOOT_COOLDOWN_SECS: f32 = 0.25;

#[derive(Component)]
struct Trix;

#[derive(Component)]
struct Alien {
    col: usize,
    row: usize,
}

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct CooldownBar;

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

pub fn alien_col_x(col: usize, swarm_center_x: f32) -> f32 {
    let total_grid_width =
        ALIEN_COLS as f32 * ALIEN_RENDERED_SIZE + (ALIEN_COLS - 1) as f32 * ALIEN_GAP;
    swarm_center_x + col as f32 * (ALIEN_RENDERED_SIZE + ALIEN_GAP) - total_grid_width / 2.0
        + ALIEN_RENDERED_SIZE / 2.0
}

pub fn alien_row_y(row: usize, swarm_center_y: f32) -> f32 {
    swarm_center_y - row as f32 * ALIEN_DROP_DISTANCE
}

pub fn speed_after_miss(current: f32) -> f32 {
    current + SPEED_PER_PLAYER_MISS
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
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            move_trix,
            handle_trix_shooting,
            move_player_bullets,
            update_cooldown_bar,
            move_swarm,
        ),
    );
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
    for row in 0..row_count {
        for col in 0..ALIEN_COLS {
            let x = alien_col_x(col, swarm.center_x);
            let y = alien_row_y(row, swarm.center_y);
            commands.spawn((
                Sprite {
                    color: Color::linear_rgb(0.8, 0.3, 0.1),
                    custom_size: Some(Vec2::splat(ALIEN_RENDERED_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 1.0),
                Alien { col, row },
            ));
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
    mut bar_query: Query<(&mut Transform, &mut Sprite), (With<CooldownBar>, Without<Trix>)>,
) {
    let fraction_ready = 1.0 - (cooldown.0 / PLAYER_SHOOT_COOLDOWN_SECS).clamp(0.0, 1.0);
    let bar_width = TRIX_RENDERED_SIZE * fraction_ready;

    for trix_transform in trix_query.iter() {
        for (mut bar_transform, mut sprite) in bar_query.iter_mut() {
            bar_transform.translation.x = trix_transform.translation.x;
            bar_transform.translation.y = trix_transform.translation.y - TRIX_RENDERED_SIZE / 2.0 - 5.0;
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
}
