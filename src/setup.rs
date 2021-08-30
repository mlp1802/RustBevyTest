use crate::meshes::primitives;
use crate::model;
use crate::physics::colliders;
use crate::physics::rigid_bodies;
use bevy::prelude::*;
use std::{borrow::BorrowMut, primitive};
use rand::Rng;
use bevy_rapier3d::prelude as P;

pub fn setup_labyrinth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<(&mut Transform, &mut P::RigidBodyPosition, &model::Player)>,
) {
    //println!("setup lab");
    let gab = 0.3;
    let size = 20;
    let mut c = &commands;
    let mut create_wall = |commands: &mut Commands,
                           meshes: &mut ResMut<Assets<Mesh>>,
                           materials: &mut ResMut<Assets<StandardMaterial>>,
                           x: u32,
                           z: u32| {
        let x = x as f32 * gab;
        let z = z as f32 * gab;
        let collider = colliders::box_collider(gab);
        let mut mesh = primitives::cube(meshes, materials, gab);
        mesh.transform.translation.x = x;
        mesh.transform.translation.z = z;
        commands.spawn().insert_bundle(collider).insert_bundle(mesh);
    };
    let mut create_dot = |commands: &mut Commands,
                          meshes: &mut ResMut<Assets<Mesh>>,
                          materials: &mut ResMut<Assets<StandardMaterial>>,
                          dot_x: u32,
                          dot_z: u32| {
        let dot_x = dot_x as f32 * gab;
        let dot_z = dot_z as f32 * gab;
        let collider = colliders::box_collider(gab);
        let mut mesh = primitives::cube(meshes, materials, gab);
        mesh.transform.translation.x = dot_x;
        mesh.transform.translation.z = dot_z;
        commands.spawn().insert_bundle(collider).insert_bundle(mesh);
    };

    //outer walls
    for x in 0..size {
        create_wall(&mut commands, &mut meshes, &mut materials, x, 0);
        create_wall(&mut commands, &mut meshes, &mut materials, x, size);
        create_wall(&mut commands, &mut meshes, &mut materials, 0, x);
        create_wall(&mut commands, &mut meshes, &mut materials, size, x);
    }
    //inner walls
    for x in (2..size - 2).step_by(2) {
        let d:f32 = 0.6;
        for z in (2..size - 2) {
            let mut rng = rand::thread_rng();
            let v:f32 = rng.gen();
            if v<d {
                create_wall(&mut commands, &mut meshes, &mut materials, x, z);
            }

            let v:f32 = rng.gen();
            if v<d {
                //create_wall(&mut commands, &mut meshes, &mut materials, z, x);
            }

        }
    }

    for (mut transform, mut pos, mut player) in transforms.iter_mut() {
        //cube.x+=0.001;
        //transform.translation.x = 10.0;//pos.position.translation.x;
        //transform.translation.y = pos.position.translation.y;
        //pos.position.translation.x = 10.0;
        //pos.position.translation.y = 10.0;
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("setup player");
    let player = model::Player {};
    let size = 0.1;
    let x = 0.0;
    let y = 2.0;
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
        .insert_bundle(ball_collider)
        .insert_bundle(ball_shape)
        .insert(player);

    //commands.sp
}
pub fn setup_light_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 3.6, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
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
