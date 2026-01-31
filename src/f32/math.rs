
pub(crate) trait F32Ext {
    fn rsqrt(self) -> f32;
}

impl F32Ext for f32 {
    #[inline]
    fn rsqrt(self) -> f32 {
        1.0 / self.sqrt()
    }
}
