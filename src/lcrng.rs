pub struct LCRNG {
    seed: u32,
    multiplier: u64,
    adder: u64,
}

impl LCRNG {
    pub fn new_emerald(seed: u32) -> Self {
        Self {
            seed,
            multiplier: 0x41C64E6D,
            adder: 0x6073,
        }
    }

    pub fn next_u16(&mut self) -> u16 {
        self.seed = (u64::from(self.seed) * self.multiplier + self.adder) as u32;
        (self.seed >> 16) as u16
    }

    pub fn next_u32(&mut self) -> u32 {
        self.seed = (u64::from(self.seed) * self.multiplier + self.adder) as u32;
        self.seed
    }
}
