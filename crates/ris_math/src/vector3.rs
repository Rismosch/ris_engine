#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const ZERO: Vector3 = Vector3 {
    x: 0.,
    y: 0.,
    z: 0.,
};
pub const ONE: Vector3 = Vector3 {
    x: 1.,
    y: 1.,
    z: 1.,
};
pub const RIGHT: Vector3 = Vector3 {
    x: 1.,
    y: 0.,
    z: 0.,
};
pub const LEFT: Vector3 = Vector3 {
    x: -1.,
    y: 0.,
    z: 0.,
};
pub const FORWARD: Vector3 = Vector3 {
    x: 0.,
    y: 1.,
    z: 0.,
};
pub const BACKWARD: Vector3 = Vector3 {
    x: 0.,
    y: -1.,
    z: 0.,
};
pub const UP: Vector3 = Vector3 {
    x: 0.,
    y: 0.,
    z: 1.,
};
pub const DOWN: Vector3 = Vector3 {
    x: 0.,
    y: 0.,
    z: -1.,
};

impl Vector3 {
    // initialization
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x,y,z}
    }
    
    // utility
    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    /// Think of `a` as the thumb of your right hand and of `b` as the index finger of your right
    /// hand. The result will be the middle finger.
    pub fn cross(a: Vector3, b: Vector3) -> Self {
        Self {
            x: a.y * b.z - b.y * a.z,
            y: a.z * b.x - b.z * a.x,
            z: a.x * b.y - b.x * a.y,
        }
    }

    pub fn normalized(self) -> Self {
        let magnitude = self.magnitude();
        if magnitude < super::MIN_NORM {
            ZERO
        } else {
            Self {
                x: self.x / magnitude,
                y: self.y / magnitude,
                z: self.z / magnitude,
            }
        }
    }

    pub fn magnitude_squared(self) -> f32 {
        Self::dot(self, self)
    }

    pub fn magnitude(self) -> f32 {
        super::sqrt(self.magnitude_squared())
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
