//! MKS value bundled with its unit of measurement
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!


use super::MksUnit;

/// MKS value bundled with its unit of measurement.
///
/// # Example
///
/// ```
/// # use rustamath_mks::*;
/// // simple pendulum period `T = 2*Pi*sqrt(L/g)`
/// let pendulum_len = MksVal::new(6.0, f64::FOOT, FOOT_UNIT);
/// let g = MksVal::new(1.0, f64::GRAV_ACCEL, GRAV_ACCEL_UNIT);
/// let pendulum_len_over_accel = pendulum_len / g;
/// assert!(pendulum_len_over_accel.unit == TIME_UNIT * TIME_UNIT);
/// let pi_x_2 = MksVal::new_scalar(2.0 * std::f64::consts::PI);
/// let period = pi_x_2 * pendulum_len_over_accel.sqrt();
/// assert!(period.unit == TIME_UNIT);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MksVal {
    /// Value
    pub val: f64,
    /// Unit of measure
    pub unit: MksUnit
}

impl MksVal {
    /// Create new MKS value
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_mks::*;
    /// let half_speed_of_light = MksVal::new(0.5, f64::SPEED_OF_LIGHT, SPEED_OF_LIGHT_UNIT);
    /// assert!(half_speed_of_light.unit != MASS_PROTON_UNIT);
    /// assert!(half_speed_of_light.unit == VELOCITY_UNIT);
    /// ```
    pub fn new(val: f64, factor: f64, unit: MksUnit) -> MksVal {
        MksVal {
            val: val * factor,
            unit
        }
    }

    /// Value without any units of measure
    pub fn new_scalar(val: f64) -> MksVal {
        MksVal {
            val,
            unit: MksUnit {m: 0, k: 0, s: 0, a: 0}
        }
    }

    /// Find square root value and adjust units
    pub fn sqrt(&self) -> Self {
        Self {
            val: self.val.sqrt(),
            unit: MksUnit {m: self.unit.m/2, k: self.unit.k/2, s: self.unit.s/2, a: self.unit.a/2}
        }
    }

    /// Find cubic root value and adjust units
    pub fn cbrt(&self) -> Self {
        Self {
            val: self.val.cbrt(),
            unit: MksUnit {m: self.unit.m/3, k: self.unit.k/3, s: self.unit.s/3, a: self.unit.a/3}
        }
    }

    /// Raise to integer power and adjust units
    pub fn pow(&self, n: i8) -> Self {
        Self {
            val: self.val.powi(n.into()),
            unit: MksUnit {
                m: self.unit.m * n ,
                k: self.unit.k * n,
                s: self.unit.s * n,
                a: self.unit.a * n
            }
        }
    }
}

impl std::ops::Add for MksVal {
    type Output = Self;

    /// Add 2 MKS values respecting their units
    ///
    /// # Example
    ///
    /// ```should_panic
    /// use rustamath_mks::*;
    /// let half_speed_of_light = MksVal::new(0.5, f64::SPEED_OF_LIGHT, SPEED_OF_LIGHT_UNIT);
    /// let speed_of_light = half_speed_of_light + half_speed_of_light;
    /// // we are going to panic on next line
    /// let speed_of_light = half_speed_of_light + MksVal::new(1.0, f64::MASS_PROTON, MASS_PROTON_UNIT);
    /// ```
    fn add(self, rhs: Self) -> Self {
        debug_assert!(self.unit == rhs.unit);
        Self {
            unit: self.unit,
            val: self.val + rhs.val
        }
    }
}


impl std::ops::Sub for MksVal {
    type Output = Self;

    /// Subtruct 2 MKS values respecting their units
    ///
    /// # Example
    ///
    /// ```should_panic
    /// use rustamath_mks::*;
    /// let half_speed_of_light = MksVal::new(0.5, f64::SPEED_OF_LIGHT, SPEED_OF_LIGHT_UNIT);
    /// let speed_of_light = half_speed_of_light + half_speed_of_light;
    /// // we are going to panic on next line
    /// let speed_of_light = half_speed_of_light - MksVal::new(1.0, f64::MASS_PROTON, MASS_PROTON_UNIT);
    /// ```
    fn sub(self, rhs: Self) -> Self {
        debug_assert!(self.unit == rhs.unit);
        Self {
            unit: self.unit,
            val: self.val - rhs.val
        }
    }
}

impl std::ops::Mul for MksVal {
    type Output = Self;

    /// Multiply 2 MKS values calculating result unit
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_mks::*;
    /// let half_speed_of_light = MksVal::new(0.5, f64::SPEED_OF_LIGHT, SPEED_OF_LIGHT_UNIT);
    /// let distance_after_2_weeks = half_speed_of_light * MksVal::new(2.0, f64::WEEK, WEEK_UNIT);
    /// assert!(distance_after_2_weeks.unit == DISTANCE_UNIT);
    /// ```
    fn mul(self, rhs: Self) -> Self {
        Self {
            unit: self.unit * rhs.unit,
            val: self.val * rhs.val
        }
    }
}

impl std::ops::Div for MksVal {
    type Output = Self;

    /// Divide 2 MKS values calculating result unit
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_mks::*;
    /// # use assert_float_eq::*;
    /// let speed_of_light = MksVal::new(1.0, f64::SPEED_OF_LIGHT, SPEED_OF_LIGHT_UNIT);
    /// let traveling_time = MksVal::new(1.0, f64::LIGHT_YEAR, LIGHT_YEAR_UNIT) / speed_of_light;
    /// assert!(traveling_time.unit == TIME_UNIT);
    /// assert_float_relative_eq!(traveling_time.val, f64::DAY * 365.25, 1.0e-4); // https://en.wikipedia.org/wiki/Light-year
    /// ```
    fn div(self, rhs: Self) -> Self {
        Self {
            unit: self.unit / rhs.unit,
            val: self.val / rhs.val
        }
    }
}