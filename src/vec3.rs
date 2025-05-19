use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

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
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn length(&self) -> f64 {
        self.length_sqaured().sqrt()
    }
    pub fn length_sqaured(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot_product(&self, rhs: Self) -> f64 {
        (0..3).map(|i| self[i] * rhs[i]).sum()
    }
    pub fn cross_product(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
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

// immutable vector operations
impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// scalar immutable operations
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// mutable vector operations
impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        for i in 0..3 {
            self[i] += rhs[i]
        }
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self[i] *= rhs
        }
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_newialisers() {
        let default_vec = Vec3::default();
        assert_eq!(
            default_vec,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );
        let new_vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            new_vec,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_immutable_vector_ops() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
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
    fn test_immutable_scalar_ops() {
        let unit_vector = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
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
        // division
        assert_eq!(b / 2.0, unit_vector);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        a += b;
        assert_eq!(
            a,
            Vec3 {
                x: 3.0,
                y: 3.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_mul_assign_with_f64_rhs() {
        let mut a = Vec3::new(1.0, 1.0, 1.0);
        a *= 5.0;
        assert_eq!(
            a,
            Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0
            }
        );
    }

    #[test]
    fn test_div_assign_with_f64_rhs() {
        let mut a = Vec3::new(1.0, 1.0, 1.0);
        a /= 5.0;
        assert_eq!(
            a,
            Vec3 {
                x: 0.2,
                y: 0.2,
                z: 0.2
            }
        );
    }
}
