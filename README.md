# Rustamath MKS

![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
[![CI](https://github.com/igorlesik/rustamath_mks/actions/workflows/test.yml/badge.svg)](https://github.com/igorlesik/rustamath_mks/actions/workflows/test.yml)
[![crates.io version][crates-io-shields]][crates-io]
[![docs.rs][docs-rs-shields]][docs-rs]

[crates-io]: https://crates.io/crates/rustamath_mks
[crates-io-shields]: https://img.shields.io/crates/v/rustamath_mks.svg
[docs-rs]: https://docs.rs/rustamath_mks
[docs-rs-shields]: https://img.shields.io/badge/docs.rs-rustdoc-green.svg

Rustamath MKS is Rust library with support for MKS system of units
and values of physical constants in MKS.

[List of all constants](https://docs.rs/rustamath_mks/0.1.0/rustamath_mks/trait.Mks.html#impl-Mks-for-f64)

Here is an example that demonstrates the basic usage,
it can be run with `cargo test simple_pendulum -- --nocapture` in the source directory.

```rust
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
```

And the output is:

```console
Pendulum length is 1.83 [m]
G on Earth is 9.81 [m / s^2]
Pendulum period is 2.71 [s]
```

(Check the result with any online calculator, for example https://www.omnicalculator.com/physics/simple-pendulum)


This crate provides:

- Physical constants, such as the speed of light, `c`, and gravitational constant, `G`.
  The values are available in the standard MKSA unit system (meters, kilograms, seconds, amperes).
  For example: `let half_speed_of_light = f64::SPEED_OF_LIGHT / 2.0;`.
- MKS unit type, for  example: `assert_eq!(SPEED_OF_LIGHT_UNIT * TIME_UNIT, LIGHT_YEAR_UNIT);`.
- Printing unit as a string, for example: `assert_eq!(&SPEED_OF_LIGHT_UNIT.to_string(), "[m / s]");`.
- Values with units attached, for example:
  `let pendulum_len = MksVal::new(6.0, f64::FOOT, FOOT_UNIT);`.
- Operations on values, for example:
  `let pendulum_len_over_accel = pendulum_len / g;`.
