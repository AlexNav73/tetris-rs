use bevy::{color::palettes::css::*, prelude::*};

const VCELL_COUNT: f32 = 21.0;
const HCELL_COUNT: f32 = 10.0;
const CELL_SIZE: f32 = 20.0;
const BORDER_SIZE: f32 = 5.0;
const BITS_PER_CELL: usize = 3;
const CELL_BIT_MASK: u32 = 0b111;

const CELL_CENTER: f32 = CELL_SIZE / 2.0;
const FIELD_WIDTH: f32 = HCELL_COUNT * CELL_SIZE;
const FIELD_HEIGHT: f32 = VCELL_COUNT * CELL_SIZE;

const V_DIST_FROM_CENTER: f32 = FIELD_HEIGHT / 2.0;

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_camera, spawn_field, spawn_initial_tetrimino),
        )
        .add_systems(
            Update,
            (
                handle_exit_key_pressed,
                tetrimino_fall,
                move_sideways,
                show_tetrinino_debug_view,
                update_speed,
            ),
        )
        .run();
}

#[derive(Default)]
struct Row(u32);

impl Row {
    fn set(&mut self, mask: u32) {
        self.0 |= mask;
    }

    fn can_move(&self, mask: u32) -> bool {
        (self.0 & mask) == 0
    }
}

#[derive(Resource)]
struct GameState {
    rows: [Row; VCELL_COUNT as usize],
    speed: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            rows: [const { Row(0) }; VCELL_COUNT as usize],
            speed: 50.0,
        }
    }
}

impl GameState {
    fn set(&mut self, tetrimino: &Tetrimino) {
        let row = &mut self.rows[tetrimino.row];
        row.set(tetrimino.mask);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Clone, Copy, Component)]
struct Active;

#[derive(Component)]
struct Tetrimino {
    mask: u32,
    row: usize,
    column: usize,
}

impl Tetrimino {
    fn new(column: usize) -> Self {
        Self {
            mask: CELL_BIT_MASK << ((HCELL_COUNT as usize - column) * BITS_PER_CELL),
            row: 0,
            column,
        }
    }

    fn can_move(&self, row: &Row) -> bool {
        row.can_move(self.mask)
    }

    fn x(&self) -> f32 {
        (self.column as f32 * CELL_SIZE) - (FIELD_WIDTH / 2.0) + CELL_CENTER
    }

    fn y(&self) -> f32 {
        V_DIST_FROM_CENTER - (self.row as f32 * CELL_SIZE)
    }

    fn move_left(&mut self, row: &Row) {
        let new_mask = self.mask << BITS_PER_CELL;

        if !row.can_move(new_mask) {
            return;
        }

        if let Some(c) = self.column.checked_sub(1) {
            self.column = c;
            self.mask = new_mask;
        }
    }

    fn move_right(&mut self, row: &Row) {
        let new_mask = self.mask >> BITS_PER_CELL;

        if !row.can_move(new_mask) {
            return;
        }

        if self.column + 1 < HCELL_COUNT as usize {
            self.column += 1;
            self.mask = new_mask;
        }
    }
}

fn spawn_tetrimino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let tetrimino = Tetrimino::new(5);

    let vertical = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let color = materials.add(Color::srgb(1.0, 0.0, 0.0));

    let x = tetrimino.x();

    commands
        .spawn((
            tetrimino,
            Active,
            Mesh2d(vertical),
            MeshMaterial2d(color),
            Transform::from_xyz(x, V_DIST_FROM_CENTER, 1.0),
        ))
        .observe(on_tetrimino_stopped);
}

fn spawn_initial_tetrimino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_tetrimino(&mut commands, &mut meshes, &mut materials);
}

#[derive(Event)]
struct TetriminoStopped;

fn on_tetrimino_stopped(
    trigger: Trigger<TetriminoStopped>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = trigger.entity();

    commands.entity(entity).remove::<Active>();

    spawn_tetrimino(&mut commands, &mut meshes, &mut materials);
}

