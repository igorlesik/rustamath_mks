//! MKS constants for physical constants and units of measure.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! This crate provides:
//!
//! - Physical constants, such as the speed of light, `c`, and gravitational constant, `G`.
//!   The values are available in the standard MKSA unit system (meters, kilograms, seconds, amperes).
//!   For example: `let half_speed_of_light = f64::SPEED_OF_LIGHT / 2.0;`.
//! - MKS unit type, for  example: `assert_eq!(SPEED_OF_LIGHT_UNIT * TIME_UNIT, LIGHT_YEAR_UNIT);`.
//! - Unit as a string, for example: `assert_eq!(&SPEED_OF_LIGHT_UNIT.to_string(), "[m / s]");`.
//! - Values with units attached, for example:
//!   `let pendulum_len = MksVal::new(6.0, f64::FOOT, FOOT_UNIT);`.
//! - Operations on values, for example:
//!   `let pendulum_len_over_accel = pendulum_len / g;`.
//!
//! References:
//! - <https://github.com/ampl/gsl/blob/master/const/gsl_const_mks.h>
//!
use std::fmt;

pub mod list;

mod value;
pub use self::value::{MksVal};

/// MKS unit as tuple of integer powers/dimentions (meter, kg, sec, ampere).
///
/// # Example
///
/// ```
/// use rustamath_mks::*;
/// assert_eq!(SPEED_OF_LIGHT_UNIT * SECOND_UNIT, LIGHT_YEAR_UNIT);
/// assert_eq!(&SPEED_OF_LIGHT_UNIT.to_string(), "[m / s]");
/// let _half_speed_of_light = 0.5_f64.to_units(f64::SPEED_OF_LIGHT);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MksUnit {
    m: i8, k: i8, s: i8, a: i8
}

impl std::cmp::PartialEq for MksUnit {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m && self.k == other.k && self.s == other.s && self.a == other.a
    }
}

impl std::ops::Mul for MksUnit {
    type Output = Self;

    /// Return unit type of multiplication of 2 values with unit type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_mks::*;
    /// assert_eq!(SPEED_OF_LIGHT_UNIT * TIME_UNIT, LIGHT_YEAR_UNIT);
    /// ```
    fn mul(self, rhs: Self) -> Self {
        Self {
            m: self.m + rhs.m,
            k: self.k + rhs.k,
            s: self.s + rhs.s,
            a: self.a + rhs.a
        }
    }
}

impl std::ops::Div for MksUnit {
    type Output = Self;

    /// Return unit type of division of 2 values with unit type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_mks::*;
    /// assert_eq!(LIGHT_YEAR_UNIT / SPEED_OF_LIGHT_UNIT, TIME_UNIT);
    /// ```
    fn div(self, rhs: Self) -> Self {
        Self {
            m: self.m - rhs.m,
            k: self.k - rhs.k,
            s: self.s - rhs.s,
            a: self.a - rhs.a
        }
    }
}

impl MksUnit {
    /// Return unit string representation.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_mks::*;
    /// assert_eq!(&VACUUM_PERMITTIVITY_UNIT.as_string(), "s^4 A^2 / m^3 kg");
    /// assert_eq!(&VACUUM_PERMITTIVITY_UNIT.to_string(), "[s^4 A^2 / m^3 kg]");
    /// ```
    pub fn as_string(&self) -> String {
        let mut s = String::new();
        let has_pos_powers = self.m > 0 || self.k > 0 || self.s > 0 || self.a > 0;
        let has_neg_powers = self.m < 0 || self.k < 0 || self.s < 0 || self.a < 0;

        if !has_pos_powers && !has_neg_powers { return s; }

        fn make_power(p: i8, name: &str, count: usize) -> String {
            let mut ps = String::from(name);
            if count > 0 { ps.insert(0, ' '); }
            if p > 1 { ps.push('^'); ps.push_str(&p.to_string()); }
            ps
        }

        if has_pos_powers {
            let mut count: usize = 0;
            if self.m > 0 { s.push_str(&make_power(self.m, "m" , count)); count += 1; }
            if self.k > 0 { s.push_str(&make_power(self.k, "kg", count)); count += 1; }
            if self.s > 0 { s.push_str(&make_power(self.s, "s" , count)); count += 1; }
            if self.a > 0 { s.push_str(&make_power(self.a, "A" , count)); }
        }
        else {
            s.push('1');
        }

        if has_neg_powers {
            s.push_str(" / ");
            let mut count: usize = 0;
            if self.m < 0 { s.push_str(&make_power(-self.m, "m" , count)); count += 1; }
            if self.k < 0 { s.push_str(&make_power(-self.k, "kg", count)); count += 1; }
            if self.s < 0 { s.push_str(&make_power(-self.s, "s" , count)); count += 1; }
            if self.a < 0 { s.push_str(&make_power(-self.a, "A" , count)); }
        }
        s
    }
}

