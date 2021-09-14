use crate::model::*;
use bevy::{prelude::*, render::render_graph::Command};
use bevy_rapier3d::prelude::*;
use std::{ops::{Mul, Sub}, option::*};
use crate::query_utils::*;
pub fn move_monster(
    mut commands: Commands,
    mut player: Query<(&RigidBodyPosition,&Player)>,
    mut monster: Query<(&mut RigidBodyVelocity, &RigidBodyPosition,&Monster,&RigidBodyMassProps)>,

) {
    let t: Transform;
    let r: RigidBodyPosition;
    //r.position.translation.vector.sub(rhs)


    for (mut forces, monster_pos ,_monster,props) in monster.iter_mut() {
        for (player_pos, _player) in player.iter_mut() {
            let player_translation = player_pos.position.translation;
            let monster_translation = monster_pos.position.translation;
            let dir = player_translation.vector-monster_translation.vector;
            let f = 0.0002;
            forces.apply_impulse(props, dir.normalize().mul(f).into());
        }
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
pub fn collides<A,B>(query1: Query<&A>,query2: Query<&B>,e1:Entity,e2:Entity)->(bool,bool)
where
    A: std::marker::Sync,
    A: std::marker::Send,
    A: 'static,
    B: std::marker::Sync,
    B: std::marker::Send,
    B: 'static
{
    let a1 = query1.get_component::<A>(e1);
    let a2 = query1.get_component::<A>(e2);
    let a3 = query2.get_component::<A>(e1);
    let a4 = query2.get_component::<A>(e2);

    let b1 = query1.get_component::<B>(e1);
    let b2 = query1.get_component::<B>(e2);
    let b3 = query2.get_component::<B>(e1);
    let b4 = query2.get_component::<B>(e2);
    let alist = vec![a1,a2,a3,a4];
    let blist = vec![b1,b2,b3,b4];
    let hasA = alist.iter().find (|x|{x.is_ok()}).is_some();
    let hasB = blist.iter().find (|x|{x.is_ok()}).is_some();
    (hasA,hasB)
}
/* Test intersections inside of a system. */

pub fn collision_detection(
    mut commands: Commands,
    mut gameQuery: Query<(&mut Game)>,
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
    player: Query<(&Player)>,
    monster: Query<(&Monster)>,
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
                let e1  = h1.entity();
                let e2 = h2.entity();
                let result1 = player.get_component::<Player>(e1);
                let result2 = dots.get_component::<Monster>(e2);

            }
            ContactEvent::Stopped(h1, h2) => {
                println!("Stopped");
            }
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
