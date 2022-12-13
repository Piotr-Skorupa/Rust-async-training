use std::{sync::{Arc, Mutex}, time::Duration};
use tokio::{task::JoinError, time::sleep};

use engine::Engine;
use fuel_tank::FuelTank;
use refiller::{Refiller, RefillerError};

mod engine;
mod fuel_tank;
mod refiller;


#[tokio::main]
async fn main() -> Result<(), JoinError> {
    let tank = Arc::new(Mutex::new(FuelTank::new(100)));
    let refiller = Arc::new(Mutex::new(Refiller::new(1000, &tank)));
    let small_engine = Engine::new("Small".to_owned(), 9, &tank);
    let big_engine = Engine::new("Big".to_owned(), 15, &tank);
    let mut thread_handles = Vec::new();

    println!("Filling the fuel tank...");
    tank.lock().unwrap().state();
    println!("Starting 2 engines!");


    let mut refiller_clone = Arc::clone(&refiller);
    thread_handles.push(tokio::spawn(async move{
        while !refiller_clone.lock().unwrap().is_empty() {
            small_engine.run().await;
            sleep(Duration::from_secs(1)).await;
        }
    }));

    refiller_clone = Arc::clone(&refiller);
    thread_handles.push(tokio::spawn(async move{
        while !refiller_clone.lock().unwrap().is_empty() {
            big_engine.run().await;
            sleep(Duration::from_secs(1)).await;
        }
    }));

    thread_handles.push(tokio::spawn(async move{
        while refiller.lock().unwrap().refill() != Err(RefillerError::NoFuel) {
            sleep(Duration::from_millis(10)).await;
        }
    }));

    for handle in thread_handles {
        match handle.await {
            Ok(_) => {},
            Err(err) => { eprintln!("{:?}", err); return Err(err); }
        }
    }

    println!("[END] No fuel. All engines has been stopped.");
    Ok(())
}
