use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.r, self.g, self.b)
    }
}

impl Color {
    pub fn default() -> Self {
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

// impl std::ops::Index<usize> for Color {
//     type Output = i32;
//     fn index(&self, index: usize) -> &Self::Output {
//         match index {
//             0 => &self.r,
//             1 => &self.g,
//             2 => &self.b,
//             n => panic!("cannot access vector for index: {}", n),
//         }
//     }
// }

// impl std::ops::IndexMut<usize> for Color {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         match index {
//             0 => &mut self.r,
//             1 => &mut self.g,
//             2 => &mut self.b,
//             n => panic!("cannot access vector for index: {}", n),
//         }
//     }
// }

// // immutable vector operations
// impl std::ops::Neg for Color {
//     type Output = Color;
//     fn neg(self) -> Self::Output {
//         Self {
//             x: -self.r,
//             y: -self.g,
//             z: -self.b,
//         }
//     }
// }

// impl std::ops::Add<Color> for Color {
//     type Output = Color;
//     fn add(self, rhs: Color) -> Self::Output {
//         Self {
//             x: self.r + rhs.x,
//             y: self.g + rhs.y,
//             z: self.b + rhs.z,
//         }
//     }
// }

// impl std::ops::Sub<Color> for Color {
//     type Output = Color;
//     fn sub(self, rhs: Color) -> Self::Output {
//         Self {
//             x: self.r - rhs.x,
//             y: self.g - rhs.y,
//             z: self.b - rhs.z,
//         }
//     }
// }

// impl std::ops::Mul<Color> for Color {
//     type Output = Color;
//     fn mul(self, rhs: Color) -> Self::Output {
//         Self {
//             x: self.r * rhs.x,
//             y: self.g * rhs.y,
//             z: self.b * rhs.z,
//         }
//     }
// }

// // scalar immutable operations
// impl std::ops::Div<i32> for Color {
//     type Output = Color;
//     fn div(self, rhs: i32) -> Self::Output {
//         self * (1.0 / rhs)
//     }
// }

// impl std::ops::Mul<i32> for Color {
//     type Output = Color;
//     fn mul(self, rhs: i32) -> Self::Output {
//         Self {
//             x: self.r * rhs,
//             y: self.g * rhs,
//             z: self.b * rhs,
//         }
//     }
// }

// impl std::ops::Mul<Color> for i32 {
//     type Output = Color;
//     fn mul(self, rhs: Color) -> Self::Output {
//         rhs * self
//     }
// }

// // mutable vector operations
// impl std::ops::AddAssign<Color> for Color {
//     fn add_assign(&mut self, rhs: Color) {
//         for i in 0..3 {
//             self[i] += rhs[i]
//         }
//     }
// }

// impl std::ops::MulAssign<i32> for Color {
//     fn mul_assign(&mut self, rhs: i32) {
//         for i in 0..3 {
//             self[i] *= rhs
//         }
//     }
// }

// impl std::ops::DivAssign<i32> for Color {
//     fn div_assign(&mut self, rhs: i32) {
//         *self *= 1.0 / rhs
//     }
// }
