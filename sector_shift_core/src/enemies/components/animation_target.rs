use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
/// When spawning an enemy, its armature is a descendant of the [`SceneRoot`] (which sits on the
/// same entity as the [`Enemy`] component).
/// This component, also being inserted on the [`Enemy`] entity, marks the descendant entity
/// holding the armature and thus the [`AnimationPlayer`] for animating this enemy.
/// It will automatically be inserted by the [`insert_targets`] system in the [`enemy_controller`]
/// module.
pub struct EnemyAnimationTarget(pub Entity);

#[derive(Component)]
/// A temporary storage component for the [`AnimationGraphHandle`] when spawning an [`Enemy`]. This
/// handle will be inserted into the descendant entity holding the armature and
/// [`AnimationPlayer`] by the [`insert_targets`] system in the [`enemy_controller`] module.
pub struct EnemyAnimationGraphTempStorage(pub Handle<AnimationGraph>);

#[derive(Component, Reflect)]
#[reflect(Component)]
/// This component is inserted in blender using [`bevy_skein`] and saved in the gltf assets
pub struct EnemyArmature;
