
#[derive(Debug)]
pub struct FuelTank {
    capacity_: i32,
    volume_: i32
}

impl Default for FuelTank {
    fn default() -> Self {
        FuelTank { capacity_: 100, volume_: 100 }
    }
}

impl FuelTank {
    pub fn new(capacity: i32) -> Self {
        FuelTank { capacity_: capacity, volume_: capacity }
    }

    pub fn is_empty(&self) -> bool {
        self.volume_ <= 0
    }

    pub fn capacity(&self) -> i32 {
        self.capacity_
    }

    pub fn state(&self) {
        println!("Fuel availability {}/100 liters", self.volume_);
    }

    pub fn get_fuel(&mut self, value: i32) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        self.volume_ -= value;
        if self.volume_ >= 0 {
            return Some(value);
        }

        let fuel_tip = value + self.volume_;
        self.volume_ = 0;
        Some(fuel_tip)
    }

    pub fn fill(&mut self, value: i32) {
        self.volume_ += value;
    }
}