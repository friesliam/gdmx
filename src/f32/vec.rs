use crate::{
    F32Ext,
};
use std::{
    array::{
        self,
        IntoIter,
    },
    slice::{
        Iter,
        IterMut,
    },
    iter::{
        IntoIterator,
    },
    ops::{
        Add,
        Sub,
        Mul,
        Div,
    },
};

// sum, min, min_vec, max, max_vec, abs, clamp, clamp_vec
// recip, rem_euclid, rem_euclid_vec, div_euclid, div_euclid_vec
// are all zero-cost abstractions compared to manually implementing them for each vector type of length N
// The exact same assembly is generated in release mode
// (given, but every other method is, or should be, zero-cost compared to manual impl as well)

pub trait VecExt<const N: usize>:
    Copy
    + Clone
    + Default
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
    + Into<[f32; N]>
    + From<[f32; N]>
    + AsRef<[f32; N]>
    + AsMut<[f32; N]>
{
    #[inline]
    fn to_array(self) -> [f32; N] {
        self.into()
    }

    #[inline]
    fn min(self, v: f32) -> Self {
        let a = self.to_array();
        let res: [f32; N] = array::from_fn(|i| a[i].min(v));
        Self::from(res)
    }

    #[inline]
    fn min_vec(self, rhs: Self) -> Self {
        let a = self.to_array();
        let b = rhs.to_array();
        let res: [f32; N] = array::from_fn(|i| a[i].min(b[i]));
        Self::from(res)
    }

    #[inline]
    fn max(self, v: f32) -> Self {
        let a = self.to_array();
        let res: [f32; N] = array::from_fn(|i| a[i].max(v));
        Self::from(res)
    }

    #[inline]
    fn max_vec(self, rhs: Self) -> Self {
        let a = self.to_array();
        let b = rhs.to_array();
        let res: [f32; N] = array::from_fn(|i| a[i].max(b[i]));
        Self::from(res)
    }

    #[inline]
    fn sum(self) -> f32 {
        self.to_array().iter().sum()
    }

    #[inline]
    fn abs(self) -> Self {
        let a = self.to_array();
        let res: [f32; N] = array::from_fn(|i| a[i].abs());
        Self::from(res)
    }

    #[inline]
    fn clamp(self, min: f32, max: f32) -> Self {
        let a = self.to_array();
        let res = array::from_fn(|i| a[i].clamp(min, max));
        Self::from(res)
        // vvv is not zero-cost compared to the manual implementation unlike ^^^
        // self.max(min).min(max)
    }

    #[inline]
    fn clamp_vec(self, min_vec: Self, max_vec: Self) -> Self {
        let a = self.to_array();
        let min_arr = min_vec.to_array();
        let max_arr = max_vec.to_array();
        let res = array::from_fn(|i| a[i].clamp(min_arr[i], max_arr[i]));
        Self::from(res)
        // self.max_vec(min_vec).min_vec(max_vec)
    }

    #[inline]
    fn dot(self, rhs: Self) -> f32 {
        (self * rhs).sum()
    }

    #[inline]
    fn normalize(self) -> Self {
        self * self.length_recip()
    }

    #[inline]
    fn normalize_and_length(self) -> (Self, f32) {
        let length = self.length();
        (self / length, length)
    }

    #[inline]
    fn length(self) -> f32 {
        self.length_2().sqrt()
    }

    #[inline]
    fn length_recip(self) -> f32 {
        self.length_2().rsqrt()
    }

    #[inline]
    fn length_2(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    fn length_2_recip(self) -> f32 {
        self.dot(self).recip()
    }

    #[inline]
    fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    #[inline]
    fn distance_recip(self, rhs: Self) -> f32 {
        (self - rhs).length_recip()
    }

    #[inline]
    fn distance_2(self, rhs: Self) -> f32 {
        (self - rhs).length_2()
    }

    #[inline]
    fn distance_2_recip(self, rhs: Self) -> f32 {
        (self - rhs).length_2_recip()
    }

    #[inline]
    fn lerp(self, rhs: Self, t: f32) -> Self {
        self * (1.0 - t) + rhs * t
    }

    #[inline]
    fn midpoint(self, rhs: Self) -> Self {
        (self + rhs) * 0.5
    }

    #[inline]
    fn recip(self) -> Self {
        let a = self.to_array();
        let res = array::from_fn(|i| a[i].recip());
        Self::from(res)
    }

    #[inline]
    fn rem_euclid(self, v: f32) -> Self {
        let a = self.to_array();
        // keep the following array even though b can just be used itself
        // this generates better assembly, unrolls the loop
        // accessing two const arrays of the same size in the loop rather than one const array and an f32
        let b = [v, v, v];
        let res = array::from_fn(|i| a[i].rem_euclid(b[i]));
        Self::from(res)
    }

    #[inline]
    fn div_euclid(self, v: f32) -> Self {
        let a = self.to_array();
        // keep this array, look under rem_euclid for why
        let b = [v, v, v];
        let res = array::from_fn(|i| a[i].div_euclid(b[i]));
        Self::from(res)
    }

    #[inline]
    fn rem_euclid_vec(self, rhs: Self) -> Self {
        let a = self.to_array();
        let b = rhs.to_array();
        let res = array::from_fn(|i| a[i].rem_euclid(b[i]));
        Self::from(res)
    }

    #[inline]
    fn div_euclid_vec(self, rhs: Self) -> Self {
        let a = self.to_array();
        let b = rhs.to_array();
        let res = array::from_fn(|i| a[i].div_euclid(b[i]));
        Self::from(res)
    }


    #[inline]
    fn into_iter(self) -> IntoIter<f32, N> {
        self.to_array().into_iter()
    }

    #[inline]
    fn iter(&self) -> Iter<'_, f32> {
        self.as_ref().iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> IterMut<'_, f32> {
        self.as_mut().iter_mut()
    }
}