impl fmt::Display for MksUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", &self.as_string())
    }
}

/// Speed of light [m / s]
///
/// # Example
///
/// ```
/// use rustamath_mks::*;
/// assert_eq!(SPEED_OF_LIGHT_UNIT * TIME_UNIT, LIGHT_YEAR_UNIT);
/// assert_eq!(&SPEED_OF_LIGHT_UNIT.to_string(), "[m / s]");
/// let _half_speed_of_light = 0.5_f64.to_units(f64::SPEED_OF_LIGHT);
/// ```
pub const SPEED_OF_LIGHT_UNIT:         MksUnit = MksUnit {m:  1, k:  0, s: -1, a:  0}; // m / s
/// Gravitational constant
pub const GRAVITATIONAL_CONSTANT_UNIT: MksUnit = MksUnit {m:  3, k: -1, s: -2, a:  0}; // m^3 / kg s^2
/// Planks constant
pub const PLANCKS_CONSTANT_H_UNIT:     MksUnit = MksUnit {m:  2, k:  2, s: -1, a:  0}; // kg m^2 / s
/// Planks bar constant
pub const PLANCKS_CONSTANT_HBAR_UNIT:  MksUnit = MksUnit {m:  2, k:  2, s: -1, a:  0}; // kg m^2 / s
/// Astronomical unit of lenght
pub const ASTRONOMICAL_UNIT_UNIT:      MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Light year
pub const LIGHT_YEAR_UNIT:             MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Parsec
pub const PARSEC_UNIT:                 MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Acceleration unit
pub const ACCEL_UNIT:                  MksUnit = MksUnit {m:  1, k:  0, s: -2, a:  0}; // m / s^2
/// Acceleration due to gravity on Earth
pub const GRAV_ACCEL_UNIT:             MksUnit = MksUnit {m:  1, k:  0, s: -2, a:  0}; // m / s^2
/// Electron Volt
pub const ELECTRON_VOLT_UNIT:          MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / s^2
/// Mass of electron
pub const MASS_ELECTRON_UNIT:          MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Mass of muon
pub const MASS_MUON_UNIT:              MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Mass of proton
pub const MASS_PROTON_UNIT:            MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Mass neutron
pub const MASS_NEUTRON_UNIT:           MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Rydberg
pub const RYDBERG_UNIT:                MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / s^2
/// Boltzmann
pub const BOLTZMANN_UNIT:              MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / K s^2
/// Molar of gas
pub const MOLAR_GAS_UNIT:              MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / K mol s^2
/// Standard gas volume
pub const STANDARD_GAS_VOLUME_UNIT:    MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3 / mol
/// Time unit
pub const TIME_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  1, a:  0}; // s
/// One second of time
pub const SECOND_UNIT:                 MksUnit = MksUnit {m:  0, k:  0, s:  1, a:  0}; // s
/// One minute of time
pub const MINUTE_UNIT:                 MksUnit = MksUnit {m:  0, k:  0, s:  1, a:  0}; // s
/// Hour
pub const HOUR_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  1, a:  0}; // s
/// Day
pub const DAY_UNIT:                    MksUnit = MksUnit {m:  0, k:  0, s:  1, a:  0}; // s
/// Week
pub const WEEK_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  1, a:  0}; // s
/// Distance
pub const DISTANCE_UNIT:               MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Meter
pub const METER_UNIT:                  MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Inch
pub const INCH_UNIT:                   MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Foot
pub const FOOT_UNIT:                   MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Yard
pub const YARD_UNIT:                   MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Mile
pub const MILE_UNIT:                   MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Nautical mile
pub const NAUTICAL_MILE_UNIT:          MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Fanthom
pub const FATHOM_UNIT:                 MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Mil
pub const MIL_UNIT:                    MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Point
pub const POINT_UNIT:                  MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Textpoint
pub const TEXPOINT_UNIT:               MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Micron
pub const MICRON_UNIT:                 MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Angstrom
pub const ANGSTROM_UNIT:               MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Hectare
pub const HECTARE_UNIT:                MksUnit = MksUnit {m:  2, k:  0, s:  0, a:  0}; // m^2
/// Acre
pub const ACRE_UNIT:                   MksUnit = MksUnit {m:  2, k:  0, s:  0, a:  0}; // m^2
/// Barn
pub const BARN_UNIT:                   MksUnit = MksUnit {m:  2, k:  0, s:  0, a:  0}; // m^2
/// Liter
pub const LITER_UNIT:                  MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// US gallon
pub const US_GALLON_UNIT:              MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// Quart
pub const QUART_UNIT:                  MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// Pint
pub const PINT_UNIT:                   MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// Cup
pub const CUP_UNIT:                    MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// Fluid ounce
pub const FLUID_OUNCE_UNIT:            MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// Tablespoon
pub const TABLESPOON_UNIT:             MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// Teaspoon
pub const TEASPOON_UNIT:               MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// Canadian gallon
pub const CANADIAN_GALLON_UNIT:        MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// UK gallon
pub const UK_GALLON_UNIT:              MksUnit = MksUnit {m:  3, k:  0, s:  0, a:  0}; // m^3
/// Velocity unit
pub const VELOCITY_UNIT:               MksUnit = MksUnit {m:  1, k:  0, s: -1, a:  0}; // m / s
/// miles/h
pub const MILES_PER_HOUR_UNIT:         MksUnit = MksUnit {m:  1, k:  0, s: -1, a:  0}; // m / s
/// km/h dimentions is [m/s]
pub const KILOMETERS_PER_HOUR_UNIT:    MksUnit = MksUnit {m:  1, k:  0, s: -1, a:  0}; // m / s
/// Knot
pub const KNOT_UNIT:                   MksUnit = MksUnit {m:  1, k:  0, s: -1, a:  0}; // m / s
/// Kilogram
pub const KILOGRAM_UNIT:               MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Pound mass
pub const POUND_MASS_UNIT:             MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Ounce mass
pub const OUNCE_MASS_UNIT:             MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Ton non-metric
pub const TON_UNIT:                    MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Metric ton
pub const METRIC_TON_UNIT:             MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// UK ton
pub const UK_TON_UNIT:                 MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Troy ounce
pub const TROY_OUNCE_UNIT:             MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Carat
pub const CARAT_UNIT:                  MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Unified atomic mass
pub const UNIFIED_ATOMIC_MASS_UNIT:    MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Gram force
pub const GRAM_FORCE_UNIT:             MksUnit = MksUnit {m:  1, k:  1, s: -2, a:  0}; // kg m / s^2
/// Pound force
pub const POUND_FORCE_UNIT:            MksUnit = MksUnit {m:  1, k:  1, s: -2, a:  0}; // kg m / s^2
/// Kilopound force
pub const KILOPOUND_FORCE_UNIT:        MksUnit = MksUnit {m:  1, k:  1, s: -2, a:  0}; // kg m / s^2
/// Poundal
pub const POUNDAL_UNIT:                MksUnit = MksUnit {m:  1, k:  1, s: -2, a:  0}; // kg m / s^2
/// Calorie
pub const CALORIE_UNIT:                MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / s^2
/// Btu
pub const BTU_UNIT:                    MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / s^2
/// Therm
pub const THERM_UNIT:                  MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / s^2
/// Horsepower
pub const HORSEPOWER_UNIT:             MksUnit = MksUnit {m:  2, k:  1, s: -3, a:  0}; // kg m^2 / s^3
/// Bar
pub const BAR_UNIT:                    MksUnit = MksUnit {m: -1, k:  1, s: -2, a:  0}; // kg / m s^2
/// Std atmosphere
pub const STD_ATMOSPHERE_UNIT:         MksUnit = MksUnit {m: -1, k:  1, s: -2, a:  0}; // kg / m s^2
/// Torr
pub const TORR_UNIT:                   MksUnit = MksUnit {m: -1, k:  1, s: -2, a:  0}; // kg / m s^2
/// Meter of mercury
pub const METER_OF_MERCURY_UNIT:       MksUnit = MksUnit {m: -1, k:  1, s: -2, a:  0}; // kg / m s^2
/// Inch of mercury
pub const INCH_OF_MERCURY_UNIT:        MksUnit = MksUnit {m: -1, k:  1, s: -2, a:  0}; // kg / m s^2
/// Inch of water
pub const INCH_OF_WATER_UNIT:          MksUnit = MksUnit {m: -1, k:  1, s: -2, a:  0}; // kg / m s^2
/// Psi
pub const PSI_UNIT:                    MksUnit = MksUnit {m: -1, k:  1, s: -2, a:  0}; // kg / m s^2
/// Poise
pub const POISE_UNIT:                  MksUnit = MksUnit {m: -1, k:  1, s: -1, a:  0}; // kg m^-1 s^-1
/// Stokes
pub const STOKES_UNIT:                 MksUnit = MksUnit {m:  2, k:  0, s: -1, a:  0}; // m^2 / s
/// Stilb
pub const STILB_UNIT:                  MksUnit = MksUnit {m: -2, k:  0, s:  0, a:  0}; // cd / m^2
/// Lumen
pub const LUMEN_UNIT:                  MksUnit = MksUnit {m:  0, k:  0, s:  0, a:  0}; // cd sr
/// Lux
pub const LUX_UNIT:                    MksUnit = MksUnit {m: -2, k:  0, s:  0, a:  0}; // cd sr / m^2
/// Phot
pub const PHOT_UNIT:                   MksUnit = MksUnit {m: -2, k:  0, s:  0, a:  0}; // cd sr / m^2
/// Footcandle
pub const FOOTCANDLE_UNIT:             MksUnit = MksUnit {m: -2, k:  0, s:  0, a:  0}; // cd sr / m^2
/// Lambert
pub const LAMBERT_UNIT:                MksUnit = MksUnit {m: -2, k:  0, s:  0, a:  0}; // cd sr / m^2
/// Footlambert
pub const FOOTLAMBERT_UNIT:            MksUnit = MksUnit {m: -2, k:  0, s:  0, a:  0}; // cd sr / m^2
/// Curie
pub const CURIE_UNIT:                  MksUnit = MksUnit {m:  0, k:  0, s: -1, a:  0}; // 1 / s
/// Roentgen
pub const ROENTGEN_UNIT:               MksUnit = MksUnit {m:  0, k: -1, s:  1, a:  1}; // A s / kg
/// Rad
pub const RAD_UNIT:                    MksUnit = MksUnit {m:  2, k:  0, s: -2, a:  0}; // m^2 / s^2
/// Solar mass
pub const SOLAR_MASS_UNIT:             MksUnit = MksUnit {m:  0, k:  1, s:  0, a:  0}; // kg
/// Bohr radius
pub const BOHR_RADIUS_UNIT:            MksUnit = MksUnit {m:  1, k:  0, s:  0, a:  0}; // m
/// Newton force
pub const NEWTON_UNIT:                 MksUnit = MksUnit {m:  1, k:  1, s: -2, a:  0}; // kg m / s^2
/// Dyne
pub const DYNE_UNIT:                   MksUnit = MksUnit {m:  1, k:  1, s: -2, a:  0}; // kg m / s^2
/// Joule
pub const JOULE_UNIT:                  MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / s^2
/// Erg
pub const ERG_UNIT:                    MksUnit = MksUnit {m:  2, k:  1, s: -2, a:  0}; // kg m^2 / s^2
/// STEFAN_BOLTZMANN_CONSTANT
pub const STEFAN_BOLTZMANN_CONSTANT_UNIT: MksUnit = MksUnit {m:  0, k:  1, s: -3, a:  0}; // kg / K^4 s^3
/// THOMSON_CROSS_SECTION
pub const THOMSON_CROSS_SECTION_UNIT:  MksUnit = MksUnit {m:  2, k:  0, s:  0, a:  0}; // m^2
/// Bohr magneton
pub const BOHR_MAGNETON_UNIT:          MksUnit = MksUnit {m:  2, k:  0, s:  0, a:  1}; // A m^2
/// Nuclear magneton
pub const NUCLEAR_MAGNETON_UNIT:       MksUnit = MksUnit {m:  2, k:  0, s:  0, a:  1}; // A m^2
/// Electron magnetic moment
pub const ELECTRON_MAGNETIC_MOMENT_UNIT: MksUnit = MksUnit {m:  2, k:  0, s:  0, a:  1}; // A m^2
/// Photon magnetic moment
pub const PROTON_MAGNETIC_MOMENT_UNIT: MksUnit = MksUnit {m:  2, k:  0, s:  0, a:  1}; // A m^2
/// Faraday
pub const FARADAY_UNIT:                MksUnit = MksUnit {m:  0, k:  0, s:  1, a:  1}; // A s / mol
/// Electron charge
pub const ELECTRON_CHARGE_UNIT:        MksUnit = MksUnit {m:  0, k:  0, s:  1, a:  1}; // A s
/// VACUUM_PERMITTIVITY
pub const VACUUM_PERMITTIVITY_UNIT:    MksUnit = MksUnit {m: -3, k: -1, s:  4, a:  2}; // A^2 s^4 / kg m^3
/// VACUUM_PERMEABILITY
pub const VACUUM_PERMEABILITY_UNIT:    MksUnit = MksUnit {m:  1, k:  1, s: -2, a: -2}; // kg m / A^2 s^2
/// Debye
pub const DEBYE_UNIT:                  MksUnit = MksUnit {m: -2, k:  0, s:  2, a:  1}; // A s^2 / m^2
/// Gauss
pub const GAUSS_UNIT:                  MksUnit = MksUnit {m:  0, k:  1, s: -2, a: -1}; // kg / A s^2
/// Ampere
pub const AMPERE_UNIT:                 MksUnit = MksUnit {m:  0, k:  0, s:  0, a:  1}; // A


