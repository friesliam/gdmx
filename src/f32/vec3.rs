use crate::{
    F32Ext,
};
use std::{
    fmt::{
        self,
        Debug,
        Display
    },
    cmp::{
        Ordering,
    },
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign,
        Mul,
        MulAssign,
        Div,
        DivAssign,
        Rem,
        RemAssign,
        Neg,
        Index,
        IndexMut,
    },
};


/// A vector in 3-space
#[derive(Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// The default Vec3 with all 0's
    pub const ZERO: Vec3 = Vec3::splat(0.0);

    /// The positive x-axis basis vector
    pub const X: Vec3 = Vec3::new(1.0, 0.0, 0.0);

    /// The positive y-axis basis vector
    pub const Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    /// The positive z-axis basis vector
    pub const Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    /// The negative x-axis basis vector
    pub const NEG_X: Vec3 = Vec3::new(-1.0, 0.0, 0.0);

    /// The negative y-axis basis vector
    pub const NEG_Y: Vec3 = Vec3::new(0.0, -1.0, 0.0);

    /// The negative z-axis basis vector
    pub const NEG_Z: Vec3 = Vec3::new(0.0, 0.0, -1.0);


    /// Standard constructor for <x y z>
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[inline]
    pub const fn splat(v: f32) -> Vec3 {
        Vec3::new(v, v, v)
    }

    /// Creates a flattened forward Vec3 from yaw
    #[inline]
    pub fn forward_from_yaw(yaw: f32) -> Vec3 {
        let (sin_yaw, cos_yaw) = yaw.sin_cos();
        Vec3::new(sin_yaw, 0.0, cos_yaw)
    }

    /// Creates a flattened right Vec3 from yaw
    #[inline]
    pub fn right_from_yaw(yaw: f32) -> Vec3 {
        let (sin_yaw, cos_yaw) = yaw.sin_cos();
        Vec3::new(cos_yaw, 0.0, -sin_yaw)
    }

    /// Creates a flattened forward and right vector from yaw
    #[inline]
    pub fn forward_and_right_from_yaw(yaw: f32) -> (Vec3, Vec3) {
        let (sin_yaw, cos_yaw) = yaw.sin_cos();
        let forward = Vec3::new(sin_yaw, 0.0, cos_yaw);
        let right = Vec3::new(cos_yaw, 0.0, -sin_yaw);
        (forward, right)
    }


    /// Normalizes a Vec3 so its magnitute is 1.0
    /// Requires: self must not be of magnitude ~zero
    #[inline]
    pub fn normalize(self) -> Vec3 {
        self * self.length_recip()
    }

    /// Returns the normalized vector and its previous magnitute
    /// Requires: self must not be of magnitude ~zero
    #[inline]
    pub fn normalize_and_length(self) -> (Vec3, f32) {
        let length = self.length();
        (self / length, length)
    }

    /// Transforms a local-space Vec3 into world-space
    /// Requires: right, up, and forward should all be normalized
    #[inline]
    pub fn to_world(self, right: Vec3, up: Vec3, forward: Vec3) -> Vec3 {
        right * self.x + up * self.y + forward * self.z
    }

    /// Computes the sum of each Vec3 element
    #[inline]
    pub fn sum(self) -> f32 {
        self.x + self.y + self.z
    }

    /// Computes the dot product of two Vec3s
    #[inline]
    pub fn dot(self, rhs: Vec3) -> f32 {
        (self * rhs).sum()
    }

    /// Computes the cross product of two Vec3s
    #[inline]
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - rhs.y * self.z,
            self.z * rhs.x - rhs.z * self.x,
            self.x * rhs.y - rhs.x * self.y,
        )
    }

    /// Returns the Vec3 with the min for each element pair
    #[inline]
    pub fn min(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x.min(rhs.x),
            self.y.min(rhs.y),
            self.z.min(rhs.z),
        )
    }

    /// Returns the Vec3 with the max for each element pair
    #[inline]
    pub fn max(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x.max(rhs.x),
            self.y.max(rhs.y),
            self.z.max(rhs.z),
        )
    }

    /// Clamps a Vec3 so that each value is between the appropriate min and max element
    /// Requires: min < max
    #[inline]
    pub fn clamp(self, min: Vec3, max: Vec3) -> Vec3 {
        self.min(max).max(min)
    }

    /// Clamps the x value of Vec3
    /// Requires: min < max
    #[inline]
    pub fn clamp_x(self, min: f32, max: f32) -> Vec3 {
        Vec3::new(
            self.x.clamp(min, max),
            self.y,
            self.z,
        )
    }

    /// Clamps the y value of Vec3
    /// Requires: min < max
    #[inline]
    pub fn clamp_y(self, min: f32, max: f32) -> Vec3 {
        Vec3::new(
            self.x,
            self.y.clamp(min, max),
            self.z,
        )
    }

    /// Clamps the z value of Vec3
    /// Requires: min < max
    #[inline]
    pub fn clamp_z(self, min: f32, max: f32) -> Vec3 {
        Vec3::new(
            self.x,
            self.y,
            self.z.clamp(min, max),
        )
    }

    /// Computes an absolute value on each element
    #[inline]
    pub fn abs(self) -> Vec3 {
        Vec3::new(
            self.x.abs(),
            self.y.abs(),
            self.z.abs(),
        )
    }

    /// Computes the distance between a Vec3 and Vec3::ZERO
    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Computes the reciprocal of the distance between a Vec3 and Vec3::ZERO
    /// Requires: magnitude of self is not zero
    #[inline]
    pub fn length_recip(self) -> f32 {
        self.dot(self).rsqrt()
    }

    /// Computes the distance squared between a Vec3 and Vec3::ZERO
    #[inline]
    pub fn length_2(self) -> f32 {
        self.dot(self)
    }

    /// Computes the distance between one Vec3 and another
    #[inline]
    pub fn distance(self, rhs: Vec3) -> f32 {
        (self - rhs).length()
    }

    /// Computes the reciprocal of the distance between one Vec3 and another
    /// Requires: self != rhs
    #[inline]
    pub fn distance_recip(self, rhs: Vec3) -> f32 {
        (self - rhs).length_recip()
    }

    /// Computes the distance squared between one Vec3 and another
    #[inline]
    pub fn distance_2(self, rhs: Vec3) -> f32 {
        (self - rhs).length_2()
    }

    /// Linear Interpolation between two Vec3s
    #[inline]
    pub fn lerp(self, rhs: Vec3, t: f32) -> Vec3 {
        self * (1.0 - t) + rhs * t
    }

    /// Computes the midpoint between two Vec3s
    /// The same as lerp where t = 0.5
    #[inline]
    pub fn midpoint(self, rhs: Vec3) -> Vec3 {
        (self + rhs) * 0.5
    }

    /// Move along an axis by a distance d
    /// Requires: axis should be normalized
    #[inline]
    pub fn move_along(self, axis: Vec3, d: f32) -> Vec3 {
        self + axis * d
    }

    /// Move towards a point by a distance d
    /// Allows overshooting the target (no clamping d)
    /// Requires: self != point
    #[inline]
    pub fn move_towards(self, point: Vec3, d: f32) -> Vec3 {
        self + (point - self).normalize() * d
    }

    /// Computes the direction of a ray reflected off the normal of a surface
    /// Requires: normal should be normalized
    #[inline]
    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * normal * self.dot(normal)
    }

    /// Returns the direction vector of a ray refracted to the surface normal
    /// Requires: self and normal should be normalized
    // https://en.wikipedia.org/wiki/Snell's_law
    #[inline]
    pub fn refract(self, normal: Vec3, r: f32) -> Vec3 {
        let cos_a1 = -normal.dot(self);
        let cos_a2_2 = 1.0 - r * r * (1.0 - cos_a1 * cos_a1);
        if cos_a2_2 >= 0.0 {
            r * self + (r * cos_a1 - cos_a2_2.sqrt()) * normal
        } else {
            Vec3::ZERO
        }
    }

    /// Returns cos of the positive acute angle between two Vec3s
    /// Requires: neither self nor rhs should be of length zero
    #[inline]
    pub fn cos_angle_between(self, rhs: Vec3) -> f32 {
        let numerator = self.dot(rhs);
        let denominator = (self.length_2() * rhs.length_2()).sqrt();
        numerator / denominator
    }

    /// Returns sin of the positive acute angle between two Vec3s
    /// Requires: neither self nor rhs should be of length zero
    #[inline]
    pub fn sin_angle_between(self, rhs: Vec3) -> f32 {
        let cos_a = self.cos_angle_between(rhs);
        (1.0 - cos_a * cos_a).sqrt()
    }

    /// Returns sin of the positive acute angle between two Vec3s
    /// Requires: neither self nor rhs should be of length zero
    #[inline]
    pub fn sin_cos_angle_between(self, rhs: Vec3) -> (f32, f32) {
        let cos_a = self.cos_angle_between(rhs);
        let sin_a = (1.0 - cos_a * cos_a).sqrt();
        (sin_a, cos_a)
    }

    /// Returns the positive acute angle between two Vec3s
    /// Requires: neither self nor rhs should be of length zero
    #[inline]
    pub fn angle_between(self, rhs: Vec3) -> f32 {
        self.cos_angle_between(rhs).acos()
    }
}


