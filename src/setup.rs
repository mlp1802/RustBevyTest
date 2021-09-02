use crate::meshes::primitives;
use crate::model;
use crate::physics::colliders;
use crate::physics::rigid_bodies;
use bevy::prelude::*;
use bevy_rapier3d::prelude as P;
use bevy_rapier3d::prelude::ColliderType;
use bevy_rapier3d::prelude::RigidBodyType;
use rand::Rng;
use std::{borrow::BorrowMut, primitive};

pub fn setup_labyrinth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<model::Config>,
    mut transforms: Query<(&mut Transform, &mut P::RigidBodyPosition, &model::Player)>,
) {
    //println!("setup lab");
    let gab = config.gab;
    let size = config.lab_size;
    let mut c = &commands;
    let mut dots = 0;
    let mut create_wall = |commands: &mut Commands,
                           meshes: &mut ResMut<Assets<Mesh>>,
                           materials: &mut ResMut<Assets<StandardMaterial>>,
                           x: u32,
                           z: u32| {
        let x = x as f32 * gab;
        let y = 0.0;
        let z = z as f32 * gab;
        let mut ridig_body = rigid_bodies::rigid_body(x, y, z);
        let collider = colliders::cuboid_collider(gab / 2.0, 1.0, gab / 2.0);
        let mut mesh = primitives::cube(meshes, materials, gab);
        ridig_body.body_type = P::RigidBodyType::Static;
        mesh.transform.translation.x = x;
        mesh.transform.translation.z = z;
        mesh.transform.translation.y = y;
        commands
            .spawn()
            .insert_bundle(collider)
            .insert_bundle(mesh)
            .insert_bundle(ridig_body)
            .insert(Transform::default())
            .insert(P::RigidBodyPositionSync::Discrete)
            .insert(model::Wall {});
                           };

    let mut create_dot = |commands: &mut Commands,
                          meshes: &mut ResMut<Assets<Mesh>>,
                          materials: &mut ResMut<Assets<StandardMaterial>>,
                          x: u32,
                          z: u32| {
                              let x = x as f32 * gab;
                              let z = z as f32 * gab;
                              let size = gab/5.0;
                              let y = gab/2.0;
                              dots = dots+1;

                              let mut rigid_body = rigid_bodies::rigid_body(x, y, z);
                              let mut collider = colliders::ball_collider(size);
                              collider.collider_type = ColliderType::Sensor;
                              let mut mesh = primitives::ball(meshes, materials, size);
                              rigid_body.body_type = RigidBodyType::Static;
                              commands.spawn()
                                .insert_bundle(mesh)
                                  .insert_bundle(collider)
                                  .insert_bundle(rigid_body)
                                  .insert(Transform::default())
                                  .insert(P::RigidBodyPositionSync::Discrete)
                                  .insert(model::Dot {});


    };

    //outer walls
    for x in 0..size + 1 {
        create_wall(&mut commands, &mut meshes, &mut materials, x, 0);
        create_wall(&mut commands, &mut meshes, &mut materials, x, size);
        create_wall(&mut commands, &mut meshes, &mut materials, 0, x);
        create_wall(&mut commands, &mut meshes, &mut materials, size, x);
    }
    //dots in between
    for x in (1..size).step_by(1) {
        if x%2==0 {
            create_dot(&mut commands, &mut meshes, &mut materials, x, 1);
            create_dot(&mut commands, &mut meshes, &mut materials, x, size-1);
        }else {
            for z in 1..size {
                create_dot(&mut commands, &mut meshes, &mut materials, x, z);
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
                create_wall(&mut commands, &mut meshes, &mut materials, x, z);
            } else {

                //create_wall(&mut commands, &mut meshes, &mut materials, x, z);
                create_dot(&mut commands, &mut meshes, &mut materials, x, z);
            }

            let v: f32 = rng.gen();
            if v < d {
                //create_wall(&mut commands, &mut meshes, &mut materials, z, x);
            }
        }
    }

    let f = 0.01;
    let start_player_x = (config.start_x + gab * 1.0) + f;
    let start_player_z = (config.start_y + gab * 1.0) + f;
    for (mut transform, mut pos, mut player) in transforms.iter_mut() {
        pos.position.translation.x = start_player_x;
        pos.position.translation.z = start_player_z;
    }
    println!("DOTS {}",dots)
}

pub fn setup_player(
    mut commands: Commands,

    config: Res<model::Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("setup player");
    let player = model::Player {};
    let size = config.gab / 2.5;
    let x = 0.0;
    let y = size;
    let z = 1.0;
    let rigidBody = rigid_bodies::rigid_body(x, y, x);
    let ball_collider = colliders::ball_collider(size);
    let mut ball_shape = primitives::ball(&mut meshes, &mut materials, size);
    ball_shape.transform.translation.x = x;
    ball_shape.transform.translation.y = y;
    ball_shape.transform.translation.z = z;

    commands
        .spawn()
        .insert_bundle(rigidBody)
        .insert(Transform::default())
        .insert(P::RigidBodyPositionSync::Discrete)
        .insert_bundle(ball_collider)
        .insert_bundle(ball_shape)
        .insert(player);

    //commands.sp
}
pub fn setup_light_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<model::Config>,
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
