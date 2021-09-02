use crate::model;
use bevy::prelude::*;
use std::{ops::Mul, option::*};
use bevy_rapier3d::prelude::*;
pub fn sync_rigid_bodies(
    mut commands: Commands,
    mut transforms: Query<(&mut Transform, &RigidBodyPosition)>,
) {
    let t:Transform;
    let r:RigidBodyPosition;
  //  r.position.
    for (mut tramsform, mut pos) in transforms.iter_mut() {
        //transform.translation  = pos.position.translation;
        //transform.translation.y = pos.position.translation.y;
        //transform.translation.z = pos.position.translation.z;

    }
}
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut rigid_bodies: Query<(&mut RigidBodyForces, &mut RigidBodyVelocity, &RigidBodyMassProps,&model::Player)>
) {
    let p = [
        (KeyCode::Left, Vec3::new(-1.0, 0.0, 0.0)),
        (KeyCode::Right, Vec3::new(1.0, 0.0, 0.0)),
        (KeyCode::Up, Vec3::new(0.0, 0.0, -1.0)),
        (KeyCode::Down, Vec3::new(0.0, 0.0, 1.0))
    ];
    let mut is_pressed = | (k,v):(KeyCode,Vec3) | {
        if keyboard_input.pressed(k) {
            //println!("PRESSED {}",v);
            //None
            Some(v)
        }else {
            None
        }

    };
    //p.iter().filter()
    let forces = p.iter().map(|x| is_pressed(*x)).filter_map(|x| x).collect::<Vec<Vec3>>();
    //let r:RigidBodyBundlep
    for (mut rb_forces, mut rb_vel, rb_mprops, player) in rigid_bodies.iter_mut() {
        // Apply forces.
        forces.iter().for_each(|v:&Vec3| {
            let f = 0.0004;
            rb_vel.apply_impulse(rb_mprops, v.clone().mul(f).into());
            //rb_vel.apply_torque_impulse(rb_mprops, v.clone().mul(f).into());
        });
    }
}
