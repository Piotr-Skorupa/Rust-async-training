use std::sync::{ Arc, Mutex };
use engine::Engine;
use fuel_tank::FuelTank;
use tokio::task::JoinError;

mod engine;
mod fuel_tank;

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    println!("Filling the fuel tank...");

    let tank = Arc::new(Mutex::new(FuelTank::default()));
    let small_engine = Engine::new("Small".to_owned(), 9, &tank);
    let big_engine = Engine::new("Big".to_owned(), 15, &tank);

    {
        let lock = tank.lock().unwrap();
        lock.state();
        println!("Starting 2 engines!");
    }

    let handle = tokio::spawn(async move{
        small_engine.run().await;
    });

    big_engine.run().await;
    let result = handle.await;
    if result.is_err() {
        return result;
    }

    println!("end");
    Ok(())
}