impl Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec3")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entry(&self.x)
            .entry(&self.y)
            .entry(&self.z)
            .finish()
    }
}


// Vec3 cmp Vec3
impl PartialOrd for Vec3 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length_2().partial_cmp(&other.length_2())
    }
}


// Vec3 + Vec3
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}
impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        self + *rhs
    }
}
impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        *self + rhs
    }
}
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        *self + *rhs
    }
}

// Vec3 + f32
impl Add<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Vec3::new(
            self.x + rhs,
            self.y + rhs,
            self.z + rhs,
        )
    }
}
impl Add<&f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &f32) -> Self::Output {
        self + *rhs
    }
}
impl Add<f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        *self + rhs
    }
}
impl Add<&f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &f32) -> Self::Output {
        *self + *rhs
    }
}

// f32 + Vec3
impl Add<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self + rhs.x,
            self + rhs.y,
            self + rhs.z,
        )
    }
}
impl Add<&Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        self + *rhs
    }
}
impl Add<Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        *self + rhs
    }
}
impl Add<&Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        *self + *rhs
    }
}

// Vec3 += Vec3
impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl AddAssign<&Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec3) {
        *self += *rhs;
    }
}

// Vec3 += f32
impl AddAssign<f32> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}
impl AddAssign<&f32> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &f32) {
        *self += *rhs;
    }
}


