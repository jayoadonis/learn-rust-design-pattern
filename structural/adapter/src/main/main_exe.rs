
#![allow(unused)]
#![allow(dead_code)]

mod util;
mod model;
use crate::model::entity::Entity;

pub fn main() -> Result<(), usize> {
    println!("Adapter DP.");
    
    let entity1 = Entity::new();
    let entity2 = Entity::new_with_id(Some("123"));
    let entity3 = Entity::new_with_id_and_name(Some("456"), Some("Entity Name"));

    println!("Entity 1: {}", entity1);
    println!("Entity 2: {:?}", entity2);
    println!("Entity 3: {}", entity3);

    let entity4 = Entity::from_entity(&entity3);
    println!("Entity 4 (copy of Entity 3): {:?}", entity4);

    return Ok(());
}