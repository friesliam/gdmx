use crate::{
    VecExt,
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
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl VecExt<4> for Vec4 {}

impl Vec4 {
    /// The default Vec4 with all 0's
    pub const ZERO: Vec4 = Vec4::splat(0.0);

    /// The positive x-axis basis vector
    pub const X: Vec4 = Vec4::new(1.0, 0.0, 0.0, 0.0);

    /// The positive y-axis basis vector
    pub const Y: Vec4 = Vec4::new(0.0, 1.0, 0.0, 0.0);

    /// The positive z-axis basis vector
    pub const Z: Vec4 = Vec4::new(0.0, 0.0, 1.0, 0.0);

    /// The positive w-axis basis vector
    pub const W: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);

    /// The negative x-axis basis vector
    pub const NEG_X: Vec4 = Vec4::new(-1.0, 0.0, 0.0, 0.0);

    /// The negative y-axis basis vector
    pub const NEG_Y: Vec4 = Vec4::new(0.0, -1.0, 0.0, 0.0);

    /// The negative z-axis basis vector
    pub const NEG_Z: Vec4 = Vec4::new(0.0, 0.0, -1.0, 0.0);

    /// The negative w-axis basis vector
    pub const NEG_W: Vec4 = Vec4::new(0.0, 0.0, 0.0, -1.0);


    /// Standard constructor for <x y z w>
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }

    #[inline]
    pub const fn splat(v: f32) -> Vec4 {
        Vec4::new(v, v, v, v)
    }


    /// Clamps the x value of Vec4
    /// Requires: min < max
    #[inline]
    pub fn clamp_x(self, min: f32, max: f32) -> Vec4 {
        Vec4::new(
            self.x.clamp(min, max),
            self.y,
            self.z,
            self.w,
        )
    }

    /// Clamps the y value of Vec4
    /// Requires: min < max
    #[inline]
    pub fn clamp_y(self, min: f32, max: f32) -> Vec4 {
        Vec4::new(
            self.x,
            self.y.clamp(min, max),
            self.z,
            self.w
        )
    }

    /// Clamps the z value of Vec4
    /// Requires: min < max
    #[inline]
    pub fn clamp_z(self, min: f32, max: f32) -> Vec4 {
        Vec4::new(
            self.x,
            self.y,
            self.z.clamp(min, max),
            self.w,
        )
    }

    /// Clamps the w value of Vec4
    /// Requires: min < max
    #[inline]
    pub fn clamp_w(self, min: f32, max: f32) -> Vec4 {
        Vec4::new(
            self.x,
            self.y,
            self.z,
            self.w.clamp(min, max),
        )
    }
}


impl Debug for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec4")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}
impl Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entry(&self.x)
            .entry(&self.y)
            .entry(&self.z)
            .entry(&self.w)
            .finish()
    }
}


// Vec4 cmp Vec4
impl PartialOrd for Vec4 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length_2().partial_cmp(&other.length_2())
    }
}


// Vec4 + Vec4
impl Add<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}
impl Add<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &Vec4) -> Self::Output {
        self + *rhs
    }
}
impl Add<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Self::Output {
        *self + rhs
    }
}
impl Add<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &Vec4) -> Self::Output {
        *self + *rhs
    }
}

// Vec4 + f32
impl Add<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Vec4::new(
            self.x + rhs,
            self.y + rhs,
            self.z + rhs,
            self.w + rhs,
        )
    }
}
impl Add<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &f32) -> Self::Output {
        self + *rhs
    }
}
impl Add<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        *self + rhs
    }
}
impl Add<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &f32) -> Self::Output {
        *self + *rhs
    }
}

// f32 + Vec4
impl Add<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self + rhs.x,
            self + rhs.y,
            self + rhs.z,
            self + rhs.w,
        )
    }
}
impl Add<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &Vec4) -> Self::Output {
        self + *rhs
    }
}
impl Add<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Self::Output {
        *self + rhs
    }
}
impl Add<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &Vec4) -> Self::Output {
        *self + *rhs
    }
}

// Vec4 += Vec4
impl AddAssign<Vec4> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
impl AddAssign<&Vec4> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec4) {
        *self += *rhs;
    }
}

// Vec4 += f32
impl AddAssign<f32> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}
impl AddAssign<&f32> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: &f32) {
        *self += *rhs;
    }
}


// Vec4 - Vec4
impl Sub<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}
impl Sub<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &Vec4) -> Self::Output {
        self - *rhs
    }
}
impl Sub<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Self::Output {
        *self - rhs
    }
}
impl Sub<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &Vec4) -> Self::Output {
        *self - *rhs
    }
}

// Vec4 - f32
impl Sub<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Vec4::new(
            self.x - rhs,
            self.y - rhs,
            self.z - rhs,
            self.w - rhs,
        )
    }
}
impl Sub<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &f32) -> Self::Output {
        self - *rhs
    }
}
impl Sub<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        *self - rhs
    }
}
impl Sub<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &f32) -> Self::Output {
        *self - *rhs
    }
}

// f32 - Vec4
impl Sub<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self - rhs.x,
            self - rhs.y,
            self - rhs.z,
            self - rhs.w,
        )
    }
}
impl Sub<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &Vec4) -> Self::Output {
        self - *rhs
    }
}
impl Sub<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Self::Output {
        *self - rhs
    }
}
impl Sub<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &Vec4) -> Self::Output {
        *self - *rhs
    }
}

// Vec4 -= Vec4
impl SubAssign<Vec4> for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}
impl SubAssign<&Vec4> for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec4) {
        *self -= *rhs;
    }
}

