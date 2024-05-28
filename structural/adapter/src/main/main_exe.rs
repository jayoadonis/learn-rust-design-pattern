
#![allow(unused)]
#![allow(dead_code)]

use std::time::Instant;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::time::Duration;
use rand::{ Rng, rngs::ThreadRng };

mod util;
mod model;
use crate::model::adapater_into_ii_pin::AdapterIntoIIPin;
use crate::model::entity::Entity;
use crate::model::i_three_pin::IThreePin;
use crate::model::i_two_pin::ITwoPin;
use crate::model::plug_i::PlugI;
use crate::model::plug_ii::PlugII;

pub fn main() -> Result<(), usize> {
    println!("Adapter DP.");

    let old_plug: Box<dyn ITwoPin> = Box::new(PlugI::default());
    let new_plug: Box<dyn IThreePin> = Box::new(PlugII::default());
    let adap_plug: Box<dyn ITwoPin> = Box::new(AdapterIntoIIPin::new(new_plug));

    let mut rng: ThreadRng = rand::thread_rng();

    let mut entities: Vec<Entity> = Vec::with_capacity(1_000_000);
    for _ in 0..1_000{
        let id: String = 
            (0..8).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();
        let name: String = 
            (0..12).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();
        let entity: Entity = 
            Entity::new_with_id_and_name(Some(&id), Some(&name));
        entities.push(entity);
    }

    let start: Instant = Instant::now();
    let start_system_time: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    
    for (i, entity)  in entities.iter().enumerate() {
        println!("Entity {}: {}", i + 1, entity.to_string());
    }

    let duration: Duration = start.elapsed();
    let end_system_time: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    println!("Time taken: {:?}", duration);
    println!("Start timestamp (seconds since UNIX epoch): {:?}", start_system_time);
    println!("End timestamp (seconds since UNIX epoch): {:?}", end_system_time);

    return Ok(());
}