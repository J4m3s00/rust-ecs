use std::{any::Any, collections::HashMap};

use crate::{entity::Entity, entity_tree::EntityTree};

type ComponentStore = HashMap<(Entity, String), Box<dyn Any>>;

pub struct EntityComponentManager {
    component_store: ComponentStore,
    entites: EntityTree,
    entity_counter: Entity,
}

impl Default for EntityComponentManager {
    fn default() -> Self {
        Self {
            component_store: HashMap::new(),
            entites: EntityTree::default(),
            entity_counter: Entity(0),
        }
    }
}

impl EntityComponentManager {
    pub fn create_entity(&mut self) -> Entity {
        self.entity_counter.0 += 1;
        self.entites.insert_node(self.entity_counter);
        self.entity_counter
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.entites.remove(entity);
        self.component_store.retain(|(e, _), _| *e != entity);
    }

    pub fn insert_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.component_store.insert(
            (entity, std::any::type_name::<T>().to_string()),
            Box::new(component),
        );
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.component_store
            .get(&(entity, std::any::type_name::<T>().to_string()))
            .and_then(|component| component.downcast_ref::<T>())
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.component_store
            .get_mut(&(entity, std::any::type_name::<T>().to_string()))
            .and_then(|component| component.downcast_mut::<T>())
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        self.component_store
            .remove(&(entity, std::any::type_name::<T>().to_string()));
    }

    pub fn queury_component<T: 'static>(&self) -> Vec<(Entity, &T)> {
        self.component_store
            .iter()
            .filter(|(_, component)| component.is::<T>())
            .map(|((entity, _), component)| (*entity, component.downcast_ref::<T>().unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entity() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        assert_eq!(entity, Entity(1));
    }

    #[test]
    fn delete_entity() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.delete_entity(entity);
        assert_eq!(manager.entites.len(), 0);
    }

    #[test]
    fn insert_component() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        assert_eq!(manager.get_component::<i32>(entity), Some(&1));
    }

    #[test]
    fn get_component() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        assert_eq!(manager.get_component::<i32>(entity), Some(&1));
    }

    #[test]
    fn get_component_mut() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        assert_eq!(manager.get_component_mut::<i32>(entity), Some(&mut 1));
    }

    #[test]
    fn remove_component() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        manager.remove_component::<i32>(entity);
        assert_eq!(manager.get_component::<i32>(entity), None);
    }

    #[test]
    fn get_component_with_different_type() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        assert_eq!(manager.get_component::<f32>(entity), None);
    }

    #[test]
    fn get_component_mut_with_different_type() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        assert_eq!(manager.get_component_mut::<f32>(entity), None);
    }

    #[test]
    fn remove_component_with_different_type() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        manager.remove_component::<f32>(entity);
        assert_eq!(manager.get_component::<i32>(entity), Some(&1));
    }

    #[test]
    fn get_component_with_different_entity() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        assert_eq!(manager.get_component::<i32>(Entity(2)), None);
    }

    #[test]
    fn get_component_mut_with_different_entity() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        assert_eq!(manager.get_component_mut::<i32>(Entity(2)), None);
    }

    #[test]
    fn remove_component_with_different_entity() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        manager.remove_component::<i32>(Entity(2));
        assert_eq!(manager.get_component::<i32>(entity), Some(&1));
    }

    #[test]
    fn remove_entity_with_component() {
        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, 1);
        manager.delete_entity(entity);
        assert_eq!(manager.get_component::<i32>(entity), None);
        assert_eq!(manager.component_store.len(), 0);
    }

    #[test]
    fn add_struct_component() {
        #[derive(Debug, PartialEq)]
        struct Component {
            value: i32,
        }

        let mut manager = EntityComponentManager::default();
        let entity = manager.create_entity();
        manager.insert_component(entity, Component { value: 1 });
        assert_eq!(
            manager.get_component::<Component>(entity),
            Some(&Component { value: 1 })
        );
    }
}
