use std::fmt;
use std::ops;

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

// Print
impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}

// Add Vector2 + Vector2
impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, _rhs: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}

// Sub Vector2 - Vector2
impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

// Scale Vector2 * f32
impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, _rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

#[allow(dead_code)]
impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn add_xy(mut self, x: f32, y: f32) -> Vector2 {
        self.x += x;
        self.y += y;
        self
    }

    pub fn length(self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y))
    }

    pub fn normalize(self) -> Vector2 {
        let l = self.length();
        if l == 0.0 {
            return Vector2 { x: 0.0, y: 0.0 };
        }
        Vector2 {
            x: self.x / l,
            y: self.y / l,
        }
    }

    pub fn dot(self, v2: Vector2) -> f32 {
        self.x * v2.x + self.y * v2.y
    }

    pub fn distance_to(self, to: Vector2) -> f32 {
        let a = self.x - to.x;
        let b = self.y - to.y;
        f32::sqrt(a * a + b * b)
    }
}
