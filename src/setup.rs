use crate::meshes::primitives;
use crate::model::*;
use crate::physics::colliders;
use crate::physics::rigid_bodies;
use bevy::prelude::*;
use bevy_rapier3d::prelude as P;
use bevy_rapier3d::prelude::ColliderType;
use bevy_rapier3d::prelude::RigidBodyType;
use rand::Rng;
use std::cell::RefCell;
use std::{borrow::BorrowMut, primitive};

pub fn setup_labyrinth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: Query<&mut Game>,
    config: Res<Config>,
    mut transforms: Query<(&mut Transform, &mut P::RigidBodyPosition, &Player)>,
) {
    let gab = config.gab;
    let size = config.lab_size;
    let mut dots = 0;
    let commands = RefCell::new(commands);
    let materials = RefCell::new(materials);
    let meshes: RefCell<ResMut<Assets<Mesh>>> = RefCell::new(meshes);
    // Create wall
    let mut create_wall = |x: u32, z: u32| {
        let x = x as f32 * gab;
        let y = 0.0;
        let z = z as f32 * gab;
        let mut ridig_body = rigid_bodies::rigid_body(x, y, z);
        let collider = colliders::cuboid_collider(gab / 2.0, 1.0, gab / 2.0);
        let mut mesh = primitives::cube(&mut meshes.borrow_mut(), &mut materials.borrow_mut(), gab);
        ridig_body.body_type = P::RigidBodyType::Static;
        commands
            .borrow_mut()
            .spawn()
            .insert_bundle(collider)
            .insert_bundle(mesh)
            .insert_bundle(ridig_body)
            .insert(P::RigidBodyPositionSync::Discrete)
            .insert(Wall {});
    };

    // Create dot
    let mut create_dot = |x: u32, z: u32| {
        let x = x as f32 * gab;
        let z = z as f32 * gab;
        let size = gab / 5.0;
        let y = gab / 2.0;
        dots = dots + 1;

        let mut rigid_body = rigid_bodies::rigid_body(x, y, z);
        let mut collider = colliders::ball_collider(size);
        collider.collider_type = ColliderType::Sensor;
        let mut mesh =
            primitives::ball(&mut meshes.borrow_mut(), &mut materials.borrow_mut(), size);
        rigid_body.body_type = RigidBodyType::Static;
        commands
            .borrow_mut()
            .spawn()
            .insert_bundle(mesh)
            .insert_bundle(collider)
            .insert_bundle(rigid_body)
            .insert(Transform::default())
            .insert(P::RigidBodyPositionSync::Discrete)
            .insert(Dot {});
    };

    //outer walls
    for x in 0..size + 1 {
        create_wall(x, 0);
        create_wall(x, size);
        create_wall(0, x);
        create_wall(size, x);
    }
    //dots in between
    for x in (1..size).step_by(1) {
        if x % 2 == 0 {
            create_dot(x, 1);
            create_dot(x, size - 1);
        } else {
            for z in 1..size {
                create_dot(x, z);
            }
        }
    }
    //inner walls
    for x in (2..size).step_by(2) {
        let d: f32 = 0.7;
        for z in (2..size - 1) {
            let mut rng = rand::thread_rng();
            let v: f32 = rng.gen();
            if v < d {
                create_wall(x, z);
            } else {
                //create_wall(&mut commands, &mut meshes, &mut materials, x, z);
                create_dot(x, z);
            }

            let v: f32 = rng.gen();
            if v < d {
                //create_wall(&mut commands, &mut meshes, &mut materials, z, x);
            }
        }
    }

    for mut g in game.iter_mut() {
        g.dots = dots;
    }
}

pub fn setup_game(mut commands: Commands, config: Res<Config>) {
    let game: Game = Game {
        points: 0,
        dots: 0,
        status: GameStatus::Running,
    };
    commands.spawn().insert(game);
}

pub fn setup_player_and_monster(
    mut commands: Commands,
    config: Res<Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let gab = config.gab;
    let mut setup_entity = |x: u32, z: u32, e: Ent| {
        let size = config.gab / 2.5;
        let x = x as f32 * gab;
        let z = z as f32 * gab;
        let y = size;
        let rigid_body = rigid_bodies::rigid_body(x, y, z);
        let ball_collider = colliders::ball_collider(size);
        let ball_shape =
            primitives::ball(&mut meshes.borrow_mut(), &mut materials.borrow_mut(), size);
        let spawn = &mut commands.spawn();
        let c = spawn
            .insert_bundle(ball_collider)
            .insert_bundle(ball_shape)
            .insert_bundle(rigid_body)
            .insert(P::RigidBodyPositionSync::Discrete);
        match e {
            Ent::Player => {
                c.insert(Player {});
            }
            Ent::Monster => {
                c.insert(Monster {});
            }
        };
    };

    setup_entity(1, 1, Ent::Player);
    setup_entity(config.lab_size - 1, config.lab_size - 1, Ent::Monster);
}
pub fn setup_light_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<Config>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    let start_x = (config.start_x + config.gab * config.lab_size as f32) / 2.0;
    let start_z = (config.start_y + config.gab * config.lab_size as f32) / 2.0;
    let look_at = Vec3::new(start_x, 0.0, start_z); //.normalize();
    println!("start x {}", start_x);
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(start_x, 5.0, start_z).looking_at(look_at, -Vec3::Z),
        ..Default::default()
    });
}

pub fn setup_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_size = 20.0;
    //let rigid_body = rigid_bodies::rigid_body(0.0,0.0,0.0);
    let collider = colliders::plane_collider(plane_size);
    let plane_shape = primitives::plane(&mut meshes, &mut materials, plane_size);
    commands
        .spawn()
        .insert_bundle(collider)
        .insert_bundle(plane_shape);
}
