
use bevy::prelude::*;
use crate::model;

use bevy_rapier3d::prelude as P;
pub fn move_player(
    mut commands: Commands,
    mut transforms:Query<(&mut Transform,&P::RigidBodyPosition,&model::Player)>)
{
    for (mut transform,mut pos,mut player) in transforms.iter_mut() {
        //cube.x+=0.001;
        transform.translation.x = pos.position.translation.x;
        transform.translation.y = pos.position.translation.y;
    }
}
