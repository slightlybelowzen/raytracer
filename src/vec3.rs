#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
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
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            n => panic!("cannot access vector index for n = {}", n),
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

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
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
    fn test_operations() {
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
        // indexing
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 1.0);
    }
}
