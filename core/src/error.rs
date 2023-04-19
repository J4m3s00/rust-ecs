use crate::entity::Entity;

#[derive(Debug, Copy, Clone)]
pub enum FindEntityLocation {
    EntityTree,
    ComponentManager,
}

#[derive(Debug, Copy, Clone)]
pub enum EcsError {
    EntityNotFound(Entity, FindEntityLocation),
    NoRootEntity,
}
