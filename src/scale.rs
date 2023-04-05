//! Dimensionless scaling factors.
//!
//! (c) Igor Lesik 2023
//! MIT license
//!

/// Dimensionless scaling factors
pub trait Scale:
    Copy +
    core::ops::Mul<Output = Self> +
    core::ops::Div<Output = Self>
{

    /// Scale a number by factor.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath::constant::scale::{Scale};
    /// assert_eq!(2.1f64.scale(f64::MEGA), 2100_000.0_f64)
    /// ```
    fn scale(&self, factor: Self) -> Self {
        *self * factor
    }

    /// Divide a number by factor.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath::constant::scale::{Scale};
    /// assert_eq!(2.1f64.scale(f64::MEGA).in_units(f64::KILO), 2100.0_f64)
    /// ```
    fn in_units(&self, factor: Self) -> Self {
        *self / factor
    }

    /// 10^24
    const YOTTA: Self;
    /// 10^21
    const ZETTA: Self;
    /// 10^18
    const EXA: Self;
    /// 10^15
    const PETA: Self;
    /// 10^12
    const TERA: Self;
    /// 10^9
    const GIGA: Self;
    /// 10^6
    const MEGA: Self;
    /// 10^3
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath::constant::scale::{Scale};
    /// assert_eq!(f64::KILO, 1000.0_f64)
    /// ```
    const KILO: Self;
    /// -3
    const MILLI: Self;
    /// -6
    const MICRO: Self;
    /// -9
    const NANO: Self;
    /// -12
    const PICO: Self;
    /// -15
    const FEMTO: Self;
    /// -18
    const ATTO: Self;
    /// -21
    const ZEPTO: Self;
    /// -24
    const YOCTO: Self;

    /// Kilobyte, 1024 bytes
    const KILOBYTE: Self;
    /// Megabyte
    const MEGABYTE: Self;
    /// Terabyte
    const TERABYTE: Self;
    /// Petabyte
    const PETABYTE: Self;
}

impl Scale for f64 {
    const YOTTA: f64 = 1.0e24_f64;
    const ZETTA: f64 = 1.0e21_f64;
    const EXA:   f64 = 1.0e18_f64;
    const PETA:  f64 = 1.0e15_f64;
    const TERA:  f64 = 1.0e12_f64;
    const GIGA:  f64 = 1.0e9_f64;
    const MEGA:  f64 = 1.0e6_f64;
    const KILO:  f64 = 1.0e3_f64;
    const MILLI: f64 = 1.0e-3_f64;
    const MICRO: f64 = 1.0e-6_f64;
    const NANO:  f64 = 1.0e-9_f64;
    const PICO:  f64 = 1.0e-12_f64;
    const FEMTO: f64 = 1.0e-15_f64;
    const ATTO:  f64 = 1.0e-18_f64;
    const ZEPTO: f64 = 1.0e-21_f64;
    const YOCTO: f64 = 1.0e-24_f64;

    const KILOBYTE: f64 = 1024.0_f64;
    const MEGABYTE: f64 = Self::KILOBYTE * Self::KILOBYTE;
    const TERABYTE: f64 = Self::MEGABYTE * Self::KILOBYTE;
    const PETABYTE: f64 = Self::TERABYTE * Self::KILOBYTE;
}

impl Scale for f32 {
    const YOTTA: f32 = 1.0e24_f32;
    const ZETTA: f32 = 1.0e21_f32;
    const EXA:   f32 = 1.0e18_f32;
    const PETA:  f32 = 1.0e15_f32;
    const TERA:  f32 = 1.0e12_f32;
    const GIGA:  f32 = 1.0e9_f32;
    const MEGA:  f32 = 1.0e6_f32;
    const KILO:  f32 = 1.0e3_f32;
    const MILLI: f32 = 1.0e-3_f32;
    const MICRO: f32 = 1.0e-6_f32;
    const NANO:  f32 = 1.0e-9_f32;
    const PICO:  f32 = 1.0e-12_f32;
    const FEMTO: f32 = 1.0e-15_f32;
    const ATTO:  f32 = 1.0e-18_f32;
    const ZEPTO: f32 = 1.0e-21_f32;
    const YOCTO: f32 = 1.0e-24_f32;

    const KILOBYTE: f32 = 1024.0_f32;
    const MEGABYTE: f32 = Self::KILOBYTE * Self::KILOBYTE;
    const TERABYTE: f32 = Self::MEGABYTE * Self::KILOBYTE;
    const PETABYTE: f32 = Self::TERABYTE * Self::KILOBYTE;
}
