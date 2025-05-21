use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow, WindowMode},
};

use bevy_rapier2d::prelude::*;

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
                present_mode: PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(80.))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

const PLAYER_SPEED: f32 = 800.0;

#[derive(Component)]
struct MapTile;

#[derive(Component)]
struct Player;

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
                block_width * 0.5 - primary_window.width() / 2.0,
                block_height * 5.5 - primary_window.height() / 2.0,
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
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
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
                    block_width * j as f32 + block_height / 2.0 - primary_window.width() / 2.0,
                    primary_window.height() / 2. - block_height * i as f32 - block_height / 2.0,
                    0.0,
                ),
                MapTile,
            ));
        }
    }

    commands.spawn((
        Mesh2d(block_mesh.clone()),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgb(
            255.0, 0.0, 0.0,
        )))),
        Transform::from_xyz(
            level1.entrance.left_entrance.unwrap().x,
            level1.entrance.left_entrance.unwrap().y,
            0.0,
        ),
        Player,
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let mut next_player_d_position = player_transform.translation;
    next_player_d_position.x += PLAYER_SPEED * time.delta_secs();
    let mut collides_d = false;

    let mut next_player_a_position = player_transform.translation;
    next_player_d_position.x -= PLAYER_SPEED * time.delta_secs();
    let mut collides_a = false;

    if input.pressed(KeyCode::KeyD) && !collides_d {
        player_transform.translation.x += PLAYER_SPEED * time.delta_secs();
    }

    if input.pressed(KeyCode::KeyA) && !collides_a {
        player_transform.translation.x -= PLAYER_SPEED * time.delta_secs();
    }
}
