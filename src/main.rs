use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera_setup, player_setup, platform_setup))
        .add_systems(Update, player_update)
        .run();
}

#[derive(Component)]
struct Player{
    movement_speed: f32,
}

#[derive(Component)]
struct Platform{}

fn camera_setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

fn player_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();
    commands.spawn((
        MaterialMesh2dBundle{
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(15.0, 35.0))),
            material: materials.add(Color::FUCHSIA),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Player {
            movement_speed: 500.0,
        }
    ));
}

fn platform_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();
    commands.spawn((
        MaterialMesh2dBundle{
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(200.0, 20.0))),
            material: materials.add(Color::GRAY),
            transform: Transform::from_xyz(window.width() / 2.0, (window.height() / 2.0) - 100.0, 0.0),
            ..default()
        },
        Platform {

        }
    ));
}

fn player_update(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * player.movement_speed * time.delta_seconds();
        }
    }
}