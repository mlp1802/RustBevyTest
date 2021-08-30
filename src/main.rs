

use bevy::prelude::*;
mod physics;
mod meshes;
mod setup;
mod model;
mod systems;
use bevy_rapier3d::prelude as P;
pub use physics::colliders;
pub use meshes::primitives;
use physics::rigid_bodies;

//    fn build(&self, app: &mut AppBuilder) {
//        // the reason we call from_seconds with the true flag is to make the timer repeat itself
//            app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//            .add_startup_system(setup_light_camera.system())
//            .add_startup_system(setup_cubes.system())
//            .add_startup_system(setup_plane.system())
//           .add_system(move_player.system());
//
//            //
//            //
//            //
//
//    }
//}

fn main() {
    //    let a = 54;
    //    App::build()
    //        .insert_resource(Msaa { samples: 4 })
    //        .add_plugins(DefaultPlugins)
    //        .add_plugin(CubePlugin)
    //        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    //        .add_startup_system(setup_physics.system())
    //        .add_system(print_ball_altitude.system())
    //        .run();
    let a = 54;
    App::build()
        .insert_resource(Msaa { samples: 4 })
 //       .add_startup_stagrtup_stage(CoreStage::Startup, "player", SystemStage::parallel())
        .add_startup_stage( "player",SystemStage::parallel())
        .add_startup_stage_after("player" ,"lab",SystemStage::parallel())
        .add_plugins(DefaultPlugins)
        .add_plugin(P::RapierPhysicsPlugin::<P::NoUserData>::default())
        .add_startup_system(setup::setup_plane.system())
        .add_startup_system(setup::setup_light_camera.system())
        .add_system(systems::move_player.system())
        .add_startup_system_to_stage("player",setup::setup_player.system())
        .add_startup_system_to_stage("lab",setup::setup_labyrinth.system())
        .run();

}









//fn setup_cubes(
//    mut commands: Commands,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<StandardMaterial>>,
//) {
//    // cube
//    for n in 1..11 {
//        let cube = cube__mesh(&mut meshes,1.0, &mut materials);
//        commands.spawn_bundle(cube).insert(Cube {x:(n as f32)/2.01,y:(n as f32)/3.0});
//    }
//
//}
