use avian3d::prelude::*;
use bevy::prelude::*;
use sector_shift_core::{
    enemies::{
        components::{EnemyAttackHitbox, EnemyAttackHitboxTarget},
        resources::AttackType,
    },
    prelude::{Enemy, EnemyLibrary},
};

use crate::actors::{
    components::ActorCollisionLayer,
    enemy_controller::{AttackImpactEvent, EnemyAction},
};

pub fn insert_targets(
    mut commands: Commands,
    parents: Query<(Has<Enemy>, Option<&ChildOf>), With<Children>>,
    added: Query<Entity, Added<EnemyAttackHitbox>>,
) {
    for target_entity in added {
        let mut current_entity = target_entity;
        loop {
            if let Ok((is_enemy, maybe_parent)) = parents.get(current_entity) {
                if is_enemy {
                    commands.entity(current_entity).insert(EnemyAttackHitboxTarget(target_entity));
                    break;
                } else if let Some(parent) = maybe_parent {
                    current_entity = parent.0;
                    continue;
                }
            }
            warn!(
                "Entity {target_entity} with the EnemyAttackHitbox component does not have \
                a parent with the Enemy component",
            );
            break;
        }
    }
}

pub fn attack_hits(
    event: On<AttackImpactEvent>,
    spatial_query: SpatialQuery,
    library: Res<EnemyLibrary>,
    enemies: Query<(&Name, &EnemyAttackHitboxTarget)>,
    hitbox_query: Query<&GlobalTransform, With<EnemyAttackHitbox>>,
) {
    if let Ok((name, target)) = enemies.get(event.entity) {
        if let Ok(transform) = hitbox_query.get(target.0) {
            let action_info = &library.map[name.as_str()].action_info;
            let attack = match event.action {
                EnemyAction::PrimaryAttack => &action_info.primary_attack,
                EnemyAction::SecondaryAttack => action_info.secondary_attack.as_ref().unwrap(),
            };
            match attack.kind {
                AttackType::Melee { damage: _, hitbox } => {
                    let shape = Collider::cuboid(hitbox.x, hitbox.y, hitbox.z);
                    let config = ShapeCastConfig {
                        max_distance: 0.1,
                        target_distance: 0.,
                        compute_contact_on_penetration: false,
                        ignore_origin_penetration: false,
                    };
                    let filter = SpatialQueryFilter::from_mask(ActorCollisionLayer::Player);
                    if let Some(hit) = spatial_query.cast_shape(
                        &shape,
                        transform.translation(),
                        Quat::IDENTITY,
                        Dir3::new_unchecked(Vec3::Z),
                        &config,
                        &filter,
                    ) {
                        info!("attack hit! {hit:#?}");
                    }
                },
                AttackType::Ranged {} => todo!(),
            }
        } else {
            warn!(
                "EnemyAttackHitboxTarget on entity {} points to an invalid entity",
                event.entity
            );
        }
    } else {
        warn!(
            "AttackImpactEvent triggered for entity {}, but this entity is \
            missing Name or EnemyAttackHitboxTarget components",
            event.entity
        );
    }
}
