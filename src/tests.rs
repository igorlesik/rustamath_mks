//! MKS tests
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
use super::*;

// cargo test simple_pendulum -- --nocapture
#[test]
fn simple_pendulum() {
    // simple pendulum period formula is `T = 2*Pi*sqrt(L/g)`
    let pendulum_len = MksVal::new(6.0, f64::FOOT, FOOT_UNIT);
    let g = MksVal::new(1.0, f64::GRAV_ACCEL, GRAV_ACCEL_UNIT);

    println!("Pendulum length is {:.2} {}", pendulum_len.val, pendulum_len.unit);
    println!("G on Earth is {:.2} {}", g.val, g.unit);

    assert_eq!(pendulum_len.unit.to_string(), "[m]");
    assert_eq!(g.unit.to_string(), "[m / s^2]");

    let pendulum_len_over_accel = pendulum_len / g;
    assert!(pendulum_len_over_accel.unit == TIME_UNIT * TIME_UNIT);

    let pi_x_2 = MksVal::new_scalar(2.0 * std::f64::consts::PI);

    let period = pi_x_2 * pendulum_len_over_accel.sqrt();
    assert!(period.unit == TIME_UNIT);

    println!("Pendulum period is {:.2} {}", period.val, period.unit);
    assert_eq!(period.unit.to_string(), "[s]");
}

