use std::any::{Any, TypeId};
use std::collections::HashMap;

type EntityIdNumber = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId {
    number: EntityIdNumber,
    type_id: TypeId,
}

impl EntityId {
    pub fn new(number: EntityIdNumber, type_id: TypeId) -> Self {
        EntityId {
            number: number,
            type_id: type_id,
        }
    }

    pub fn number(&self) -> EntityIdNumber {
        self.number
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}

pub struct EntitySystem {
    entities: HashMap<EntityId, Box<Any>>,
    last_id_number: EntityIdNumber,
}

impl EntitySystem {
    pub fn new() -> Self {
        EntitySystem {
            entities: HashMap::new(),
            last_id_number: 1,
        }
    }

    pub fn register<T: Any>(&mut self, entity: T) -> EntityId {
        let id = EntityId::new(self.last_id_number, TypeId::of::<T>());
        self.entities.insert(id, Box::new(entity));
        self.last_id_number += 1;
        id
    }

    pub fn of<T: Any>(&self, id: EntityId) -> &T {
        self.entities[&id].downcast_ref().unwrap()
    }

    pub fn of_mut<T: Any>(&mut self, id: EntityId) -> &mut T {
        self.entities.get_mut(&id).unwrap().downcast_mut().unwrap()
    }
}
