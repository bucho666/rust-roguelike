use std::any::Any;
use std::collections::HashMap;

pub type EntityId = u64;

pub struct EntitySystem {
    entities: HashMap<EntityId, Box<Any>>,
    last_id: EntityId,
}

impl EntitySystem {
    pub fn new() -> Self {
        EntitySystem {
            entities: HashMap::new(),
            last_id: 1,
        }
    }

    pub fn register<T: Any>(&mut self, entity: T) -> EntityId {
        let id = self.last_id;
        self.entities.insert(id, Box::new(entity));
        self.last_id += 1;
        id
    }

    //pub fn of<T: Any>(&self, id: EntityId) -> &T {
    //    self.entities[&id].downcast_ref().unwrap()
    //}

    pub fn of_mut<T: Any>(&mut self, id: EntityId) -> &mut T {
        self.entities.get_mut(&id).unwrap().downcast_mut().unwrap()
    }
}
