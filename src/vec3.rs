use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

type Point3 = Vec3;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn init(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn length(&self) -> f32 {
        self.length_sqaured().sqrt()
    }
    pub fn length_sqaured(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            n => panic!("cannot access vector for index: {}", n),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            n => panic!("cannot access vector for index: {}", n),
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        for i in 0..3 {
            self[i] += rhs[i]
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..3 {
            self[i] *= rhs
        }
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_initialisers() {
        let default_vec = Vec3::default();
        assert_eq!(
            default_vec,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );
        let init_vec = Vec3::init(1.0, 2.0, 3.0);
        assert_eq!(
            init_vec,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_immutable_ops() {
        let a = Vec3::init(1.0, 1.0, 1.0);
        let b = Vec3::init(2.0, 2.0, 2.0);
        // negation
        assert_eq!(
            -a,
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0
            }
        );
        // addition
        assert_eq!(
            a + b,
            Vec3 {
                x: 3.0,
                y: 3.0,
                z: 3.0
            }
        );
        // subtraction
        assert_eq!(
            b - a,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
        // multiplication
        assert_eq!(
            a * b,
            Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        // indexing
        for i in 0..3 {
            assert_eq!(a[i], 1.0);
            assert_eq!(b[i], 2.0);
        }
    }

    #[test]
    fn test_scalar_ops() {
        let a = Vec3::init(1.0, 1.0, 1.0);
        let b = Vec3::init(2.0, 2.0, 2.0);
        // multiplication
        assert_eq!(
            b * 5.0,
            Vec3 {
                x: 10.0,
                y: 10.0,
                z: 10.0
            }
        );
        // should be commutative
        assert_eq!(
            5.0 * b,
            Vec3 {
                x: 10.0,
                y: 10.0,
                z: 10.0
            }
        );
    }

    #[test]
    fn test_mutable_ops() {
        let mut a = Vec3::init(1.0, 1.0, 1.0);
        let mut b = Vec3::init(2.0, 2.0, 2.0);
        a += b;
        assert_eq!(
            a,
            Vec3 {
                x: 3.0,
                y: 3.0,
                z: 3.0
            }
        )
    }
}