fn tetrimino_fall(
    mut commands: Commands,
    mut tetriminos: Query<(Entity, &mut Transform, &mut Tetrimino), With<Active>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    if let Ok((entity, mut pos, mut tetrimino)) = tetriminos.get_single_mut() {
        let new_y = pos.translation.y - (time.delta_secs_f64() as f32 * game_state.speed);
        let translated_y = new_y + V_DIST_FROM_CENTER;
        let row_idx = ((FIELD_HEIGHT - translated_y) / CELL_SIZE).ceil() as usize;

        let mut can_move_down = false;
        if row_idx < VCELL_COUNT as usize {
            let row_to_check = &game_state.rows[row_idx];

            if tetrimino.can_move(row_to_check) {
                pos.translation.y = new_y;
                tetrimino.row = row_idx;
                can_move_down = true;
            }
        }

        if !can_move_down {
            tetrimino.row = row_idx - 1;
            pos.translation.y = tetrimino.y();

            game_state.set(&tetrimino);
            commands.trigger_targets(TetriminoStopped, entity);
        }
    }
}

fn move_sideways(
    mut tetriminos: Query<(&mut Transform, &mut Tetrimino), With<Active>>,
    key: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
) {
    if let Ok((mut pos, mut tetrimino)) = tetriminos.get_single_mut() {
        let row = &game_state.rows[tetrimino.row];

        if key.just_released(KeyCode::KeyA) {
            tetrimino.move_left(row);
        } else if key.just_pressed(KeyCode::KeyD) {
            tetrimino.move_right(row);
        }

        if tetrimino.can_move(row) {
            pos.translation.x = tetrimino.x();
        }
    }
}

fn show_tetrinino_debug_view(
    tetriminos: Query<&Tetrimino, With<Active>>,
    mut gizmos: Gizmos,
    key: Res<ButtonInput<KeyCode>>,
) {
    if !key.pressed(KeyCode::KeyE) {
        return;
    }

    let tetrimino = tetriminos.single();

    gizmos.circle_2d(Isometry2d::IDENTITY, 1.0, GRAY);

    gizmos.rect_2d(
        Isometry2d::from_translation(Vec2::new(tetrimino.x(), tetrimino.y())),
        Vec2::new(CELL_SIZE, CELL_SIZE),
        Color::srgb(0.0, 0.0, 1.0),
    );

    gizmos.grid_2d(
        Isometry2d::from_translation(Vec2::new(0.0, CELL_CENTER)),
        UVec2::new(HCELL_COUNT as u32, VCELL_COUNT as u32),
        Vec2::new(CELL_SIZE, CELL_SIZE),
        Color::srgb(0.2, 0.2, 0.2),
    );
}

fn update_speed(key: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<GameState>) {
    if key.pressed(KeyCode::ArrowUp) {
        game_state.speed += 3.0;
    } else if key.pressed(KeyCode::ArrowDown) && game_state.speed - 3.0 > 0.0 {
        game_state.speed -= 3.0;
    }
}

#[derive(Component)]
struct Field;

fn spawn_field(
    mut commans: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let h_dist_from_center = FIELD_WIDTH / 2.0;
    let v_dist_from_center = (VCELL_COUNT / 2.0).ceil() * CELL_SIZE;
    let border_center = BORDER_SIZE / 2.0;

    let vertical = meshes.add(Rectangle::new(BORDER_SIZE, FIELD_HEIGHT));
    let horizontal = meshes.add(Rectangle::new(
        FIELD_WIDTH + (BORDER_SIZE * 2.0),
        BORDER_SIZE,
    ));
    let color = materials.add(Color::WHITE);

    // left
    commans.spawn((
        Field,
        Mesh2d(vertical.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(-h_dist_from_center - border_center, CELL_CENTER, 0.0),
    ));
    // right
    commans.spawn((
        Field,
        Mesh2d(vertical.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(h_dist_from_center + border_center, CELL_CENTER, 0.0),
    ));
    // top
    commans.spawn((
        Field,
        Mesh2d(horizontal.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(0.0, v_dist_from_center + border_center, 0.0),
    ));
    // bottom
    commans.spawn((
        Field,
        Mesh2d(horizontal),
        MeshMaterial2d(color),
        Transform::from_xyz(
            0.0,
            -(FIELD_HEIGHT - v_dist_from_center) - border_center,
            0.0,
        ),
    ));
}

fn handle_exit_key_pressed(
    key: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if key.just_pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