// Vec3 - Vec3
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}
impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        self - *rhs
    }
}
impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        *self - rhs
    }
}
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        *self - *rhs
    }
}

// Vec3 - f32
impl Sub<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Vec3::new(
            self.x - rhs,
            self.y - rhs,
            self.z - rhs,
        )
    }
}
impl Sub<&f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &f32) -> Self::Output {
        self - *rhs
    }
}
impl Sub<f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        *self - rhs
    }
}
impl Sub<&f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &f32) -> Self::Output {
        *self - *rhs
    }
}

// f32 - Vec3
impl Sub<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self - rhs.x,
            self - rhs.y,
            self - rhs.z,
        )
    }
}
impl Sub<&Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        self - *rhs
    }
}
impl Sub<Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        *self - rhs
    }
}
impl Sub<&Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        *self - *rhs
    }
}

// Vec3 -= Vec3
impl SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl SubAssign<&Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec3) {
        *self -= *rhs;
    }
}

// Vec3 -= f32
impl SubAssign<f32> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}
impl SubAssign<&f32> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &f32) {
        *self -= *rhs;
    }
}


// Vec3 * Vec3
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
        )
    }
}
impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        self * *rhs
    }
}
impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        *self * rhs
    }
}
impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        *self * *rhs
    }
}

// Vec3 * f32
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
}
impl Mul<&f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        self * *rhs
    }
}
impl Mul<f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        *self * rhs
    }
}
impl Mul<&f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        *self * *rhs
    }
}