/// Constant factors for MKS constants and units.
///
/// Physical constants, such as the speed of light, `c`, and gravitational constant, `G`.
/// The values are available in the standard MKSA unit system (meters, kilograms, seconds, amperes).
///
/// # Example
///
/// ```
/// use rustamath_mks::*;
/// use assert_float_eq::*;
/// assert_float_absolute_eq!(
///     1.0_f64.to_units(f64::SPEED_OF_LIGHT).in_units(f64::KILOMETERS_PER_HOUR),
///     1.079e9_f64, 1.0e6);
/// ```
pub trait Mks
where
    Self: Copy,
    Self: core::ops::Mul<Output = Self>,
    Self: core::ops::Div<Output = Self>,
{

    /// Scale a number by Unit.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath_mks::{Mks};
    /// assert_eq!(1.2_f64.to_units(f64::SPEED_OF_LIGHT), 2.99792458e8_f64 * 1.2)
    /// ```
    fn to_units(&self, unit: Self) -> Self {
        *self * unit
    }

    /// Divide a number by Unit.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_mks::*;
    /// use assert_float_eq::*;
    /// assert!(SPEED_OF_LIGHT_UNIT == KILOMETERS_PER_HOUR_UNIT);
    /// assert!(SPEED_OF_LIGHT_UNIT != TIME_UNIT);
    /// assert_float_absolute_eq!(
    ///     1.0_f64.to_units(f64::SPEED_OF_LIGHT).in_units(f64::KILOMETERS_PER_HOUR),
    ///     1.079e9_f64, 1.0e6);
    /// ```
    fn in_units(&self, unit: Self) -> Self {
        *self / unit
    }

    /// Speed of light
    const SPEED_OF_LIGHT: Self;
    /// Gravitational constant
    const GRAVITATIONAL_CONSTANT: Self;
    /// Plank h constant
    const PLANCKS_CONSTANT_H: Self;
    /// Plank h-bar constant
    const PLANCKS_CONSTANT_HBAR: Self;
    /// Astronomical unit
    const ASTRONOMICAL_UNIT: Self;
    /// Light year
    const LIGHT_YEAR: Self;
    /// Parsec
    const PARSEC: Self;
    /// Acceleration
    const GRAV_ACCEL: Self;
    /// Electron Volt
    const ELECTRON_VOLT: Self;
    /// Mass of electron
    const MASS_ELECTRON: Self;
    /// Mass of muon
    const MASS_MUON: Self;
    /// Mass of proton
    const MASS_PROTON: Self;
    /// Mass of neutron
    const MASS_NEUTRON: Self;
    /// Rydberg
    const RYDBERG:Self;
    /// Boltzmann
    const BOLTZMANN: Self;
    /// Molar of gas
    const MOLAR_GAS: Self;
    /// Standard gas volume
    const STANDARD_GAS_VOLUME:Self;
    /// One second of time
    const SECOND: Self;
    /// One minute of time, 60s
    const MINUTE: Self;
    /// Hour
    const HOUR: Self;
    /// Day
    const DAY: Self;
    /// Week
    const WEEK: Self;
    /// Meter
    const METER: Self;
    /// Inch
    const INCH: Self;
    /// Foot
    const FOOT: Self;
    /// Yard
    const YARD: Self;
    /// Mile
    const MILE:Self;
    /// Nautical mile
    const NAUTICAL_MILE: Self;
    /// Fathom
    const FATHOM: Self;
    /// Mil
    const MIL: Self;
    /// Point
    const POINT: Self;
    /// Textpoint
    const TEXPOINT: Self;
    /// Micron
    const MICRON: Self;
    /// Angstrom
    const ANGSTROM: Self;
    /// Hectare
    const HECTARE: Self;
    /// Acre
    const ACRE: Self;
    /// Barn
    const BARN: Self;
    /// Liter
    const LITER: Self;
    /// US gallon
    const US_GALLON: Self;
    /// Quart
    const QUART: Self;
    /// Pint
    const PINT: Self;
    /// Cup
    const CUP: Self;
    /// Fluid ounce
    const FLUID_OUNCE: Self;
    /// Tablespoon
    const TABLESPOON: Self;
    /// Teaspoon
    const TEASPOON: Self;
    /// Canadian gallon
    const CANADIAN_GALLON: Self;
    /// UK gallon
    const UK_GALLON: Self;
    /// miles/h
    const MILES_PER_HOUR: Self;
    /// km/h
    const KILOMETERS_PER_HOUR: Self;
    /// Knot
    const KNOT: Self;
    /// Kilogram
    const KILOGRAM: Self;
    /// Pound
    const POUND_MASS: Self;
    /// Ounce
    const OUNCE_MASS: Self;
    /// Ton
    const TON: Self;
    /// Metric ton
    const METRIC_TON: Self;
    /// UK ton
    const UK_TON: Self;
    /// Troy ounce
    const TROY_OUNCE: Self;
    /// Carat
    const CARAT: Self;
    /// Unified atomic mass
    const UNIFIED_ATOMIC_MASS: Self;
    /// Gram force
    const GRAM_FORCE: Self;
    /// Pound force
    const POUND_FORCE: Self;
    /// Kilopound force
    const KILOPOUND_FORCE: Self;
    /// Poundal
    const POUNDAL: Self;
    /// Calorie
    const CALORIE: Self;
    /// BTU
    const BTU: Self;
    /// Therm
    const THERM: Self;
    /// Horsepower
    const HORSEPOWER: Self;
    /// Bar
    const BAR: Self;
    /// STD atmosphere
    const STD_ATMOSPHERE: Self;
    /// Torr
    const TORR: Self;
    /// Meter of mercury
    const METER_OF_MERCURY: Self;
    /// Inch of mercury
    const INCH_OF_MERCURY: Self;
    /// Inch of water
    const INCH_OF_WATER: Self;
    /// PSI
    const PSI: Self;
    /// Poise
    const POISE: Self;
    /// Stokes
    const STOKES: Self;
    /// Stilb
    const STILB: Self;
    /// Lumen
    const LUMEN: Self;
    /// Lux
    const LUX: Self;
    /// Phot
    const PHOT: Self;
    /// Footcandle
    const FOOTCANDLE: Self;
    /// Lambert
    const LAMBERT: Self;
    /// Footlambert
    const FOOTLAMBERT: Self;
    /// Curie
    const CURIE: Self;
    /// Roentgen
    const ROENTGEN: Self;
    /// Rad
    const RAD: Self;
    /// Solar mass
    const SOLAR_MASS: Self;
    /// Bohr radius
    const BOHR_RADIUS: Self;
    /// Newton
    const NEWTON: Self;
    /// Dyne
    const DYNE: Self;
    /// Joule
    const JOULE: Self;
    /// Erg
    const ERG: Self;
    /// STEFAN_BOLTZMANN_CONSTANT
    const STEFAN_BOLTZMANN_CONSTANT: Self;
    /// THOMSON_CROSS_SECTION
    const THOMSON_CROSS_SECTION: Self;
    /// Bohr magneton
    const BOHR_MAGNETON: Self;
    /// Nuclear magneton
    const NUCLEAR_MAGNETON: Self;
    /// Electron magnetic moment
    const ELECTRON_MAGNETIC_MOMENT: Self;
    /// Proton magnetic moment
    const PROTON_MAGNETIC_MOMENT: Self;
    /// Faraday
    const FARADAY: Self;
    /// Electron charge
    const ELECTRON_CHARGE: Self;
    /// VACUUM_PERMITTIVITY
    const VACUUM_PERMITTIVITY: Self;
    /// VACUUM_PERMEABILITY
    const VACUUM_PERMEABILITY: Self;
    /// Debye
    const DEBYE: Self;
    /// Gauss
    const GAUSS: Self;
}

