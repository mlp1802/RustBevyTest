use crate::model::*;
use crate::query_utils::*;
use bevy::{prelude::*, render::render_graph::Command};
use bevy_rapier3d::prelude::*;
use std::{
    ops::{Mul, Sub},
    option::*,
};
pub fn move_monsters(
    mut commands: Commands,
    game_query: Query<&Game>,
    mut player: Query<(&RigidBodyPosition, &Player)>,
    mut monster: Query<(
        &mut RigidBodyVelocity,
        &RigidBodyPosition,
        &Monster,
        &RigidBodyMassProps,
    )>,
) {
    for game in game_query.iter() {
        for (mut forces, monster_pos, _monster, props) in monster.iter_mut() {
            for (player_pos, _player) in player.iter_mut() {
                let mut apply_force =  | | {
                    let player_translation = player_pos.position.translation;
                    let monster_translation = monster_pos.position.translation;
                    let dir = player_translation.vector - monster_translation.vector;
                    let f = 0.0001;
                    forces.apply_impulse(props, dir.normalize().mul(f).into());
                };
                match game.status {
                    GameStatus::Running=> {
                        apply_force();
                    }
                    GameStatus::Lost=> {
                        apply_force();
                    }
                    GameStatus::Won =>{}
                }
            }
        }
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    game_query: Query<&Game>,
    mut rigid_bodies: Query<(
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
        &Player,
    )>,
) {
    let status = first_value(&game_query, |x| x.status);
    match status {
        Some(GameStatus::Lost) => {
            return;
        }
        _ => {}
    };
    let p = [
        (KeyCode::Left, Vec3::new(-1.0, 0.0, 0.0)),
        (KeyCode::Right, Vec3::new(1.0, 0.0, 0.0)),
        (KeyCode::Up, Vec3::new(0.0, 0.0, -1.0)),
        (KeyCode::Down, Vec3::new(0.0, 0.0, 1.0)),
    ];
    let is_pressed = |(k, v): (KeyCode, Vec3)| {
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
    for (mut rb_vel, rb_mprops, _player) in rigid_bodies.iter_mut() {
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
    mut game_query: Query<&mut Game>,
    narrow_phase: Res<NarrowPhase>,
    player: Query<(Entity, &Player)>,
    monster: Query<(Entity, &Monster)>,
    dots: Query<(Entity, &Dot)>,
) {
    for (player_entity, _) in player.iter() {
        // Update dots
        for mut game in game_query.iter_mut() {
            for (dot_entity, _) in dots.iter() {
                let contact =
                    narrow_phase.intersection_pair(player_entity.handle(), dot_entity.handle());
                match contact {
                    Some(_) => {
                        commands.entity(dot_entity).despawn();
                        game.points = game.points + 10;
                        game.dots = game.dots - 1;
                        if game.dots == 0 {
                            game.status = GameStatus::Won;
                        }
                    }
                    None => {}
                };
            }

            //check monster
            for (monster_entity, _) in monster.iter() {
                let contact =
                    narrow_phase.contact_pair(player_entity.handle(), monster_entity.handle());
                match (game.status,contact) {
                    (GameStatus::Running, Some(_)) => {
                        game.status = GameStatus::Lost;
                    }
                    _ => {}
                }
            }
        }
    }
}