// f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z,
        )
    }
}
impl Mul<&Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        self * *rhs
    }
}
impl Mul<Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        *self * rhs
    }
}
impl Mul<&Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        *self * *rhs
    }
}

// Vec3 *= Vec3
impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl MulAssign<&Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Vec3) {
        *self *= *rhs;
    }
}

// Vec3 *= f32
impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl MulAssign<&f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        *self *= *rhs;
    }
}


// Vec3 / Vec3
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
        )
    }
}
impl Div<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        self / *rhs
    }
}
impl Div<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        *self / rhs
    }
}
impl Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        *self / *rhs
    }
}

// Vec3 / f32
impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        )
    }
}
impl Div<&f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        self / *rhs
    }
}
impl Div<f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        *self / rhs
    }
}
impl Div<&f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        *self / *rhs
    }
}

// f32 / Vec3
impl Div<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self / rhs.x,
            self / rhs.y,
            self / rhs.z,
        )
    }
}
impl Div<&Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        self / *rhs
    }
}
impl Div<Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        *self / rhs
    }
}
impl Div<&Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        *self / *rhs
    }
}

// Vec3 /= Vec3
impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl DivAssign<&Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &Vec3) {
        *self /= *rhs;
    }
}

// Vec3 /= f32
impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl DivAssign<&f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        *self /= *rhs;
    }
}


// Vec3 % Vec3
impl Rem<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x % rhs.x,
            self.y % rhs.y,
            self.z % rhs.z,
        )
    }
}
impl Rem<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: &Vec3) -> Self::Output {
        self % *rhs
    }
}
impl Rem<Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: Vec3) -> Self::Output {
        *self % rhs
    }
}
impl Rem<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: &Vec3) -> Self::Output {
        *self % *rhs
    }
}

// Vec3 % f32
impl Rem<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        Vec3::new(
            self.x % rhs,
            self.y % rhs,
            self.z % rhs,
        )
    }
}
impl Rem<&f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: &f32) -> Self::Output {
        self % *rhs
    }
}
impl Rem<f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        *self % rhs
    }
}
impl Rem<&f32> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: &f32) -> Self::Output {
        *self % *rhs
    }
}

// f32 % Vec3
impl Rem<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self % rhs.x,
            self % rhs.y,
            self % rhs.z,
        )
    }
}
impl Rem<&Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: &Vec3) -> Self::Output {
        self % *rhs
    }
}
impl Rem<Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: Vec3) -> Self::Output {
        *self % rhs
    }
}
impl Rem<&Vec3> for &f32 {
    type Output = Vec3;
    #[inline]
    fn rem(self, rhs: &Vec3) -> Self::Output {
        *self % *rhs
    }
}

// Vec3 %= Vec3
impl RemAssign<Vec3> for Vec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: Vec3) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}
impl RemAssign<&Vec3> for Vec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Vec3) {
        *self %= *rhs;
    }
}

// Vec3 %= f32
impl RemAssign<f32> for Vec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
    }
}
impl RemAssign<&f32> for Vec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: &f32) {
        *self %= *rhs;
    }
}


// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}
impl Neg for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}


// Vec3[]
impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Cannot index into a Vec3 at i > 2"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Cannot index into a Vec3 at i > 2"),
        }
    }
}


impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(arr: [f32; 3]) -> Vec3 {
        Vec3::new(arr[0], arr[1], arr[2])
    }
}
impl From<&[f32; 3]> for Vec3 {
    #[inline]
    fn from(arr: &[f32; 3]) -> Vec3 {
        Vec3::new(arr[0], arr[1], arr[2])
    }
}

impl Into<[f32; 3]> for Vec3 {
    #[inline]
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}
impl Into<[f32; 3]> for &Vec3 {
    #[inline]
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(vals: (f32, f32, f32)) -> Self {
        Vec3::new(vals.0, vals.1, vals.2)
    }
}
impl From<&(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(vals: &(f32, f32, f32)) -> Self {
        Vec3::new(vals.0, vals.1, vals.2)
    }
}

impl Into<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}
impl Into<(f32, f32, f32)> for &Vec3 {
    #[inline]
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}
