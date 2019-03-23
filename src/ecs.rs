use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::default::Default;

pub type EntityId = u64;
type Component = HashMap<TypeId, Box<Any>>;

#[allow(dead_code)] // TODO
#[derive(Default)]
pub struct Ecs {
    next_entity: EntityId,
    entities: HashMap<EntityId, Component>,
}

#[allow(dead_code)] // TODO
impl Ecs {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_entity(&mut self) -> EntityId {
        self.next_entity += 1;
        self.entities.insert(self.next_entity, Default::default());
        self.next_entity
    }

    pub fn exists_entity(&self, entity: EntityId) -> bool {
        self.entities.contains_key(&entity)
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        self.entities.remove(&entity);
    }

    pub fn set<C: Any>(&mut self, entity: EntityId, value: C) {
        self.component_mut::<C>(entity)
            .insert(TypeId::of::<C>(), Box::new(value))
            .map(|old| *old.downcast::<C>().expect("down cast error"));
    }

    pub fn has<C: Any>(&self, entity: EntityId) -> bool {
        self.component::<C>(entity).contains_key(&TypeId::of::<C>())
    }

    pub fn of<C: Any>(&self, entity: EntityId) -> &C {
        self.component::<C>(entity)
            .get(&TypeId::of::<C>())
            .expect("get component error")
            .downcast_ref()
            .expect("downcast_ref error")
    }

    pub fn of_mut<C: Any>(&mut self, entity: EntityId) -> &mut C {
        self.component_mut::<C>(entity)
            .get_mut(&TypeId::of::<C>())
            .expect("get mut component error")
            .downcast_mut()
            .expect("downcast_mut error")
    }

    pub fn remove<C: Any>(&mut self, entity: EntityId) {
        self.component_mut::<C>(entity).remove(&TypeId::of::<C>());
    }

    fn component<C: Any>(&self, entity: EntityId) -> &Component {
        self.entities.get(&entity).expect("Entity does not exists")
    }

    fn component_mut<C: Any>(&mut self, entity: EntityId) -> &mut Component {
        self.entities
            .get_mut(&entity)
            .expect("Entity mut does not exists")
    }
}

#[cfg(test)]
mod ecs_test {
    use super::*;
    #[test]
    fn create_entity() {
        let mut component = Ecs::new();
        assert_eq!(component.new_entity(), 1);
        assert_eq!(component.new_entity(), 2);
        assert_eq!(component.new_entity(), 3);
    }

    #[test]
    fn has_entity() {
        let mut component = Ecs::new();
        assert!(component.exists_entity(1) == false);
        let e = component.new_entity();
        assert!(component.exists_entity(e));
    }

    #[test]
    fn remove_entity() {
        let mut component = Ecs::new();
        let e = component.new_entity();
        component.remove_entity(e);
        assert!(component.exists_entity(e) == false);
    }

    #[test]
    fn component() {
        struct Name {
            name: String,
        }
        struct Age {
            age: u8,
        }
        let mut component = Ecs::new();
        let man = component.new_entity();
        assert!(component.has::<Age>(man) == false);
        component.set(man, Age { age: 24 });
        component.set(
            man,
            Name {
                name: "bob".to_string(),
            },
        );
        assert!(component.has::<Age>(man));
        assert!(component.has::<Name>(man));
        assert_eq!(component.of::<Age>(man).age, 24);
        assert_eq!(component.of::<Name>(man).name, "bob");
        component.of_mut::<Age>(man).age = 16;
        assert_eq!(component.of::<Age>(man).age, 16);
        component.remove::<Age>(man);
        assert_eq!(component.has::<Age>(man), false);
    }
}
