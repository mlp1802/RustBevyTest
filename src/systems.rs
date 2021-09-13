use crate::model::*;
use bevy::{prelude::*, render::render_graph::Command};
use bevy_rapier3d::prelude::*;
use std::{ops::Mul, option::*};
use crate::query_utils::*;
pub fn sync_rigid_bodies(
    mut commands: Commands,
    mut transforms: Query<(&mut Transform, &RigidBodyPosition)>,
) {
    let t: Transform;
    let r: RigidBodyPosition;
    //  r.position.

    for (mut tramsform, mut pos) in transforms.iter_mut() {
        //transform.translation  = pos.position.translation;
        //transform.translation.y = pos.position.translation.y;
        //transform.translation.z = pos.position.translation.z;
    }
}


pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    game_query: Query<&Game>,
    mut rigid_bodies: Query<(
        &mut RigidBodyForces,
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
        &Player,
    )>,
) {
    let status = first_value(&game_query, |x| x.status);
      match status {
          Some (GameStatus::Lost)=>{return;},
          Some (GameStatus::Won)=>{return;},
           _ =>{}
      };
    let p = [
        (KeyCode::Left, Vec3::new(-1.0, 0.0, 0.0)),
        (KeyCode::Right, Vec3::new(1.0, 0.0, 0.0)),
        (KeyCode::Up, Vec3::new(0.0, 0.0, -1.0)),
        (KeyCode::Down, Vec3::new(0.0, 0.0, 1.0)),
    ];
    let mut is_pressed = |(k, v): (KeyCode, Vec3)| {
        if keyboard_input.pressed(k) {
            Some(v)
        } else {
            None
        }
    };
    //p.iter().filter()
    let forces = p
        .iter()
        .map(|x| is_pressed(*x))
        .filter_map(|x| x)
        .collect::<Vec<Vec3>>();
    //let r:RigidBodyBundlep
    for (mut rb_forces, mut rb_vel, rb_mprops, player) in rigid_bodies.iter_mut() {
        // Apply forces.
        forces.iter().for_each(|v: &Vec3| {
            let f = 0.0006;
            rb_vel.apply_impulse(rb_mprops, v.clone().mul(f).into());
        });
    }
}
/* Test intersections inside of a system. */

pub fn collision_detection(
    mut commands: Commands,
    mut gameQuery: Query<(&mut Game)>,
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
    player: Query<(&Player)>,
    dots: Query<(&Dot)>,
) {
    for intersection_event in intersection_events.iter() {
        let check_dot = |c, e| match c {
            (Ok(player), Ok(dot)) => Some(e),
            _ => None,
        };

        let mut remove_dot = |c: &mut Commands, entity: Option<Entity>| {
            match entity {
                Some(x) => {
                    c.entity(x).despawn();
                    for mut game in gameQuery.iter_mut() {
                        game.points = game.points+10;
                        game.dots = game.dots - 1;
                    }
                }
                _ => {}
            }
        };
        if intersection_event.intersecting {
            let e1 = intersection_event.collider1.entity();
            let e2 = intersection_event.collider2.entity();
            let result1 = player.get_component::<Player>(e1);
            let result2 = dots.get_component::<Dot>(e2);
            let c1 = (result1, result2);
            remove_dot(&mut commands, check_dot(c1, e2));
            let result1 = player.get_component::<Player>(e2);
            let result2 = dots.get_component::<Dot>(e1);
            let c1 = (result1, result2);
            remove_dot(&mut commands, check_dot(c1, e1));
        }
    }

    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let e1 = h1.entity();
                let e2 = h2.entity();
                let result1 = player.get_component::<Player>(e1);
                let result2 = dots.get_component::<Dot>(e2);
                let result3 = player.get_component::<Player>(e2);
                let result4 = dots.get_component::<Dot>(e1);
                let c1 = (result1, result2, result3, result4);
                match c1 {
                    (Ok(player), Ok(dot), _, _) => {
                        print!("LOL1")
                    }
                    (_, _, Ok(dot), Ok(player)) => {
                        print!("LOL2")
                    }
                    _ => {}
                }
            }
            ContactEvent::Stopped(h1, h2) => {}
        }
    }
}
pub fn check_if_game_over(mut query: Query<&mut Game, Changed<Game>>) {
    for mut game in query.iter_mut() {
        println!("GAME CHANGED {}", game.dots);
        if game.dots == 0 {
            game.status = GameStatus::Won;
        }
    }
}
