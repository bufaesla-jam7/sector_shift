use avian3d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default)]
pub enum ActorCollisionLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Projectiles,
}
