use std::num::Wrapping;

#[derive(Copy, Clone, Default)]
#[allow(overflowing_literals, arithmetic_overflow)]
pub struct Random {
    pub seed: i32,
}

#[allow(dead_code)]
impl Random {
    pub fn new(seed: i32) -> Random {
        Random { seed }
    }

    pub fn int(&mut self) -> i32 {
        self.seed = (Wrapping(self.seed) + Wrapping(0x6D2B79F5)).0;
        let mut t = self.seed;
        t = (Wrapping(t ^ t >> 15) * Wrapping(t | 1)).0;
        let t2 = (Wrapping(t) + Wrapping(t ^ t >> 7) * Wrapping(t | 31)).0;
        t = (Wrapping(t) ^ Wrapping(t2)).0;
        Wrapping((t ^ t >> 14) >> 0).0
    }

    pub fn float(&mut self) -> f32 {
        let v = self.int();
        let v1 = v as f64;
        let v2 = <i32>::max_value() as f64;
        return (v1 / v2) as f32;
    }

    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        min + self.float() * (max - min)
    }
}
