use std::sync::{ Arc, Mutex };
use crate::fuel_tank;

pub struct Refiller {
    capacity_: i32,
    volume_: i32,
    fuel_tank_: Arc<Mutex<fuel_tank::FuelTank>>
}

pub enum RefillerError {
    NoFuel
}

impl Refiller {
    pub fn new(capacity: i32, arc: &Arc<Mutex<fuel_tank::FuelTank>>) -> Self {
        Refiller { capacity_: capacity, volume_: capacity, fuel_tank_: Arc::clone(&arc) }
    }

    pub fn refill(&mut self) -> Result<(), RefillerError> {
        let mut lock = self.fuel_tank_.lock().unwrap();
        let portion = lock.capacity();
        if portion <= self.volume_ {
            lock.fill(portion);
            self.volume_ -= portion;
            return Ok(());
        }

        Err(RefillerError::NoFuel)
    }
}