impl Mks for f64 {
    const SPEED_OF_LIGHT:           f64 = 2.99792458e8_f64; // m / s
    const GRAVITATIONAL_CONSTANT:   f64 = 6.673e-11_f64; // m^3 / kg s^2
    const PLANCKS_CONSTANT_H:       f64 = 6.62606896e-34_f64; // kg m^2 / s
    const PLANCKS_CONSTANT_HBAR:    f64 = 1.05457162825e-34_f64; // kg m^2 / s
    const ASTRONOMICAL_UNIT:        f64 = 1.49597870691e11_f64; // m
    const LIGHT_YEAR:               f64 = 9.46053620707e15_f64; // m
    const PARSEC:                   f64 = 3.08567758135e16_f64; /* m */
    const GRAV_ACCEL:               f64 = 9.80665e0_f64; /* m / s^2 */
    const ELECTRON_VOLT:            f64 = 1.602176487e-19_f64; /* kg m^2 / s^2 */
    const MASS_ELECTRON:            f64 = 9.10938188e-31_f64; /* kg */
    const MASS_MUON:                f64 = 1.88353109e-28_f64; /* kg */
    const MASS_PROTON:              f64 = 1.67262158e-27_f64; /* kg */
    const MASS_NEUTRON:             f64 = 1.67492716e-27_f64; /* kg */
    const RYDBERG:                  f64 = 2.17987196968e-18_f64; /* kg m^2 / s^2 */
    const BOLTZMANN:                f64 = 1.3806504e-23_f64; /* kg m^2 / K s^2 */
    const MOLAR_GAS:                f64 = 8.314472e0_f64; /* kg m^2 / K mol s^2 */
    const STANDARD_GAS_VOLUME:      f64 = 2.2710981e-2_f64; /* m^3 / mol */
    const SECOND:                   f64 = 1.0_f64; // s
    const MINUTE:                   f64 = 6.0e1_f64; // s
    const HOUR:                     f64 = 3.6e3_f64; // s
    const DAY:                      f64 = 8.64e4_f64; // s
    const WEEK:                     f64 = 6.048e5_f64; // s
    const METER:                    f64 = 1.0_f64; // m
    const INCH:                     f64 = 2.54e-2_f64; /* m */
    const FOOT:                     f64 = 3.048e-1_f64; /* m */
    const YARD:                     f64 = 9.144e-1_f64; /* m */
    const MILE:                     f64 = 1.609344e3_f64; /* m */
    const NAUTICAL_MILE:            f64 = 1.852e3_f64; /* m */
    const FATHOM:                   f64 = 1.8288e0_f64; /* m */
    const MIL:                      f64 = 2.54e-5_f64; /* m */
    const POINT:                    f64 = 3.52777777778e-4_f64; /* m */
    const TEXPOINT:                 f64 = 3.51459803515e-4_f64; /* m */
    const MICRON:                   f64 = 1e-6_f64; /* m */
    const ANGSTROM:                 f64 = 1e-10_f64; /* m */
    const HECTARE:                  f64 = 1e4_f64; /* m^2 */
    const ACRE:                     f64 = 4.04685642241e3_f64; /* m^2 */
    const BARN:                     f64 = 1e-28_f64; /* m^2 */
    const LITER:                    f64 = 1e-3_f64; /* m^3 */
    const US_GALLON:                f64 = 3.78541178402e-3_f64; /* m^3 */
    const QUART:                    f64 = 9.46352946004e-4_f64; /* m^3 */
    const PINT:                     f64 = 4.73176473002e-4_f64; /* m^3 */
    const CUP:                      f64 = 2.36588236501e-4_f64; /* m^3 */
    const FLUID_OUNCE:              f64 = 2.95735295626e-5_f64; /* m^3 */
    const TABLESPOON:               f64 = 1.47867647813e-5_f64; /* m^3 */
    const TEASPOON:                 f64 = 4.92892159375e-6_f64; /* m^3 */
    const CANADIAN_GALLON:          f64 = 4.54609e-3_f64; /* m^3 */
    const UK_GALLON:                f64 = 4.546092e-3_f64; /* m^3 */
    const MILES_PER_HOUR:           f64 = 4.4704e-1_f64; /* m / s */
    const KILOMETERS_PER_HOUR:      f64 = 2.77777777778e-1_f64; // m / s
    const KNOT:                     f64 = 5.14444444444e-1_f64; /* m / s */
    const KILOGRAM:                 f64 = 1.0_f64; // kg
    const POUND_MASS:               f64 = 4.5359237e-1_f64; /* kg */
    const OUNCE_MASS:               f64 = 2.8349523125e-2_f64; /* kg */
    const TON:                      f64 = 9.0718474e2_f64; /* kg */
    const METRIC_TON:               f64 = 1e3_f64; /* kg */
    const UK_TON:                   f64 = 1.0160469088e3_f64; /* kg */
    const TROY_OUNCE:               f64 = 3.1103475e-2_f64; /* kg */
    const CARAT:                    f64 = 2e-4_f64; /* kg */
    const UNIFIED_ATOMIC_MASS:      f64 = 1.660538782e-27_f64; /* kg */
    const GRAM_FORCE:               f64 = 9.80665e-3_f64; /* kg m / s^2 */
    const POUND_FORCE:              f64 = 4.44822161526e0_f64; /* kg m / s^2 */
    const KILOPOUND_FORCE:          f64 = 4.44822161526e3_f64; /* kg m / s^2 */
    const POUNDAL:                  f64 = 1.38255e-1_f64; /* kg m / s^2 */
    const CALORIE:                  f64 = 4.1868e0_f64; /* kg m^2 / s^2 */
    const BTU:                      f64 = 1.05505585262e3_f64; /* kg m^2 / s^2 */
    const THERM:                    f64 = 1.05506e8_f64; /* kg m^2 / s^2 */
    const HORSEPOWER:               f64 = 7.457e2_f64; /* kg m^2 / s^3 */
    const BAR:                      f64 = 1e5_f64; /* kg / m s^2 */
    const STD_ATMOSPHERE:           f64 = 1.01325e5_f64; /* kg / m s^2 */
    const TORR:                     f64 = 1.33322368421e2_f64; /* kg / m s^2 */
    const METER_OF_MERCURY:         f64 = 1.33322368421e5_f64; /* kg / m s^2 */
    const INCH_OF_MERCURY:          f64 = 3.38638815789e3_f64; /* kg / m s^2 */
    const INCH_OF_WATER:            f64 = 2.490889e2_f64; /* kg / m s^2 */
    const PSI:                      f64 = 6.89475729317e3_f64; /* kg / m s^2 */
    const POISE:                    f64 = 1e-1_f64; /* kg m^-1 s^-1 */
    const STOKES:                   f64 = 1e-4_f64; /* m^2 / s */
    const STILB:                    f64 = 1e4_f64; /* cd / m^2 */
    const LUMEN:                    f64 = 1e0_f64; /* cd sr */
    const LUX:                      f64 = 1e0_f64; /* cd sr / m^2 */
    const PHOT:                     f64 = 1e4_f64; /* cd sr / m^2 */
    const FOOTCANDLE:               f64 = 1.076e1_f64; /* cd sr / m^2 */
    const LAMBERT:                  f64 = 1e4_f64; /* cd sr / m^2 */
    const FOOTLAMBERT:              f64 = 1.07639104e1_f64; /* cd sr / m^2 */
    const CURIE:                    f64 = 3.7e10_f64; /* 1 / s */
    const ROENTGEN:                 f64 = 2.58e-4_f64; /* A s / kg */
    const RAD:                      f64 = 1e-2_f64; /* m^2 / s^2 */
    const SOLAR_MASS:               f64 = 1.98892e30_f64; /* kg */
    const BOHR_RADIUS:              f64 = 5.291772083e-11_f64; /* m */
    const NEWTON:                   f64 = 1e0_f64; /* kg m / s^2 */
    const DYNE:                     f64 = 1e-5_f64; /* kg m / s^2 */
    const JOULE:                    f64 = 1e0_f64; /* kg m^2 / s^2 */
    const ERG:                      f64 = 1e-7_f64; /* kg m^2 / s^2 */
    const STEFAN_BOLTZMANN_CONSTANT:f64 = 5.67040047374e-8_f64; /* kg / K^4 s^3 */
    const THOMSON_CROSS_SECTION:    f64 = 6.65245893699e-29_f64; /* m^2 */
    const BOHR_MAGNETON:            f64 = 9.27400899e-24_f64; /* A m^2 */
    const NUCLEAR_MAGNETON:         f64 = 5.05078317e-27_f64; /* A m^2 */
    const ELECTRON_MAGNETIC_MOMENT: f64 = 9.28476362e-24_f64; /* A m^2 */
    const PROTON_MAGNETIC_MOMENT:   f64 = 1.410606633e-26_f64; /* A m^2 */
    const FARADAY:                  f64 = 9.64853429775e4_f64; /* A s / mol */
    const ELECTRON_CHARGE:          f64 = 1.602176487e-19_f64; /* A s */
    const VACUUM_PERMITTIVITY:      f64 = 8.854187817e-12_f64; /* A^2 s^4 / kg m^3 */
    const VACUUM_PERMEABILITY:      f64 = 1.25663706144e-6_f64; /* kg m / A^2 s^2 */
    const DEBYE:                    f64 = 3.33564095198e-30_f64; /* A s^2 / m^2 */
    const GAUSS:                    f64 = 1e-4_f64; /* kg / A s^2 */
}