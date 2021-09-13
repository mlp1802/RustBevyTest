//use bevy::prelude as B;
pub mod colliders {

    use bevy_rapier3d::prelude::*;
    pub fn box_collider(size:f32) -> ColliderBundle {
        cuboid_collider(size,size,size)
    }

    pub fn cuboid_collider(size_x: f32, size_y: f32, size_z: f32) -> ColliderBundle {
        ColliderBundle {
            //flags: ColliderFlags::default(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            shape: ColliderShape::cuboid(size_x, size_y, size_z),
            ..Default::default()
        }
    }

    pub fn plane_collider(size: f32) -> ColliderBundle {
        cuboid_collider(size, 0.01, size)
    }

    pub fn ball_collider(size: f32) -> ColliderBundle {
        ColliderBundle {
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            shape: ColliderShape::ball(size),
            ..Default::default()
        }
    }

}
pub mod rigid_bodies {

    use bevy_rapier3d::prelude::*;
    use bevy::prelude as B;
    pub fn rigid_body(x:f32,y:f32,z:f32)->RigidBodyBundle {
        RigidBodyBundle {
            position: B::Vec3::new(x,y,z).into(),
            ..Default::default()
        }
    }
}
