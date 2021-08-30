pub mod primitives {

    use bevy::prelude::*;

    pub fn cube(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        size: f32,
    ) -> PbrBundle {
        let bundle = PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        };
        bundle
    }

    pub fn ball(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        size: f32,
    ) -> PbrBundle {
        PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Icosphere {radius:size,subdivisions:20})),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        }
    }

    pub fn plane(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        size: f32,
    ) -> PbrBundle {
        PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane { size })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        }
    }
}
