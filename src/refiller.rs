use std::sync::{Arc, Mutex};

use crate::fuel_tank;


pub struct Refiller {
    volume_: i32,
    fuel_tank_: Arc<Mutex<fuel_tank::FuelTank>>
}

#[derive(PartialEq)]
pub enum RefillerError {
    NoFuel
}

impl Refiller {
    pub fn new(volume: i32, arc: &Arc<Mutex<fuel_tank::FuelTank>>) -> Self {
        Refiller { volume_: volume, fuel_tank_: Arc::clone(&arc) }
    }

    pub fn is_empty(&self) -> bool {
        self.volume_ <= 0
    }

    pub fn refill(&mut self) -> Result<(), RefillerError> {
        let mut tank_lock = self.fuel_tank_.lock().unwrap();
        if !tank_lock.is_empty() {
            return Ok(());
        }

        let portion = tank_lock.capacity();
        if portion <= self.volume_ {
            tank_lock.fill(portion);
            self.volume_ -= portion;
            println!("Refilling fuel tank...");
            tank_lock.state();
            println!("{} liters stays in refiller.", self.volume_);
            return Ok(());
        }

        Err(RefillerError::NoFuel)
    }
}