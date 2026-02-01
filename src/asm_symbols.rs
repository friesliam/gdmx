use crate::*;
use std::hint::black_box;


// Expose functions so they can be found in assembly
// gen_asm.sh dumps the functions to fns.asm

// Test trait methods such as from VecExt<const N: usize>
// vs
// Their corresponding manual implementations for a vector of length N


#[unsafe(no_mangle)]
pub extern "C" fn vec3_sum_manual(v: Vec3) -> f32 {
    black_box(v.x + v.y + v.z)
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_sum_trait(v: Vec3) -> f32 {
    black_box(v.sum())
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_min_manual(a: Vec3, b: f32) -> Vec3 {
    black_box(Vec3::new(a.x.min(b), a.y.min(b), a.z.min(b)))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_min_trait(a: Vec3, b: f32) -> Vec3 {
    black_box(a.min(b))
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_min_vec_manual(a: Vec3, b: Vec3) -> Vec3 {
    black_box(Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)))
}

#[unsafe(no_mangle)]

pub extern "C" fn vec3_min_vec_trait(a: Vec3, b: Vec3) -> Vec3 {
    black_box(a.min_vec(b))
}


#[unsafe(no_mangle)]

pub extern "C" fn vec3_max_manual(a: Vec3, b: f32) -> Vec3 {
    black_box(Vec3::new(a.x.max(b), a.y.max(b), a.z.max(b)))
}

#[unsafe(no_mangle)]

pub extern "C" fn vec3_max_trait(a: Vec3, b: f32) -> Vec3 {
    black_box(a.max(b))
}


#[unsafe(no_mangle)]

pub extern "C" fn vec3_max_vec_manual(a: Vec3, b: Vec3) -> Vec3 {
    black_box(Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_max_vec_trait(a: Vec3, b: Vec3) -> Vec3 {
    black_box(a.max_vec(b))
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_abs_manual(v: Vec3) -> Vec3 {
    black_box(Vec3::new(v.x.abs(), v.y.abs(), v.z.abs()))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_abs_trait(v: Vec3) -> Vec3 {
    black_box(v.abs())
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_clamp_manual(v: Vec3, min: f32, max: f32) -> Vec3 {
    black_box(Vec3::new(
        v.x.clamp(min, max),
        v.y.clamp(min, max),
        v.z.clamp(min, max),
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_clamp_trait(v: Vec3, min: f32, max: f32) -> Vec3 {
    black_box(v.clamp(min, max))
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_clamp_vec_manual(v: Vec3, min: Vec3, max: Vec3) -> Vec3 {
    black_box(Vec3::new(
        v.x.clamp(min.x, max.x),
        v.y.clamp(min.y, max.y),
        v.z.clamp(min.z, max.z),
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_clamp_vec_trait(v: Vec3, min: Vec3, max: Vec3) -> Vec3 {
    black_box(v.clamp_vec(min, max))
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_recip_manual(v: Vec3) -> Vec3 {
    black_box(Vec3::new(v.x.recip(), v.y.recip(), v.z.recip()))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_recip_trait(v: Vec3) -> Vec3 {
    black_box(v.recip())
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_rem_euclid_manual(v: Vec3, rhs: f32) -> Vec3 {
    black_box(Vec3::new(
        v.x.rem_euclid(rhs),
        v.y.rem_euclid(rhs),
        v.z.rem_euclid(rhs),
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_rem_euclid_trait(v: Vec3, rhs: f32) -> Vec3 {
    black_box(v.rem_euclid(rhs))
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_rem_euclid_vec_manual(a: Vec3, b: Vec3) -> Vec3 {
    black_box(Vec3::new(
        a.x.rem_euclid(b.x),
        a.y.rem_euclid(b.y),
        a.z.rem_euclid(b.z),
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_rem_euclid_vec_trait(a: Vec3, b: Vec3) -> Vec3 {
    black_box(a.rem_euclid_vec(b))
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_div_euclid_manual(v: Vec3, rhs: f32) -> Vec3 {
    black_box(Vec3::new(
        v.x.div_euclid(rhs),
        v.y.div_euclid(rhs),
        v.z.div_euclid(rhs),
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_div_euclid_trait(v: Vec3, rhs: f32) -> Vec3 {
    black_box(v.div_euclid(rhs))
}


#[unsafe(no_mangle)]
pub extern "C" fn vec3_div_euclid_vec_manual(a: Vec3, b: Vec3) -> Vec3 {
    black_box(Vec3::new(
        a.x.div_euclid(b.x),
        a.y.div_euclid(b.y),
        a.z.div_euclid(b.z),
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3_div_euclid_vec_trait(a: Vec3, b: Vec3) -> Vec3 {
    black_box(a.div_euclid_vec(b))
}
