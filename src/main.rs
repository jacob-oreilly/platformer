use bevy::{math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume}, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow};

#[derive(Component)]
struct Player{
    movement_speed: f32,
    velocity: Vec3,
}

#[derive(Component)]
struct Platform{}

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera_setup, player_setup, platform_setup))
        .add_systems(FixedUpdate, (player_update, check_for_collisions).chain())
        .add_event::<CollisionEvent>()
        .run();
}


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
            velocity: Vec3::new(0.0, 0.0, 0.0)
        },
        Collider,
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

        },
        Collider
    ));
}

fn player_update(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    let acceleration: f32 = 9.8;
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let dt = time.delta_seconds();
        player.velocity.y += acceleration * dt;
        let y_direction =  player.velocity.y * dt - transform.translation.y;
        let mut direction = Vec3::new(0.0, y_direction, 0.0);
        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, y_direction, 0.0);
        }
        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, y_direction, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * player.movement_speed * dt;
        }
    }
}

fn check_for_collisions(
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>
) {
    let (player_transform, mut player) = player_query.single_mut();

    for collider_transform in &collider_query {
        let collision = platform_collision(
            Aabb2d::new(player_transform.translation.truncate(), player_transform.scale.truncate() / 2.),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = player.velocity.x > 0.0,
                Collision::Right => reflect_x = player.velocity.x < 0.0,
                Collision::Top => reflect_y = player.velocity.y < 0.0,
                Collision::Bottom => reflect_y = player.velocity.y > 0.0,
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                player.velocity.x = 0.0;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                player.velocity.y = 0.0
            }
        }
    }
}

fn platform_collision(
    player: Aabb2d, platform: Aabb2d
) -> Option<Collision> {
    if !player.intersects(&platform) {
        return None;
    }

    let closest = platform.closest_point(player.center());
    let offset = player.center() - closest;
    let current_platform = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(current_platform)
}