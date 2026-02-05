use crate::{
    F32Ext,
    Vec2,
    Vec3,
    Vec4,
};
use std::{
    array::{
        self,
        IntoIter,
    },
    iter::{
        IntoIterator,
    },
    fmt::{
        self,
        Debug,
        Display,
    },
    ops::{
        Add,
        Sub,
        Mul,
        Div,
        Rem,
        AddAssign,
        SubAssign,
        MulAssign,
        DivAssign,
        RemAssign,
        Neg,
        Index,
        IndexMut,
    },
};

macro_rules! impl_vector {
	($vec:path, $d:expr) => {
	    impl $vec {
            #[inline]
            pub fn min(self, rhs: Self) -> Self {
                let a = self.to_array();
                let b = rhs.to_array();
                let res: [f32; $d] = array::from_fn(|i| a[i].min(b[i]));
                Self::from(res)
            }

            #[inline]
            pub fn min_f(self, v: f32) -> Self {
               let a = self.to_array();
               let res: [f32; $d] = array::from_fn(|i| a[i].min(v));
               Self::from(res)
            }

            #[inline]
            pub fn max(self, rhs: Self) -> Self {
                let a = self.to_array();
                let b = rhs.to_array();
                let res: [f32; $d] = array::from_fn(|i| a[i].max(b[i]));
                Self::from(res)
            }

            #[inline]
            pub fn max_f(self, v: f32) -> Self {
                let a = self.to_array();
                let res: [f32; $d] = array::from_fn(|i| a[i].max(v));
                Self::from(res)
            }

            #[inline]
            pub fn sum(self) -> f32 {
                self.to_array().iter().sum()
            }

            #[inline]
            pub fn abs(self) -> Self {
                let a = self.to_array();
                let res: [f32; $d] = array::from_fn(|i| a[i].abs());
                Self::from(res)
            }

            #[inline]
            pub fn clamp(self, min_vec: Self, max_vec: Self) -> Self {
                let a = self.to_array();
                let min_arr = min_vec.to_array();
                let max_arr = max_vec.to_array();
                let res = array::from_fn(|i| a[i].clamp(min_arr[i], max_arr[i]));
                Self::from(res)
            }

            #[inline]
            pub fn clamp_f(self, min: f32, max: f32) -> Self {
                let a = self.to_array();
                let res = array::from_fn(|i| a[i].clamp(min, max));
                Self::from(res)
                // vvv is not zero-cost compared to the manual implementation unlike ^^^
                // self.max(min).min(max)
            }

            #[inline]
            pub fn dot(self, rhs: Self) -> f32 {
                (self * rhs).sum()
            }

            #[inline]
            pub fn normalize(self) -> Self {
                self * self.length_recip()
            }

            #[inline]
            pub fn normalize_and_length(self) -> (Self, f32) {
                let length = self.length();
                (self / length, length)
            }

            #[inline]
            pub fn length(self) -> f32 {
                self.length_2().sqrt()
            }

            #[inline]
            pub fn length_recip(self) -> f32 {
                self.length_2().rsqrt()
            }

            #[inline]
            pub fn length_2(self) -> f32 {
                self.dot(self)
            }

            #[inline]
            pub fn length_2_recip(self) -> f32 {
                self.dot(self).recip()
            }

            #[inline]
            pub fn distance(self, rhs: Self) -> f32 {
                (self - rhs).length()
            }

            #[inline]
            pub fn distance_recip(self, rhs: Self) -> f32 {
                (self - rhs).length_recip()
            }

            #[inline]
            pub fn distance_2(self, rhs: Self) -> f32 {
                (self - rhs).length_2()
            }

            #[inline]
            pub fn distance_2_recip(self, rhs: Self) -> f32 {
                (self - rhs).length_2_recip()
            }

            #[inline]
            pub fn lerp(self, rhs: Self, t: f32) -> Self {
                self * (1.0 - t) + rhs * t
            }

            #[inline]
            pub fn midpoint(self, rhs: Self) -> Self {
                (self + rhs) * 0.5
            }

            #[inline]
            pub fn recip(self) -> Self {
                let a = self.to_array();
                let res = array::from_fn(|i| a[i].recip());
                Self::from(res)
            }

            #[inline]
            pub fn rem_euclid(self, rhs: Self) -> Self {
                let a = self.to_array();
                let b = rhs.to_array();
                let res = array::from_fn(|i| a[i].rem_euclid(b[i]));
                Self::from(res)
            }

            #[inline]
            pub fn rem_euclid_f(self, v: f32) -> Self {
                let a = self.to_array();
                // keep the following array even though b can just be used itself
                // this generates better assembly, unrolls the loop
                // accessing two const arrays of the same size in the loop rather than one const array and an f32
                let b = [v, v, v];
                let res = array::from_fn(|i| a[i].rem_euclid(b[i]));
                Self::from(res)
            }

            #[inline]
            pub fn div_euclid(self, rhs: Self) -> Self {
                let a = self.to_array();
                let b = rhs.to_array();
                let res = array::from_fn(|i| a[i].div_euclid(b[i]));
                Self::from(res)
            }

            #[inline]
            pub fn div_euclid_f(self, v: f32) -> Self {
                let a = self.to_array();
                // keep this array, look under rem_euclid_f for why
                let b = [v, v, v];
                let res = array::from_fn(|i| a[i].div_euclid(b[i]));
                Self::from(res)
            }
		}


		// $vec + $vec
		impl Add for $vec {
		    type Output = $vec;
			#[inline]
			fn add(self, rhs: $vec) -> Self::Output {
				let a0 = self.to_array();
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] + a1[i]);
				<$vec>::from(res)
            }
        }
    	impl Add<$vec> for &$vec {
            type Output = $vec;
			#[inline]
            fn add(self, rhs: $vec) -> Self::Output {
                *self + rhs
            }
        }
        impl Add<&$vec> for $vec {
            type Output = $vec;
			#[inline]
            fn add(self, rhs: &$vec) -> Self::Output {
                self + *rhs
            }
        }
        impl Add for &$vec {
            type Output = $vec;
			#[inline]
            fn add(self, rhs: &$vec) -> Self::Output {
                *self + *rhs
            }
        }

        // $vec + f32
        impl Add<f32> for $vec {
            type Output = $vec;
			#[inline]
			fn add(self, val: f32) -> Self::Output {
				let a0 = self.to_array();
				let a1 = [val, val, val];
				let res = array::from_fn(|i| a0[i] + a1[i]);
				<$vec>::from(res)
			}
        }
        impl Add<f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn add(self, val: f32) -> Self::Output {
          		*self + val
            }
        }
        impl Add<&f32> for $vec {
            type Output = $vec;
			#[inline]
           	fn add(self, val: &f32) -> Self::Output {
          		self + *val
            }
        }
        impl Add<&f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn add(self, val: &f32) -> Self::Output {
          		*self + *val
            }
        }

        // f32 + $vec
        impl Add<$vec> for f32 {
            type Output = $vec;
            #[inline]
			fn add(self, rhs: $vec) -> Self::Output {
				let a0 = [self, self, self];
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] + a1[i]);
				<$vec>::from(res)
			}
        }
        impl Add<$vec> for &f32 {
            type Output = $vec;
            #[inline]
           	fn add(self, rhs: $vec) -> Self::Output {
          		*self + rhs
            }
        }
        impl Add<&$vec> for f32 {
            type Output = $vec;
            #[inline]
           	fn add(self, rhs: &$vec) -> Self::Output {
          		self + *rhs
            }
        }
        impl Add<&$vec> for &f32 {
            type Output = $vec;
            #[inline]
           	fn add(self, rhs: &$vec) -> Self::Output {
          		*self + *rhs
            }
        }


        // $vec - $vec
		impl Sub for $vec {
    		type Output = $vec;
    		#[inline]
			fn sub(self, rhs: $vec) -> Self::Output {
				let a0 = self.to_array();
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] - a1[i]);
				<$vec>::from(res)
			   }
		    }
		impl Sub<$vec> for &$vec {
    		type Output = $vec;
    		#[inline]
			fn sub(self, rhs: $vec) -> Self::Output {
				*self - rhs
			}
		}
		impl Sub<&$vec> for $vec {
    		type Output = $vec;
    		#[inline]
			fn sub(self, rhs: &$vec) -> Self::Output {
				self - *rhs
			}
		}
		impl Sub for &$vec {
    		type Output = $vec;
    		#[inline]
			fn sub(self, rhs: &$vec) -> Self::Output {
				*self - *rhs
			}
		}

        // $vec - f32
        impl Sub<f32> for $vec {
            type Output = $vec;
			#[inline]
			fn sub(self, val: f32) -> Self::Output {
				let a0 = self.to_array();
				let a1 = [val, val, val];
				let res = array::from_fn(|i| a0[i] - a1[i]);
				<$vec>::from(res)
			}
        }
        impl Sub<f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn sub(self, val: f32) -> Self::Output {
          		*self - val
            }
        }
        impl Sub<&f32> for $vec {
            type Output = $vec;
			#[inline]
           	fn sub(self, val: &f32) -> Self::Output {
          		self - *val
            }
        }
        impl Sub<&f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn sub(self, val: &f32) -> Self::Output {
          		*self - *val
            }
        }

        // f32 - $vec
        impl Sub<$vec> for f32 {
            type Output = $vec;
			#[inline]
			fn sub(self, rhs: $vec) -> Self::Output {
				let a0 = [self, self, self];
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] - a1[i]);
				<$vec>::from(res)
			}
        }
        impl Sub<$vec> for &f32 {
            type Output = $vec;
			#[inline]
           	fn sub(self, rhs: $vec) -> Self::Output {
          		*self - rhs
            }
        }
        impl Sub<&$vec> for f32 {
            type Output = $vec;
			#[inline]
           	fn sub(self, rhs: &$vec) -> Self::Output {
          		self - *rhs
            }
        }
        impl Sub<&$vec> for &f32 {
            type Output = $vec;
			#[inline]
           	fn sub(self, rhs: &$vec) -> Self::Output {
          		*self - *rhs
            }
        }


        // $vec * $vec
		impl Mul for $vec {
    		type Output = $vec;
    		#[inline]
			fn mul(self, rhs: $vec) -> Self::Output {
				let a0 = self.to_array();
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] * a1[i]);
				<$vec>::from(res)
			}
		}
		impl Mul<$vec> for &$vec {
    		type Output = $vec;
    		#[inline]
			fn mul(self, rhs: $vec) -> Self::Output {
				*self * rhs
			}
		}
		impl Mul<&$vec> for $vec {
    		type Output = $vec;
    		#[inline]
			fn mul(self, rhs: &$vec) -> Self::Output {
				self * *rhs
			}
		}
		impl Mul for &$vec {
    		type Output = $vec;
    		#[inline]
			fn mul(self, rhs: &$vec) -> Self::Output {
				*self * *rhs
			}
		}

        // $vec * f32
        impl Mul<f32> for $vec {
            type Output = $vec;
			#[inline]
			fn mul(self, val: f32) -> Self::Output {
				let a0 = self.to_array();
				let a1 = [val, val, val];
				let res = array::from_fn(|i| a0[i] * a1[i]);
				<$vec>::from(res)
			}
        }
        impl Mul<f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn mul(self, val: f32) -> Self::Output {
          		*self * val
            }
        }
        impl Mul<&f32> for $vec {
            type Output = $vec;
			#[inline]
           	fn mul(self, val: &f32) -> Self::Output {
          		self * *val
            }
        }
        impl Mul<&f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn mul(self, val: &f32) -> Self::Output {
          		*self * *val
            }
        }

        // f32 * $vec
        impl Mul<$vec> for f32 {
            type Output = $vec;
			#[inline]
			fn mul(self, rhs: $vec) -> Self::Output {
				let a0 = [self, self, self];
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] * a1[i]);
				<$vec>::from(res)
			}
        }
        impl Mul<$vec> for &f32 {
            type Output = $vec;
			#[inline]
           	fn mul(self, rhs: $vec) -> Self::Output {
          		*self * rhs
            }
        }
        impl Mul<&$vec> for f32 {
            type Output = $vec;
			#[inline]
           	fn mul(self, rhs: &$vec) -> Self::Output {
          		self * *rhs
            }
        }
        impl Mul<&$vec> for &f32 {
            type Output = $vec;
			#[inline]
           	fn mul(self, rhs: &$vec) -> Self::Output {
          		*self * *rhs
            }
        }


        // $vec / $vec
		impl Div for $vec {
    		type Output = $vec;
    		#[inline]
			fn div(self, rhs: $vec) -> Self::Output {
				let a0 = self.to_array();
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] / a1[i]);
				<$vec>::from(res)
			}
		}
		impl Div<$vec> for &$vec {
		type Output = $vec;
		#[inline]
			fn div(self, rhs: $vec) -> Self::Output {
				*self / rhs
			}
		}
        impl Div<&$vec> for $vec {
            type Output = $vec;
			#[inline]
            fn div(self, rhs: &$vec) -> Self::Output {
               	self / *rhs
            }
        }
        impl Div for &$vec {
            type Output = $vec;
			#[inline]
            fn div(self, rhs: &$vec) -> Self::Output {
                *self / *rhs
            }
        }

        // $vec / f32
        impl Div<f32> for $vec {
            type Output = $vec;
			#[inline]
			fn div(self, val: f32) -> Self::Output {
				let a0 = self.to_array();
				let a1 = [val, val, val];
				let res = array::from_fn(|i| a0[i] / a1[i]);
				<$vec>::from(res)
			}
        }
        impl Div<f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn div(self, val: f32) -> Self::Output {
          		*self / val
            }
        }
        impl Div<&f32> for $vec {
            type Output = $vec;
			#[inline]
           	fn div(self, val: &f32) -> Self::Output {
          		self / *val
            }
        }
        impl Div<&f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn div(self, val: &f32) -> Self::Output {
          		*self / *val
            }
        }

        // f32 / $vec
        impl Div<$vec> for f32 {
            type Output = $vec;
			#[inline]
			fn div(self, rhs: $vec) -> Self::Output {
				let a0 = [self, self, self];
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] / a1[i]);
				<$vec>::from(res)
			}
        }
        impl Div<$vec> for &f32 {
            type Output = $vec;
			#[inline]
           	fn div(self, rhs: $vec) -> Self::Output {
          		*self / rhs
            }
        }
        impl Div <&$vec> for f32 {
            type Output = $vec;
			#[inline]
           	fn div(self, rhs: &$vec) -> Self::Output {
          		self / *rhs
            }
        }
        impl Div<&$vec> for &f32 {
            type Output = $vec;
			#[inline]
           	fn div(self, rhs: &$vec) -> Self::Output {
          		*self / *rhs
            }
        }


        // $vec % $vec
		impl Rem for $vec {
		    type Output = $vec;
		    #[inline]
			fn rem(self, rhs: $vec) -> Self::Output {
				let a0 = self.to_array();
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] % a1[i]);
				<$vec>::from(res)
			}
		}
		impl Rem<$vec> for &$vec {
    		type Output = $vec;
    		#[inline]
			fn rem(self, rhs: $vec) -> Self::Output {
				*self % rhs
			}
		}
        impl Rem<&$vec> for $vec {
            type Output = $vec;
			#[inline]
            fn rem(self, rhs: &$vec) -> Self::Output {
               	self % *rhs
            }
        }
        impl Rem for &$vec {
            type Output = $vec;
			#[inline]
            fn rem(self, rhs: &$vec) -> Self::Output {
                *self % *rhs
            }
        }

        // $vec % f32
        impl Rem<f32> for $vec {
            type Output = $vec;
			#[inline]
			fn rem(self, val: f32) -> Self::Output {
				let a0 = self.to_array();
				let a1 = [val, val, val];
				let res = array::from_fn(|i| a0[i] % a1[i]);
				<$vec>::from(res)
			}
        }
        impl Rem<f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn rem(self, val: f32) -> Self::Output {
          		*self % val
            }
        }
        impl Rem<&f32> for $vec {
            type Output = $vec;
			#[inline]
           	fn rem(self, val: &f32) -> Self::Output {
          		self % *val
            }
        }
        impl Rem<&f32> for &$vec {
            type Output = $vec;
			#[inline]
           	fn rem(self, val: &f32) -> Self::Output {
          		*self % *val
            }
        }

        // f32 % $vec
        impl Rem<$vec> for f32 {
            type Output = $vec;
			#[inline]
			fn rem(self, rhs: $vec) -> Self::Output {
				let a0 = [self, self, self];
				let a1 = rhs.to_array();
				let res = array::from_fn(|i| a0[i] % a1[i]);
				<$vec>::from(res)
			}
        }
        impl Rem<$vec> for &f32 {
            type Output = $vec;
			#[inline]
           	fn rem(self, rhs: $vec) -> Self::Output {
          		*self % rhs
            }
        }
        impl Rem <&$vec> for f32 {
            type Output = $vec;
			#[inline]
           	fn rem(self, rhs: &$vec) -> Self::Output {
          		self % *rhs
            }
        }
        impl Rem<&$vec> for &f32 {
            type Output = $vec;
			#[inline]
           	fn rem(self, rhs: &$vec) -> Self::Output {
          		*self % *rhs
            }
        }


        impl AsRef<[f32; $d]> for $vec {
            #[inline]
            fn as_ref(&self) -> &[f32; $d] {
                unsafe { &*(self as *const $vec as *const [f32; $d]) }
            }
        }

        impl AsMut<[f32; $d]> for $vec {
            #[inline]
            fn as_mut(&mut self) -> &mut [f32; $d] {
                unsafe { &mut *(self as *mut $vec as *mut [f32; $d]) }
            }
        }


        impl IntoIterator for $vec {
            type Item = f32;
            type IntoIter = IntoIter<f32, $d>;
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.to_array().into_iter()
            }
        }


        // $vec += $vec
        impl AddAssign for $vec {
            #[inline]
            fn add_assign(&mut self, rhs: $vec) {
                let a0 = self.as_mut();
                let a1 = rhs.to_array();
                for (i, v) in a0.iter_mut().enumerate() {
                    *v += a1[i];
                }
            }
        }
        impl AddAssign<&$vec> for $vec {
            #[inline]
            fn add_assign(&mut self, rhs: &$vec) {
                *self += *rhs;
            }
        }

        // $vec += f32
        impl AddAssign<f32> for $vec {
            #[inline]
            fn add_assign(&mut self, val: f32) {
                let a0 = self.as_mut();
                let a1 = [val, val, val];
                for (i, v) in a0.iter_mut().enumerate() {
                    *v += a1[i];
                }
            }
        }
        impl AddAssign<&f32> for $vec {
            #[inline]
            fn add_assign(&mut self, val: &f32) {
                *self += *val;
            }
        }


        // $vec -= $vec
        impl SubAssign for $vec {
            #[inline]
            fn sub_assign(&mut self, rhs: $vec) {
                let a0 = self.as_mut();
                let a1 = rhs.to_array();
                for (i, v) in a0.iter_mut().enumerate() {
                    *v -= a1[i];
                }
            }
        }
        impl SubAssign<&$vec> for $vec {
            #[inline]
            fn sub_assign(&mut self, rhs: &$vec) {
                *self -= *rhs;
            }
        }

        // $vec -= f32
        impl SubAssign<f32> for $vec {
            #[inline]
            fn sub_assign(&mut self, val: f32) {
                let a0 = self.as_mut();
                let a1 = [val, val, val];
                for (i, v) in a0.iter_mut().enumerate() {
                    *v -= a1[i];
                }
            }
        }
        impl SubAssign<&f32> for $vec {
            #[inline]
            fn sub_assign(&mut self, val: &f32) {
                *self -= *val;
            }
        }



        // $vec *= $vec
        impl MulAssign for $vec {
            #[inline]
            fn mul_assign(&mut self, rhs: $vec) {
                let a0 = self.as_mut();
                let a1 = rhs.to_array();
                for (i, v) in a0.iter_mut().enumerate() {
                    *v *= a1[i];
                }
            }
        }
        impl MulAssign<&$vec> for $vec {
            #[inline]
            fn mul_assign(&mut self, rhs: &$vec) {
                *self *= *rhs;
            }
        }

        // $vec *= f32
        impl MulAssign<f32> for $vec {
            #[inline]
            fn mul_assign(&mut self, val: f32) {
                let a0 = self.as_mut();
                let a1 = [val, val, val];
                for (i, v) in a0.iter_mut().enumerate() {
                    *v *= a1[i];
                }
            }
        }
        impl MulAssign<&f32> for $vec {
            #[inline]
            fn mul_assign(&mut self, val: &f32) {
                *self *= *val;
            }
        }


        // $vec /= $vec
        impl DivAssign for $vec {
            #[inline]
            fn div_assign(&mut self, rhs: $vec) {
                let a0 = self.as_mut();
                let a1 = rhs.to_array();
                for (i, v) in a0.iter_mut().enumerate() {
                    *v /= a1[i];
                }
            }
        }
        impl DivAssign<&$vec> for $vec {
            #[inline]
            fn div_assign(&mut self, rhs: &$vec) {
                *self /= *rhs;
            }
        }

        // $vec /= f32
        impl DivAssign<f32> for $vec {
            #[inline]
            fn div_assign(&mut self, val: f32) {
                let a0 = self.as_mut();
                let a1 = [val, val, val];
                for (i, v) in a0.iter_mut().enumerate() {
                    *v /= a1[i];
                }
            }
        }
        impl DivAssign<&f32> for $vec {
            #[inline]
            fn div_assign(&mut self, val: &f32) {
                *self /= *val;
            }
        }


        // $vec %= $vec
        impl RemAssign for $vec {
            #[inline]
            fn rem_assign(&mut self, rhs: $vec) {
                let a0 = self.as_mut();
                let a1 = rhs.to_array();
                for (i, v) in a0.iter_mut().enumerate() {
                    *v %= a1[i];
                }
            }
        }
        impl RemAssign<&$vec> for $vec {
            #[inline]
            fn rem_assign(&mut self, rhs: &$vec) {
                *self %= *rhs;
            }
        }

        // $vec %= f32
        impl RemAssign<f32> for $vec {
            #[inline]
            fn rem_assign(&mut self, val: f32) {
                let a0 = self.as_mut();
                let a1 = [val, val, val];
                for (i, v) in a0.iter_mut().enumerate() {
                    *v %= a1[i];
                }
            }
        }
        impl RemAssign<&f32> for $vec {
            #[inline]
            fn rem_assign(&mut self, val: &f32) {
                *self %= *val;
            }
        }


        // -$vec
        impl Neg for $vec {
            type Output = $vec;
            #[inline]
            fn neg(self) -> Self::Output {
                -1.0 * self
            }
        }
        impl Neg for &$vec {
            type Output = $vec;
            #[inline]
            fn neg(self) -> Self::Output {
                -1.0 * self
            }
        }


        // $vec[]
        impl Index<usize> for $vec {
            type Output = f32;
            fn index(&self, index: usize) -> &Self::Output {
                &self.as_ref()[index]
            }
        }
        impl Index<usize> for &$vec {
            type Output = f32;
            fn index(&self, index: usize) -> &Self::Output {
                &self.as_ref()[index]
            }
        }
        impl Index<usize> for &mut $vec {
            type Output = f32;
            fn index(&self, index: usize) -> &Self::Output {
                &self.as_ref()[index]
            }
        }

        impl IndexMut<usize> for $vec {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_mut()[index]
            }
        }
        impl IndexMut<usize> for &mut $vec {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_mut()[index]
            }
        }


        impl From<[f32; $d]> for $vec {
            #[inline]
            fn from(arr: [f32; $d]) -> $vec {
                <$vec>::from_array(arr)
            }
        }
        impl From<&[f32; $d]> for $vec {
            #[inline]
            fn from(arr: &[f32; $d]) -> $vec {
                <$vec>::from_array(*arr)
            }
        }

        impl Into<[f32; $d]> for $vec {
            #[inline]
            fn into(self) -> [f32; $d] {
                <$vec>::to_array(self)
            }
        }
        impl Into<[f32; $d]> for &$vec {
            #[inline]
            fn into(self) -> [f32; $d] {
                <$vec>::to_array(*self)
            }
        }


        impl Debug for $vec {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let _ = write!(f, "Vec{}", $d);
                f.debug_list()
                    .entries(self.into_iter())
                    .finish()
            }
        }

        impl Display for $vec {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list()
                    .entries(self.into_iter())
                    .finish()
            }
        }
	}
}


impl_vector!(Vec2, 2);
impl_vector!(Vec3, 3);
impl_vector!(Vec4, 4);
