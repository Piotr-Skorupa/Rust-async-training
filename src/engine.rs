use std::{sync::{ Arc, Mutex }, time::Duration};
use tokio::time::sleep;

use crate::fuel_tank;


#[derive(Debug)]
pub struct Engine {
    fuel_tank_: Arc<Mutex<fuel_tank::FuelTank>>,
    capacity_: i32,
    name_: String
}

impl Engine {
    pub fn new(name: String, capacity: i32, arc: &Arc<Mutex<fuel_tank::FuelTank>>) -> Self {
        Engine { name_: name, capacity_: capacity, fuel_tank_: Arc::clone(&arc) }
    }

    pub async fn run(&self) {
        while self.take_fuel() {
            println!("[{}] Working...", self.name_);
            sleep(Duration::from_secs(1)).await;
        }

        println!("[{}] Turning off engine.", self.name_);
    }

    fn take_fuel(&self) -> bool {
        let mut lock = self.fuel_tank_.lock().unwrap();
        match lock.get_fuel(self.capacity_) {
            Some(val) => {
                println!("[{}] Take {} liters of fuel from tank", self.name_, val);
                lock.state();
                true
            },
            None => { false }
        }
    }
}