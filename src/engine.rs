use std::{sync::{Arc, Mutex}, time::Duration};
use tokio::time::sleep;

use crate::fuel_tank;


#[derive(Debug)]
pub struct Engine {
    name_: String,
    usage_: i32,
    fuel_tank_: Arc<Mutex<fuel_tank::FuelTank>>,
}

impl Engine {
    pub fn new(name: String, usage: i32, arc: &Arc<Mutex<fuel_tank::FuelTank>>) -> Self {
        Engine { name_: name, usage_: usage, fuel_tank_: Arc::clone(&arc) }
    }

    pub async fn run(&self) {
        println!("[{}] Starting engine...", self.name_);
        while self.take_fuel() {
            println!("[{}] Working...", self.name_);
            sleep(Duration::from_secs(1)).await;
        }

        println!("[{}] Turning off engine.", self.name_);
    }

    fn take_fuel(&self) -> bool {
        let mut lock = self.fuel_tank_.lock().unwrap();
        match lock.get_fuel(self.usage_) {
            Some(val) => {
                println!("[{}] Take {} liters of fuel from tank", self.name_, val);
                lock.state();
                true
            },
            None => { false }
        }
    }
}