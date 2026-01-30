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
        Neg,
        Rem,
        RemAssign,
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
    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    /// Positive basis vectors
    pub const X: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    /// Negative basis vectors
    pub const NEG_X: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Vec3 = Vec3::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Vec3 = Vec3::new(0.0, 0.0, -1.0);


    /// Standard constructor for <x y z>
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Constructs a Vec3 from a 3-element array
    #[inline]
    pub const fn from_array(a: [f32; 3]) -> Vec3 {
        Vec3::new(a[0], a[1], a[2])
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
    #[inline]
    pub fn normalize(self) -> Vec3 {
        self * self.r_length()
    }

    /// Returns the normalized vector and its previous magnitute
    #[inline]
    pub fn normalize_and_length(self) -> (Vec3, f32) {
        let length = self.length();
        (self / length, length)
    }

    /// Transforms a local-space Vec3 into world-space
    #[inline]
    pub fn to_world(self, forward: Vec3, right: Vec3, up: Vec3) -> Vec3 {
        forward * self.z + right * self.x + up * self.y
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

    /// Clamps a Vec3 so that each value is between the appropriate min and max element
    #[inline]
    pub fn clamp(self, _min: Vec3, _max: Vec3) -> Vec3 {
        todo!();
    }

    /// Clamps the x value of Vec3
    #[inline]
    pub fn clamp_x(self, min: f32, max: f32) -> Vec3 {
        Vec3::new(
            self.x.clamp(min, max),
            self.y,
            self.z,
        )
    }

    /// Clamps the y value of Vec3
    #[inline]
    pub fn clamp_y(self, min: f32, max: f32) -> Vec3 {
        Vec3::new(
            self.x,
            self.y.clamp(min, max),
            self.z,
        )
    }

    /// Clamps the z value of Vec3
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
    #[inline]
    pub fn r_length(self) -> f32 {
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
    #[inline]
    pub fn r_distance(self, rhs: Vec3) -> f32 {
        (self - rhs).r_length()
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
    #[inline]
    pub fn move_along(self, axis: Vec3, d: f32) -> Vec3 {
        self + axis * d
    }

    /// Move towards a point a distance d
    #[inline]
    pub fn move_towards(self, _point: Vec3, _d: f32) -> Vec3 {
        todo!();
    }

    /// Computes the direction of a ray reflected off the normal of a surface
    #[inline]
    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * normal * self.dot(normal)
    }

    #[inline]
    pub fn refract(self) -> Vec3 {
        todo!(); // ???
    }

    /// Returns cos of the positive acute angle between two Vec3s
    #[inline]
    pub fn cos_angle_between(self, rhs: Vec3) -> f32 {
        let numerator = self.dot(rhs);
        let denominator = (self.length_2() * rhs.length_2()).sqrt();
        numerator / denominator
    }

    /// Returns the positive acute angle between two Vec3s
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


/// Vec3 cmp Vec3
impl PartialOrd for Vec3 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length_2().partial_cmp(&other.length_2())
    }
}


/// Vec3 + Vec3
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

/// Vec3 + f32
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

/// f32 + Vec3
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

/// Vec3 += Vec3
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

/// Vec3 += f32
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


/// Vec3 - Vec3
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

/// Vec3 - f32
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

/// f32 - Vec3
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

/// Vec3 -= Vec3
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

/// Vec3 -= f32
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


/// Vec3 * Vec3
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

/// Vec3 * f32
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

/// f32 * Vec3
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

/// Vec3 *= Vec3
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

/// Vec3 *= f32
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


/// Vec3 / Vec3
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

/// Vec3 / f32
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

/// f32 / Vec3
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

/// Vec3 /= Vec3
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

/// Vec3 /= f32
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


/// -Vec3
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

/// Vec3 % Vec3
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

/// Vec3 %= Vec3
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
