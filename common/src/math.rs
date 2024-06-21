use std::ops::{Add, Mul, Sub};
use num_traits::real::Real;

use bevy::math::{Mat3, Quat, Vec3};


// pub fn smooth_damp<N, F>(a: N, b: N, v: N, dt: F, smooth_time: F) -> (N, N)
//     where
//         N: Mul<F, Output=N> + Add<N, Output=N> + Sub<N, Output=N> + Copy,
//         F: Real,
//     
// {
//     let two = F::one() + F::one();
//     let smooth_factor = two / smooth_time;
//     let exp = (-smooth_factor * dt).exp();
// 
//     (
//         b + ((a - b) + (v + (a - b) * smooth_factor) * dt) * exp,
//         (v - (v + (a - b) * smooth_factor) * smooth_factor * dt) * exp
//     )
// }

pub fn nudge<N, F>(a: N, b: N, decay: F, delta: F) -> N
    where
        N: Mul<F, Output=N> + Add<N, Output=N> + Sub<N, Output=N> + Copy,
        F: Real
{
    b + (a - b) * F::exp(-decay * delta)
}

/// Rotates this [`Transform`] so that [`Transform::forward`] points in the given `direction`
/// and [`Transform::up`] points towards `up`.
///
/// In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
/// * if `direction` is zero, `Vec3::NEG_Z` is used instead
/// * if `up` is zero, `Vec3::Y` is used instead
/// * if `direction` is parallel with `up`, an orthogonal vector is used as the "right" direction
#[inline]
pub fn look_to(direction: Vec3, up: Vec3) -> Quat {
    let back = -direction.try_normalize().unwrap_or(Vec3::NEG_Z);
    let up = up.try_normalize().unwrap_or(Vec3::Y);
    let right = up
        .cross(back)
        .try_normalize()
        .unwrap_or_else(|| up.any_orthonormal_vector());
    let up = back.cross(right);
    Quat::from_mat3(&Mat3::from_cols(right, up, back))
}

