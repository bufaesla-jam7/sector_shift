use avian3d::prelude::*;
use bevy::prelude::*;
use sector_shift_core::prelude::*;

use crate::{
    MAP_CELL_CEILING, MAP_CELL_HEIGHT, MAP_CELL_WIDTH, PLAYER_HEALTH,
    actors::{
        components::Player,
        functions::{spawn_actor, spawn_enemy},
    },
    items::functions::spawn_item,
    maps::functions::position_to_transform::position_to_transform,
};

pub fn spawn_level(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    level: &Level,
    enemy_library: &EnemyLibrary,
    item_library: &ItemLibrary,
) -> Entity {
    let map_width = level.width() as f32 * MAP_CELL_WIDTH;
    let map_height = level.height() as f32 * MAP_CELL_HEIGHT;

    // Preload meshes
    let wall_mesh = meshes.add(Cuboid::new(
        MAP_CELL_WIDTH,
        MAP_CELL_CEILING,
        MAP_CELL_HEIGHT,
    )); // 3 units high walls
    let door_mesh = meshes.add(Cuboid::new(
        MAP_CELL_WIDTH,
        MAP_CELL_CEILING,
        MAP_CELL_HEIGHT,
    )); // placeholder
    let floor_mesh = meshes.add(Plane3d::default().mesh().size(map_width, map_height)); // Thin floor
    // item mesh
    let item_mesh = meshes.add(Rectangle::new(2.0, 2.0));

    // Preload materials
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5),
        ..Default::default()
    });
    let door_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.2, 0.1),
        ..Default::default()
    });
    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.3),
        ..Default::default()
    });

    // Spawn container for the level
    let level_entity = commands.spawn((Name::new(level.id.clone()), Transform::default())).id();

    // Spawn floor
    let floor_entity = commands
        .spawn((
            Mesh3d(floor_mesh),
            MeshMaterial3d(floor_material),
            // Transform::default(),
            Transform::from_xyz(map_width / 2.0, 0.0, map_height / 2.0),
            Collider::cuboid(map_width, 0.1, map_height),
            RigidBody::Static,
        ))
        .id();

    // Spawn container for the cells
    let cells_entity = commands.spawn((Name::new("Cells"), Transform::default())).id();

    // Spawn walls and doors
    for position in level.tiles.position_iter() {
        // We only spawn walls/doors that are not completely enclosed by other walls
        let mut enclosed = true;
        for neighbor in level.tiles.neighbors(position) {
            if let Some(tile) = level.tiles.get(neighbor) {
                match tile {
                    TileType::Door(_door_axis) => {
                        enclosed = false;
                        break;
                    },
                    TileType::Floor => {
                        enclosed = false;
                        break;
                    },
                    TileType::Wall => {},
                }
            }
        }
        if enclosed {
            continue;
        }

        // Spawn the tile
        let Some(tile) = level.tiles.get(position) else {
            continue;
        };
        let transform = position_to_transform(position);

        match tile {
            TileType::Wall => {
                let entity = commands
                    .spawn((
                        Mesh3d(wall_mesh.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        transform,
                        Collider::cuboid(MAP_CELL_WIDTH, MAP_CELL_CEILING, MAP_CELL_HEIGHT),
                        RigidBody::Static,
                    ))
                    .id();
                commands.entity(cells_entity).add_child(entity);
            },
            TileType::Door(_door_axis) => {
                let entity = commands
                    .spawn((
                        Mesh3d(door_mesh.clone()),
                        MeshMaterial3d(door_material.clone()),
                        transform,
                        //Collider::cuboid(MAP_CELL_WIDTH, MAP_CELL_CEILING, MAP_CELL_HEIGHT),
                        RigidBody::Static,
                    ))
                    .id();
                commands.entity(cells_entity).add_child(entity);
            },
            TileType::Floor => {},
        }
    }

    // Spawn player
    let direction = match level.player_start.1 {
        Direction::NORTH => Dir3::Z,
        Direction::SOUTH => Dir3::NEG_Z,
        Direction::EAST => Dir3::X,
        Direction::WEST => Dir3::NEG_X,
        _ => {
            warn!("Player start direction is not cardinal, defaulting to NORTH");
            Dir3::Z
        },
    };
    let player_transform = Transform::from_xyz(
        MAP_CELL_WIDTH * (level.player_start.0.0 as f32 + 0.5),
        MAP_CELL_CEILING / 2.0,
        MAP_CELL_HEIGHT * (level.player_start.0.1 as f32 + 0.5),
    )
    .looking_to(direction, Vec3::Y);

    let player_entity = spawn_actor(commands, player_transform, PLAYER_HEALTH);
    commands.entity(player_entity).insert((Player,));

    let camera_entity = commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 1.7, 0.0))).id();

    commands.entity(player_entity).add_child(camera_entity);

    // Spawn container for the map objects
    let objects_entity = commands.spawn((Name::new("Objects"), Transform::default())).id();

    // Spawn container for the enemies
    let enemies_entity = commands
        .spawn((
            Name::new("Enemies"),
            Transform::default(),
            ChildOf(objects_entity),
        ))
        .id();

    // Spawn container for the items
    let items_entity = commands
        .spawn((
            Name::new("Items"),
            Transform::default(),
            ChildOf(objects_entity),
        ))
        .id();

    // Spawn the [`MapObject`]s
    for (position, object) in &level.objects {
        let transform = position_to_transform(*position);
        match object {
            MapObject::Enemy(enemy_id) => {
                if let Some(entity) = spawn_enemy(commands, enemy_library, enemy_id, transform) {
                    commands.entity(enemies_entity).add_child(entity);
                }
            },
            // TODO
            MapObject::Item(item_id) => {
                if let Some(entity) = spawn_item(
                    commands,
                    item_library,
                    item_id,
                    transform,
                    item_mesh.clone(),
                    materials,
                ) {
                    commands.entity(items_entity).add_child(entity);
                }
            },
            // TODO
            MapObject::Exit(level_id) => (),
        }
    }

    // Spawn a light
    commands.spawn((
        PointLight {
            intensity: 1_500_000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(map_width / 2.0, 10.0, map_height / 2.0),
    ));

    // Build level hierarchy
    commands.entity(level_entity).add_child(floor_entity);
    commands.entity(level_entity).add_child(cells_entity);
    commands.entity(level_entity).add_child(objects_entity);

    // Return level entity
    level_entity
}
