use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn new_colour(r: f32, g: f32, b: f32) -> Self {
        Tuple::new_vector(r, g, b)
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn r(&self) -> f32 {
        self.x
    }

    pub fn g(&self) -> f32 {
        self.y
    }

    pub fn b(&self) -> f32 {
        self.z
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        *self / magnitude
    }

    fn dot(&self, rhs: &Tuple) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    fn cross(&self, rhs: &Tuple) -> Self {
        Tuple::new_vector(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }
}

const EPSILON: f32 = 0.00001;

fn eq_f32(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        eq_f32(self.x, other.x)
            && eq_f32(self.y, other.y)
            && eq_f32(self.z, other.z)
            && eq_f32(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_point() {
        assert!(Tuple::new(0.0, 0.0, 0.0, 1.0).is_point());
        assert!(!Tuple::new(0.0, 0.0, 0.0, 0.0).is_point());
    }

    #[test]
    fn test_is_vector() {
        assert!(!Tuple::new(0.0, 0.0, 0.0, 1.0).is_vector());
        assert!(Tuple::new(0.0, 0.0, 0.0, 0.0).is_vector());
    }

    #[test]
    fn adding_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(
            a1 + a2,
            Tuple::new(1.0, 1.0, 6.0, 1.0),
            "Testing addition of {:?} + {:?}",
            a1,
            a2
        );
    }

    #[test]
    fn subtraction() {
        //Subtracting points
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);
        assert_eq!(
            p1 - p2,
            Tuple::new_vector(-2.0, -4.0, -6.0),
            "Testing subtraction of points {:?} - {:?}",
            p1,
            p2
        );

        //Subtracting point and vectors
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let v1 = Tuple::new_vector(5.0, 6.0, 7.0);
        assert_eq!(
            p1 - v1,
            Tuple::new_point(-2.0, -4.0, -6.0),
            "Testing point - vector"
        );

        //Subtracting two vectors
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
        assert_eq!(
            v1 - v2,
            Tuple::new_vector(-2.0, -4.0, -6.0),
            "Testing vector - vector = vector"
        );
    }

    #[test]
    fn negation() {
        let t1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-t1, Tuple::new(-1.0, 2.0, -3.0, 4.0))
    }

    #[test]
    fn multiplication() {
        let t1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(t1 * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
        assert_eq!(t1 * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn division() {
        let t1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(t1 / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude() {
        assert_eq!(Tuple::new_vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(
            Tuple::new_vector(1.0, 2.0, 3.0).magnitude(),
            f32::sqrt(14.0)
        );
        assert_eq!(
            Tuple::new_vector(-1.0, -2.0, -3.0).magnitude(),
            f32::sqrt(14.0)
        );
    }

    #[test]
    fn normalize() {
        assert_eq!(
            Tuple::new_vector(4.0, 0.0, 0.0).normalize(),
            Tuple::new_vector(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Tuple::new_vector(1.0, 2.0, 3.0).normalize(),
            Tuple::new_vector(0.26726, 0.53452, 0.80178)
        );
        assert!(eq_f32(
            Tuple::new_vector(1.0, 2.0, 3.0).normalize().magnitude(),
            1.0
        ))
    }

    #[test]
    fn dot() {
        let t1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let t2 = Tuple::new_vector(2.0, 3.0, 4.0);

        assert!(eq_f32(t1.dot(&t2), 20.0))
    }

    #[test]
    fn cross() {
        let t1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let t2 = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(t1.cross(&t2), Tuple::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(t2.cross(&t1), Tuple::new_vector(1.0, -2.0, 1.0));
    }
}
