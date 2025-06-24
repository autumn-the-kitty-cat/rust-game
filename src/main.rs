use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow, WindowMode},
};

use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 700.0;
const FALL_RATE: f32 = 3000.0;
const JUMP_RATE: f32 = 1300.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game!".into(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                cursor_options: CursorOptions {
                    visible: false,
                    grab_mode: CursorGrabMode::Confined,
                    ..Default::default()
                },
                resolution: (1920., 1080.).into(),
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(80.))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_walk,
                gravity_fall,
                (jump_collision, jump, move_jump).chain(),
            ),
        )
        .run();
}

#[derive(Component)]
enum Facing {
    Left,
    Right,
}

#[derive(Component)]
struct Jump;

#[derive(Component)]
struct MapTile;

#[derive(Component)]
struct Gravity;

#[derive(Component)]
struct Player;

#[allow(dead_code)]
struct Entrance {
    left_entrance: Option<Vec2>,
    right_entrance: Option<Vec2>,
    up_entrance: Option<Vec2>,
    down_entrance: Option<Vec2>,
}

struct LevelMap {
    width: usize,
    entrance: Entrance,
    map: Vec<usize>,
}

#[derive(Component)]
struct TimeFalling(f32);

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    primary_window: Single<&Window, With<PrimaryWindow>>,
) {
    let block_width = primary_window.width() / 32.0;
    let block_height = primary_window.height() / 18.0;

    let block_mesh = meshes.add(Rectangle::new(block_width, block_height));

    let level1 = LevelMap {
        width: 32,
        entrance: Entrance {
            left_entrance: Some(Vec2::new(
                block_width * 2.5 - primary_window.width() / 2.0,
                block_height * 8.5 - primary_window.height() / 2.0,
            )),
            right_entrance: None,
            up_entrance: None,
            down_entrance: None,
        },
        #[rustfmt::skip]
        map: vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
    };

    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    for index in 0..level1.map.len() {
        if level1.map[index] == 1 {
            let i = index / level1.width;
            let j = index % level1.width;
            commands.spawn((
                Mesh2d(block_mesh.clone()),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgb(
                    0.0, 255.0, 0.0,
                )))),
                Transform::from_xyz(
                    block_width * j as f32 + block_width / 2.0 - primary_window.width() / 2.0,
                    primary_window.height() / 2.0 - block_height * i as f32 - block_height / 2.0,
                    0.0,
                ),
                Collider::cuboid(block_width / 2.0, block_height / 2.0),
                RigidBody::Fixed,
                MapTile,
            ));
        }
    }

    commands.spawn((
        Player,
        Mesh2d(block_mesh.clone()),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgb(
            255.0, 0.0, 0.0,
        )))),
        Transform::from_xyz(
            level1.entrance.left_entrance.unwrap().x,
            level1.entrance.left_entrance.unwrap().y,
            0.0,
        ),
        Collider::cuboid(block_width / 2.0, block_height / 2.0),
        Gravity,
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController { ..default() },
        RigidBody::KinematicPositionBased,
        Facing::Right,
    ));
}

fn move_walk(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_controller: Single<&mut KinematicCharacterController>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    let mut player_entity = commands.entity(*player);

    let mut transform = Vec2::ZERO;
    if input.pressed(KeyCode::KeyD) {
        transform.x = PLAYER_SPEED * time.delta_secs();
        player_entity.remove::<Facing>();
        player_entity.insert(Facing::Right);
    } else if input.pressed(KeyCode::KeyA) {
        transform.x = -PLAYER_SPEED * time.delta_secs();
        player_entity.remove::<Facing>();
        player_entity.insert(Facing::Left);
    }

    match player_controller.translation {
        Some(t) => player_controller.translation = Some(t + transform),
        _ => player_controller.translation = Some(transform),
    }
}

fn jump(
    input: Res<ButtonInput<KeyCode>>,
    player_controller_output: Query<&KinematicCharacterControllerOutput, With<Player>>,
    player: Single<Entity, With<Player>>,
    jump_query: Query<&mut Jump, With<Player>>,
    mut commands: Commands,
) {
    if !player_controller_output.is_empty()
        && input.pressed(KeyCode::Space)
        && jump_query.is_empty()
        && player_controller_output.iter().next().unwrap().grounded
    {
        commands.entity(*player).insert(Jump);
    }
}

fn move_jump(
    time: Res<Time>,
    mut player_controller: Single<&mut KinematicCharacterController, With<Player>>,
    jump_query: Query<&mut Jump, With<Player>>,
) {
    if jump_query.is_empty() {
        return;
    }

    let jump_transform = Vec2::new(0.0, JUMP_RATE * time.delta_secs());
    match player_controller.translation {
        Some(t) => player_controller.translation = Some(t + jump_transform),
        _ => player_controller.translation = Some(jump_transform),
    };
}

fn gravity_fall(
    time: Res<Time>,
    mut commands: Commands,
    mut objects_with_gravity: Query<
        (
            Option<&mut TimeFalling>,
            Entity,
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
        ),
        With<Gravity>,
    >,
) {
    for (time_falling_option, entity, mut object, output) in objects_with_gravity.iter_mut() {
        if time_falling_option.is_none() {
            commands.entity(entity).insert(TimeFalling(0.0));
            continue;
        }

        if output.grounded {
            commands.entity(entity).remove::<TimeFalling>();
            continue;
        }

        let mut time_falling = time_falling_option.unwrap();
        let transform = Vec2::new(0.0, -FALL_RATE * time_falling.0 * time.delta_secs());
        time_falling.0 += time.delta_secs();

        match object.translation {
            Some(t) => object.translation = Some(t + transform),
            _ => object.translation = Some(transform),
        }
    }
}

fn jump_collision(
    mut player_controller_output: Query<
        &KinematicCharacterControllerOutput,
        (With<Player>, With<Jump>),
    >,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    if player_controller_output.is_empty() {
        return;
    }

    let output = player_controller_output.iter_mut().next().unwrap();
    if (output.desired_translation.y != output.effective_translation.y
        && output.desired_translation.x == output.effective_translation.x)
        || output.grounded
    {
        commands.entity(*player).remove::<Jump>();
    }
}
