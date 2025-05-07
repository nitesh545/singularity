#[allow(unused_imports)]
use bevy::prelude::*;

#[allow(unused_imports)]
use avian2d::prelude::*;

use bevy::window::{CursorGrabMode, PrimaryWindow, WindowMode};
#[allow(unused_imports)]
use rand::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Object;

#[derive(Resource)]
struct ObjectSpawnTimer(Timer);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from(asset_server.load("blackhole.png")),
        Transform::from_scale(Vec3::splat(0.05)),
        RigidBody::Kinematic,
        Collider::circle(5000.0),
        Sensor,
        CollidingEntities::default(),
        Player,
    ));
}

fn move_player(
    mut q_player: Query<&mut LinearVelocity, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut velocity = q_player.single_mut().unwrap();
    let mut x = 0.;
    let mut y = 0.;

    if keys.pressed(KeyCode::KeyA) {
        x -= 100.;
    }
    if keys.pressed(KeyCode::KeyD) {
        x += 100.;
    }

    if keys.pressed(KeyCode::KeyW) {
        y += 100.;
    }
    if keys.pressed(KeyCode::KeyS) {
        y -= 100.;
    }

    velocity.x = x;
    velocity.y = y;
}

fn spawn_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<ObjectSpawnTimer>,
) {
    let mut rng = rand::rng();
    let rand_loc_x = rng.random_range(-600.0..600.0);
    let rand_loc_y = rng.random_range(-600.0..600.0);
    let selected_planet = rng.random_range(0..9);

    let planets = vec![
        String::from("planet00.png"),
        String::from("planet01.png"),
        String::from("planet02.png"),
        String::from("planet03.png"),
        String::from("planet04.png"),
        String::from("planet05.png"),
        String::from("planet06.png"),
        String::from("planet07.png"),
        String::from("planet08.png"),
        String::from("planet09.png"),
    ];

    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Sprite::from(asset_server.load(&planets[selected_planet])),
            Transform::from_xyz(rand_loc_x, rand_loc_y, 0.0).with_scale(Vec3::splat(0.1)),
            RigidBody::Kinematic,
            Collider::circle(500.0),
            Sensor,
            CollidingEntities::default(),
            Object,
        ));
    }
}

// TODO: implement a logic for pulling planet towards center of blackhole
fn detect_collisions(q_colliding_entities: Query<(Entity, &CollidingEntities)>) {
    for (entity, colliding_entities) in &q_colliding_entities {
        println!(
            "entity : {}, colliding_entities: {:?}",
            entity, colliding_entities
        );
    }
}

fn hide_and_lock_cursor(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = q_window.single_mut().unwrap();
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn quit_game(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn spawn_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("bg.png")),
        Transform::from_scale(Vec3::splat(0.45)).with_translation(Vec3 {
            x: 0.,
            y: 0.,
            z: -10.,
        }),
    ));
}

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("music.ogg")),
        PlaybackSettings::LOOP,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec2::splat(0.)))
        .insert_resource(ObjectSpawnTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, spawn_bg)
        .add_systems(Startup, start_music)
        .add_systems(Startup, hide_and_lock_cursor)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player)
        .add_systems(Update, spawn_objects)
        .add_systems(Update, detect_collisions)
        .add_systems(Update, quit_game)
        .run();
}
