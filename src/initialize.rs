use crate::bundles::*;
use crate::systems::{
    ability::*,
    attack,
    camera::{CameraFollow, CanHaveCamera},
    health::*,
    selection::SelectableBuilder,
    unit::*,
};
use bevy::{math::Quat, prelude::*};
use bevy_contrib_colors::Tailwind;
use bevy_mod_picking::*;

pub fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 400.0 })),
            material: materials.add(Tailwind::RED100.into()),
            ..Default::default()
        })
        .with(PickableMesh::default())
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(UiCameraBundle::default());

    let walker_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let big_walker_mesh = meshes.add(Mesh::from(shape::Cube { size: 5.0 }));
    let material = materials.add(Tailwind::RED400.into());
    for i in 0..5 {
        for j in 0..5 {
            create_walker(
                commands,
                walker_mesh.clone(),
                material.clone(),
                Vec3::new(i as f32 * 5.0 - 10.0, 1.0, j as f32 * 5.0 - 10.0),
            );
        }
    }

    create_tp_healer(
        commands,
        big_walker_mesh,
        materials.add(Tailwind::RED700.into()),
        Vec3::new(20.0, 0.0, 20.0),
    );

    let drone_mesh = meshes.add(Mesh::from(shape::Icosphere {
        subdivisions: 4,
        radius: 1.0,
    }));
    create_drone(
        commands,
        drone_mesh.clone(),
        material.clone(),
        Vec3::new(10.0, 20.0, 5.0),
    );
    let camera_holder = create_drone(commands, drone_mesh, material, Vec3::new(-25.0, 60.0, 0.0));

    commands
        .spawn(Camera3dBundle {
            ..Default::default()
        })
        .with(PickSource::default())
        .with(CameraFollow {
            entity: Some(camera_holder),
            ..Default::default()
        });
}

fn create_walker(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) -> Entity {
    commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(Vec3::new(position.x, 1.0, position.z)),
            ..Default::default()
        })
        .with(SelectableBuilder::default())
        .with(CanHaveCamera::default())
        .with_bundle(UnitBundle::default())
        .with_bundle(WalkerBundle::default())
        .with(attack::Ranged::default())
        .current_entity()
        .unwrap()
}

fn create_tp_healer(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) -> Entity {
    commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(Vec3::new(position.x, 5.0, position.z)),
            ..Default::default()
        })
        .with(SelectableBuilder::default())
        .with(CanHaveCamera::default())
        .with_bundle(UnitBundle {
            unit: Unit {
                speed: 30.0,
                ..Default::default()
            },
            size: UnitSize(5.),
            health: Health::new(10),
            abilities: UnitAbilities {
                abilities: vec![
                    AbilityButton {
                        name: "Teleport".to_string(),
                        id: "teleport",
                        callback: |_commands, mut ability, _buttons, callback_data| {
                            ability.ability = Ability::Teleport(callback_data.entity.unwrap());
                        },
                    },
                    AbilityButton {
                        name: "Heal".to_string(),
                        id: "heal-unit",
                        callback: |_commands, mut ability, _buttons, _callback_data| {
                            ability.ability = Ability::HealUnit;
                        },
                    },
                    AbilityButton {
                        name: "Heal area".to_string(),
                        id: "heal-area",
                        callback: |_commands, mut ability, _buttons, _callback_data| {
                            ability.ability = Ability::HealArea;
                        },
                    },
                ],
            },
            ..UnitBundle::default()
        })
        .with_bundle(WalkerBundle::default())
        .current_entity()
        .unwrap()
}

fn create_drone(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) -> Entity {
    commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                position,
            )),
            ..Default::default()
        })
        .with(SelectableBuilder::default())
        .with(CanHaveCamera::default())
        .with_bundle(UnitBundle {
            unit: Unit {
                speed: 30.0,
                ..Default::default()
            },
            ..UnitBundle::default()
        })
        .with_bundle(DroneBundle::default())
        .with(attack::Ranged::default())
        .current_entity()
        .unwrap()
}
