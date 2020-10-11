use crate::bundles::*;
use crate::helpers::shapes::*;
use crate::systems::{attack, drone, faction::*, selection::Selectable, unit::*, walker};
use bevy::{math::Quat, prelude::*};
use bevy_contrib_colors::Tailwind;
use bevy_mod_picking::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let camera_entity = commands
        .spawn(Camera3dComponents {
            ..Default::default()
        })
        .with(PickSource::default())
        .current_entity()
        .unwrap();

    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 400.0 })),
            material: materials.add(Tailwind::RED100.into()),
            ..Default::default()
        })
        .with(PickableMesh::default())
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

    let walker_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let circle_mesh = meshes.add(circle_mesh());
    let circle_material = color_materials.add(Tailwind::BLUE500.into());
    let material = materials.add(Tailwind::RED400.into());
    for i in 0..5 {
        for j in 0..5 {
            create_walker(
                &mut commands,
                walker_mesh,
                material,
                None,
                Vec3::new(i as f32 * 5.0 - 10.0, 1.0, j as f32 * 5.0 - 10.0),
                circle_mesh,
                circle_material,
            );
        }
    }

    let drone_mesh = meshes.add(Mesh::from(shape::Icosphere {
        subdivisions: 4,
        radius: 1.0,
    }));
    create_drone(
        &mut commands,
        drone_mesh,
        material,
        None,
        Vec3::new(10.0, 20.0, 5.0),
        circle_mesh,
        circle_material,
    );
    create_drone(
        &mut commands,
        drone_mesh,
        material,
        Some(camera_entity),
        Vec3::new(-25.0, 60.0, 0.0),
        circle_mesh,
        circle_material,
    );
}

fn create_walker(
    mut commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    camera_entity: Option<Entity>,
    position: Vec3,
    circle_mesh: Handle<Mesh>,
    circle_material: Handle<ColorMaterial>,
) {
    let selectable = Selectable::new(&mut commands, circle_mesh, circle_material);
    commands
        .spawn(PbrComponents {
            mesh,
            material,
            transform: Transform::from_translation(Vec3::new(position.x(), 1.0, position.z())),
            ..Default::default()
        })
        .with(walker::Walker::default())
        .with(attack::Ranged::default())
        .with(Faction::new(Factions::Player))
        .with(selectable)
        .with_bundle(UnitBundle {
            unit: Unit {
                speed: 0.1,
                ..Default::default()
            },
            ..UnitBundle::new(camera_entity)
        });
}

fn create_drone(
    mut commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    camera_entity: Option<Entity>,
    position: Vec3,
    circle_mesh: Handle<Mesh>,
    circle_material: Handle<ColorMaterial>,
) {
    let selectable = Selectable::new(&mut commands, circle_mesh, circle_material);

    commands
        .spawn(PbrComponents {
            mesh,
            material,
            transform: Transform::new(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                position,
            )),
            ..Default::default()
        })
        .with(drone::Drone::default())
        .with(Faction::new(Factions::Player))
        .with(selectable)
        .with_bundle(UnitBundle {
            unit: Unit {
                speed: 0.3,
                ..Default::default()
            },
            ..UnitBundle::new(camera_entity)
        });
}
