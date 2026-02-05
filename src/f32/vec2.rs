/// A vector in 2-space
#[derive(Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// The default Vec3 with all zeros
    pub const ZERO: Self = Self::splat(0.0);

    /// The positive x-axis basis vector
    pub const X: Self = Self::new(1.0, 0.0);

    /// The positive y-axis basis vector
    pub const Y: Self = Self::new(0.0, 1.0);

    /// The negative x-axis basis vector
    pub const NEG_X: Self = Self::new(-1.0, 0.0);

    /// The negative y-axis basis vector
    pub const NEG_Y: Self = Self::new(0.0, -1.0);


    #[inline]
    pub fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub fn from_array(arr: [f32; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self::new(v, v)
    }
}
