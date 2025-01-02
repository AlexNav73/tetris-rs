use bevy::log::prelude::*;
use bevy::prelude::*;

const VCELL_COUNT: f32 = 21.0;
const HCELL_COUNT: f32 = 10.0;
const CELL_SIZE: f32 = 20.0;
const BORDER_SIZE: f32 = 5.0;
const SPEED: f32 = 40.0;

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_field))
        .add_systems(
            Update,
            (handle_exit_key_pressed, spawn_tetrimino, tetrimino_fall),
        )
        .add_observer(on_tetrimino_stopped)
        .run();
}

#[derive(Resource, Default)]
struct GameState {
    rows: [u32; VCELL_COUNT as usize],
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Clone, Copy, Component)]
struct Active;

#[derive(Clone, Copy, Component)]
struct Tetrimino(u32);

fn create_tetrimino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let v_dist_from_center = VCELL_COUNT * CELL_SIZE / 2.0;

    let tetrimino = Tetrimino(123);

    let vertical = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let color = materials.add(Color::srgb(1.0, 0.0, 0.0));

    commands.spawn((
        tetrimino,
        Active,
        Mesh2d(vertical),
        MeshMaterial2d(color),
        Transform::from_xyz(0.0, v_dist_from_center - (CELL_SIZE / 2.0), 1.0),
    ));
}

fn spawn_tetrimino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if key.just_pressed(KeyCode::KeyW) {
        create_tetrimino(&mut commands, &mut meshes, &mut materials);
    }
}

#[derive(Event)]
struct TetriminoStopped;

fn on_tetrimino_stopped(
    _trigger: Trigger<TetriminoStopped>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    create_tetrimino(&mut commands, &mut meshes, &mut materials);
}

fn tetrimino_fall(
    mut commans: Commands,
    mut tetriminos: Query<(Entity, &mut Transform, &Tetrimino), With<Active>>,
    game_state: Res<GameState>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if !key.pressed(KeyCode::Space) {
        return;
    }

    if let Ok((entity, mut pos, tetrimino)) = tetriminos.get_single_mut() {
        let v_dist_from_center = VCELL_COUNT * CELL_SIZE / 2.0;
        let height = VCELL_COUNT * CELL_SIZE;

        let translated_y = (pos.translation.y + CELL_SIZE / 2.0) + v_dist_from_center;
        let row_idx = ((height - translated_y) / CELL_SIZE).ceil() as usize;

        info!("row_idx = {row_idx}");

        if row_idx < VCELL_COUNT as usize {
            let row_to_check = game_state.rows[row_idx];

            if (row_to_check & tetrimino.0) == 0 {
                let new_y = pos.translation.y - (time.delta_secs_f64() as f32 * SPEED);

                pos.translation = pos.translation.with_y(new_y);
            }
        } else {
            commans.entity(entity).remove::<Active>();
            commans.trigger(TetriminoStopped);
        }
    }
}

#[derive(Component)]
struct Field;

fn spawn_field(
    mut commans: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let h_dist_from_center = HCELL_COUNT * CELL_SIZE / 2.0;
    let v_dist_from_center = VCELL_COUNT * CELL_SIZE / 2.0;
    let border_center = BORDER_SIZE / 2.0;

    let vertical = meshes.add(Rectangle::new(BORDER_SIZE, VCELL_COUNT * CELL_SIZE));
    let horizontal = meshes.add(Rectangle::new(
        HCELL_COUNT * CELL_SIZE + (BORDER_SIZE * 2.0),
        BORDER_SIZE,
    ));
    let color = materials.add(Color::WHITE);

    // left
    commans.spawn((
        Field,
        Mesh2d(vertical.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(-h_dist_from_center - border_center, 0.0, 0.0),
    ));
    // right
    commans.spawn((
        Field,
        Mesh2d(vertical.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(h_dist_from_center + border_center, 0.0, 0.0),
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
        Transform::from_xyz(0.0, -v_dist_from_center - border_center, 0.0),
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
