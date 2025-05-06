#[allow(unused_imports)]
use bevy::prelude::*;

#[allow(unused_imports)]
use avian2d::prelude::*;

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

    let planets = vec![String::from("planet00.png"), String::from("planet01.png"), String::from("planet02.png"), String::from("planet03.png"), String::from("planet04.png"), String::from("planet05.png"), String::from("planet06.png"), String::from("planet07.png"), String::from("planet08.png"), String::from("planet09.png")];

    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
                Sprite::from(asset_server.load(&planets[selected_planet])),
                Transform::from_xyz(rand_loc_x, rand_loc_y, 0.0).with_scale(Vec3::splat(0.1)),
                RigidBody::Kinematic,
                Collider::circle(500.0),
                Sensor,
                Object,
        ));
    }
}

fn detect_collisions(mut collision_reader: EventReader<CollisionStarted>){
    for CollisionStarted(e1, e2) in collision_reader.read() {
        println!("entity1 : {}, entity2: {}", e1, e2);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec2::splat(0.)))
        .insert_resource(ObjectSpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player)
        .add_systems(Update, spawn_objects)
        .add_systems(Update, detect_collisions)
        .run();
}
