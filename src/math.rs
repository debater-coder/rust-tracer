use std::ops;

#[derive(Debug)]
pub struct Vector3(f64, f64, f64);

impl Vector3 {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3(-self.0, -self.1, -self.2)
    }
}

pub type Point3 = Vector3;
pub type Color = Vector3;

#[cfg(test)]
mod tests {
    use crate::math::*;

    #[test]
    fn vector_eq() {
        assert_eq!(Vector3(1.0, 2.0, 3.0), Vector3(1.0, 2.0, 3.0));
    }

    #[test]
    fn vector_add() {
        assert_eq!(
            Vector3(1.0, 2.0, 3.0) + Vector3(4.0, 5.0, 6.0),
            Vector3(5.0, 7.0, 9.0)
        );
    }

    #[test]
    fn vector_add_assign() {
        let mut v = Vector3(1.0, 2.0, 3.0);
        v += Vector3(4.0, 5.0, 6.0);

        assert_eq!(v, Vector3(5.0, 7.0, 9.0));
    }

    #[test]
    fn vector_sub() {
        assert_eq!(
            Vector3(1.0, 2.0, 3.0) - Vector3(4.0, 5.0, 6.0),
            Vector3(-3.0, -3.0, -3.0)
        );
    }

    #[test]
    fn vector_sub_assign() {
        let mut v = Vector3(1.0, 2.0, 3.0);
        v -= Vector3(4.0, 5.0, 6.0);

        assert_eq!(v, Vector3(-3.0, -3.0, -3.0));
    }

    #[test]
    fn vector_div() {
        assert_eq!(Vector3(1.0, 2.0, 3.0) / 2.0, Vector3(0.5, 1.0, 1.5));
    }

    #[test]
    fn vector_div_assign() {
        let mut v = Vector3(1.0, 2.0, 3.0);
        v /= 2.0;

        assert_eq!(v, Vector3(0.5, 1.0, 1.5));
    }

    #[test]
    fn vector_mul() {
        assert_eq!(Vector3(1.0, 2.0, 3.0) * 2.0, Vector3(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector_mul_assign() {
        let mut v = Vector3(1.0, 2.0, 3.0);
        v *= 2.0;

        assert_eq!(v, Vector3(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector_neg() {
        assert_eq!(-Vector3(1.0, 2.0, 3.0), Vector3(-1.0, -2.0, -3.0));
    }
}
