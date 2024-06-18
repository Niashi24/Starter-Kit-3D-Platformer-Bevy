use std::ops::{Add, Mul, Sub};
use num_traits::real::Real;


pub fn smooth_damp<N, F>(a: N, b: N, v: N, dt: F, smooth_time: F) -> (N, N)
    where
        N: Mul<F, Output=N> + Add<N, Output=N> + Sub<N, Output=N> + Copy,
        F: Real,
    
{
    let two = F::one() + F::one();
    let smooth_factor = two / smooth_time;
    let exp = (-smooth_factor * dt).exp();

    (
        b + ((a - b) + (v + (a - b) * smooth_factor) * dt) * exp,
        (v - (v + (a - b) * smooth_factor) * smooth_factor * dt) * exp
    )
}
