
#[derive(Debug)]
pub struct FuelTank {
    volume_: i32
}

impl Default for FuelTank {
    fn default() -> Self {
        FuelTank { volume_: 100 }
    }
}

impl FuelTank {
    pub fn is_empty(&self) -> bool {
        self.volume_ <= 0
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

        Some(value + self.volume_)
    }
}