// Vec4 -= f32
impl SubAssign<f32> for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}
impl SubAssign<&f32> for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: &f32) {
        *self -= *rhs;
    }
}


// Vec4 * Vec4
impl Mul<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w
        )
    }
}
impl Mul<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Self::Output {
        self * *rhs
    }
}
impl Mul<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        *self * rhs
    }
}
impl Mul<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Self::Output {
        *self * *rhs
    }
}

// Vec4 * f32
impl Mul<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec4::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs,
        )
    }
}
impl Mul<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        self * *rhs
    }
}
impl Mul<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        *self * rhs
    }
}
impl Mul<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        *self * *rhs
    }
}

// f32 * Vec4
impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z,
            self * rhs.w,
        )
    }
}
impl Mul<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Self::Output {
        self * *rhs
    }
}
impl Mul<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        *self * rhs
    }
}
impl Mul<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Self::Output {
        *self * *rhs
    }
}

// Vec4 *= Vec4
impl MulAssign<Vec4> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}
impl MulAssign<&Vec4> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Vec4) {
        *self *= *rhs;
    }
}

// Vec4 *= f32
impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}
impl MulAssign<&f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        *self *= *rhs;
    }
}


// Vec4 / Vec4
impl Div<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }
}
impl Div<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &Vec4) -> Self::Output {
        self / *rhs
    }
}
impl Div<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: Vec4) -> Self::Output {
        *self / rhs
    }
}
impl Div<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &Vec4) -> Self::Output {
        *self / *rhs
    }
}

// Vec4 / f32
impl Div<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vec4::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
            self.w / rhs,
        )
    }
}
impl Div<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        self / *rhs
    }
}
impl Div<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        *self / rhs
    }
}
impl Div<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        *self / *rhs
    }
}

// f32 / Vec4
impl Div<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self / rhs.x,
            self / rhs.y,
            self / rhs.z,
            self / rhs.w,
        )
    }
}
impl Div<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &Vec4) -> Self::Output {
        self / *rhs
    }
}
impl Div<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: Vec4) -> Self::Output {
        *self / rhs
    }
}
impl Div<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &Vec4) -> Self::Output {
        *self / *rhs
    }
}

// Vec4 /= Vec4
impl DivAssign<Vec4> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec4) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}
impl DivAssign<&Vec4> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: &Vec4) {
        *self /= *rhs;
    }
}

// Vec4 /= f32
impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}
impl DivAssign<&f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        *self /= *rhs;
    }
}


// Vec4 % Vec4
impl Rem<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x % rhs.x,
            self.y % rhs.y,
            self.z % rhs.z,
            self.w % rhs.w,
        )
    }
}
impl Rem<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &Vec4) -> Self::Output {
        self % *rhs
    }
}
impl Rem<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: Vec4) -> Self::Output {
        *self % rhs
    }
}
impl Rem<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &Vec4) -> Self::Output {
        *self % *rhs
    }
}

// Vec4 % f32
impl Rem<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        Vec4::new(
            self.x % rhs,
            self.y % rhs,
            self.z % rhs,
            self.w % rhs,
        )
    }
}
impl Rem<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &f32) -> Self::Output {
        self % *rhs
    }
}
impl Rem<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        *self % rhs
    }
}
impl Rem<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &f32) -> Self::Output {
        *self % *rhs
    }
}

// f32 % Vec4
impl Rem<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self % rhs.x,
            self % rhs.y,
            self % rhs.z,
            self % rhs.w,
        )
    }
}
impl Rem<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &Vec4) -> Self::Output {
        self % *rhs
    }
}
impl Rem<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: Vec4) -> Self::Output {
        *self % rhs
    }
}
impl Rem<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &Vec4) -> Self::Output {
        *self % *rhs
    }
}

// Vec4 %= Vec4
impl RemAssign<Vec4> for Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: Vec4) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}
impl RemAssign<&Vec4> for Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Vec4) {
        *self %= *rhs;
    }
}

// Vec4 %= f32
impl RemAssign<f32> for Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
        self.w %= rhs;
    }
}
impl RemAssign<&f32> for Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: &f32) {
        *self %= *rhs;
    }
}


// -Vec4
impl Neg for Vec4 {
    type Output = Vec4;
    #[inline]
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}
impl Neg for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}


// Vec4[]
impl Index<usize> for Vec4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Cannot index into a Vec4 at i > 3"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Cannot index into a Vec4 at i > 3"),
        }
    }
}


impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(arr: [f32; 4]) -> Vec4 {
        Vec4::new(arr[0], arr[1], arr[2], arr[3])
    }
}
impl From<&[f32; 4]> for Vec4 {
    #[inline]
    fn from(arr: &[f32; 4]) -> Vec4 {
        Vec4::new(arr[0], arr[1], arr[2], arr[3])
    }
}

impl Into<[f32; 4]> for Vec4 {
    #[inline]
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
impl Into<[f32; 4]> for &Vec4 {
    #[inline]
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(vals: (f32, f32, f32, f32)) -> Self {
        Vec4::new(vals.0, vals.1, vals.2, vals.3)
    }
}
impl From<&(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(vals: &(f32, f32, f32, f32)) -> Self {
        Vec4::new(vals.0, vals.1, vals.2, vals.3)
    }
}

impl Into<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}
impl Into<(f32, f32, f32, f32)> for &Vec4 {
    #[inline]
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}


impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Vec4 as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Vec4 as *mut [f32; 4]) }
    }
}
