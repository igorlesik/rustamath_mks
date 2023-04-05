//! List of MKS constants names
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!

use super::*;

/// List of MKS constants name
pub enum Name {
    /// Speef of light
    SpeedOfLight,
    /// Gravitational constant
    GravitationalConstant,
    /// Plancks h constant
    PlancksConstantH,
    /// Plancks h-bar constant
    PlancksConstantHBar,
    /// Astronomical unit
    AstronomicalUnit,
    /// Light year
    LightYear,
    /// Parsec
    Parsec,
    /// Acceleration
    GravAccel,
    /// Electron Volt
    ElectronVolt,
    /// Mass of electron
    MassElectron,
    /// Mass of muon
    MassMuon,
    /// Mass of proton
    MassProton,
    /// Mass of neutron
    MassNeutron,
    /// Rydberg
    Rydberg,
    /// Boltzmann
    Boltzmann,
    /// Molar
    MolarGas,
    /// Standard gas volume
    StandardGasVolume,
    /// Second of time
    Second,
    /// Minute of time
    Minute,
    /// Hour
    Hour,
    /// Day
    Day,
    /// Week
    Week,
    /// Meter
    Meter,
    /// Inch
    Inch,
    /// Foot
    Foot,
    /// Yard
    Yard,
    /// Mile
    Mile,
    /// NauticalMile
    NauticalMile,
    /// Fathom
    Fathom,
    /// Mil
    Mil,
    /// Point
    Point,
    /// Textpoint
    Textpoint,
    /// Micron
    Micron,
    /// Angstrom
    Angstrom,
    /// Hectare
    Hectare,
    /// Acre
    Acre,
    /// Barn
    Barn,
    /// Liter,
    Liter,
    /// US gallon
    UsGallon,
    /// Quart
    Quart,
    /// Pint
    Pint,
    /// Cup
    Cup,
    /// Fluid ounce
    FluidOunce,
    /// Tablespoon
    Tablespoon,
    /// Teaspoon,
    Teaspoon,
    /// Canadian gallon
    CanadianGallon,
    /// UK gallon
    UkGallon,
    /// miles/h
    MilesPerHour,
    /// km/h
    KilometersPerHour,
    /// Knot
    Knot,
    /// kg
    Kilogram,
    /// Pound mass
    PoundMass,
    /// Ounce mass
    OunceMass,
    /// Ton
    Ton,
    /// Metric ton
    MetricTon,
    /// UK ton
    UkTon,
    /// Troy ounce
    TroyOunce,
    /// Carat
    Carat,
    /// Unified atomic mass
    UnifiedAtomicMass,
    /// Gram force
    GramForce,
    /// Pound force
    PoundForce,
    /// Kilopound force
    KilopoundForce,
    /// Poundal
    Poundal,
    /// Calorie
    Calorie,
    /// BTU
    Btu,
    /// Therm
    Therm,
    /// Horsepower
    Horsepower,
    /// Bar
    Bar,
    /// Standard atmosphere
    StdAtmosphere,
    /// Torr
    Torr,
    /// Meter of mercury
    MeterOfMercury,
    /// Inch of mercury
    InchOfMercury,
    /// Inch of water
    InchOfWater,
    /// PSI
    Psi,
    /// Poise
    Poise,
    /// Stokes
    Stokes,
    /// STILB
    Stilb,
    /// Lumen
    Lumen,
    /// Lux
    Lux,
    /// Phot
    Phot,
    /// Footcandle
    Footcandle,
    /// Lambert
    Lambert,
    /// Footlambert
    Footlambert,
    /// Curie
    Curie,
    /// Roentgen
    Roentgen,
    /// Rad
    Rad,
    /// Solar mass
    SolarMass,
    /// Bohr radius
    BohrRadius,
    /// Newton
    Newton,
    /// Dyne
    Dyne,
    /// Joule
    Joule,
    /// Erg
    Erg,
    /// STEFAN_BOLTZMANN_CONSTANT
    StefanBolzmannConstant,
    /// THOMSON_CROSS_SECTION
    ThomsonCrossSection,
    /// Bohr magneton
    BohrMagneton,
    /// Nuclear magneton
    NuclearMagneton,
    /// ELECTRON_MAGNETIC_MOMENT
    ElectronMagneticMoment,
    /// PROTON_MAGNETIC_MOMENT
    ProtonMagneticMoment,
    /// Faraday
    Faraday,
    /// Electron charge
    ElectronCharge,
    /// VACUUM_PERMITTIVITY
    VacuumPermittivity,
    /// VACUUM_PERMEABILITY
    VacuumPermeability,
    /// Debye
    Debye,
    /// Gauss
    Gauss,
}

/// Record in the list of constants
pub type MksTuple<'a> = (Name, MksUnit, f64, &'a str);

/// List of MKS units with dimentions and factors
pub const UNITS: [MksTuple; 104] = [
    (Name::SpeedOfLight,           SPEED_OF_LIGHT_UNIT,           f64::SPEED_OF_LIGHT,          "Speed of light"),
    (Name::GravitationalConstant,  GRAVITATIONAL_CONSTANT_UNIT,   f64::GRAVITATIONAL_CONSTANT,  "Gravitational constant"),
    (Name::PlancksConstantH,       PLANCKS_CONSTANT_H_UNIT,       f64::PLANCKS_CONSTANT_H,      "Planck's constant h"),
    (Name::PlancksConstantHBar,    PLANCKS_CONSTANT_HBAR_UNIT,    f64::PLANCKS_CONSTANT_HBAR,   "Planck's constant h bar"),
    (Name::AstronomicalUnit,       ASTRONOMICAL_UNIT_UNIT,        f64::ASTRONOMICAL_UNIT,       "Astronomical unit"),
    (Name::LightYear,              LIGHT_YEAR_UNIT,               f64::LIGHT_YEAR,              "Light year"),
    (Name::Parsec,                 PARSEC_UNIT,                   f64::PARSEC,                  "Parsec"),
    (Name::GravAccel,              GRAV_ACCEL_UNIT,               f64::GRAV_ACCEL,              "Grav Acceleration"),
    (Name::ElectronVolt,           ELECTRON_VOLT_UNIT,            f64::ELECTRON_VOLT,           "Electron Volt"),
    (Name::MassElectron,           MASS_ELECTRON_UNIT,            f64::MASS_ELECTRON,           "Mass of electron"),
    (Name::MassMuon,               MASS_MUON_UNIT,                f64::MASS_MUON,               "Mass of muon"),
    (Name::MassProton,             MASS_PROTON_UNIT,              f64::MASS_PROTON,             "Mass of proton"),
    (Name::MassNeutron,            MASS_NEUTRON_UNIT,             f64::MASS_NEUTRON,            "Mass of neutron"),
    (Name::Rydberg,                RYDBERG_UNIT,                  f64::RYDBERG,                 "Rydberg"),
    (Name::Boltzmann,              BOLTZMANN_UNIT,                f64::BOLTZMANN,               "Boltzmann"),
    (Name::MolarGas,               MOLAR_GAS_UNIT,                f64::MOLAR_GAS,               "Molar gas"),
    (Name::StandardGasVolume,      STANDARD_GAS_VOLUME_UNIT,      f64::STANDARD_GAS_VOLUME,     "Standard gas volume"),
    (Name::Second,                 SECOND_UNIT,                   f64::SECOND,                  "Second"),
    (Name::Minute,                 MINUTE_UNIT,                   f64::MINUTE,                  "Minute"),
    (Name::Hour,                   HOUR_UNIT,                     f64::HOUR,                    "Hour"),
    (Name::Day,                    DAY_UNIT,                      f64::DAY,                     "Day"),
    (Name::Week,                   WEEK_UNIT,                     f64::WEEK,                    "Week"),
    (Name::Meter,                  METER_UNIT,                    f64::METER,                   "Meter"),
    (Name::Inch,                   INCH_UNIT,                     f64::INCH,                    "Inch"),
    (Name::Foot,                   FOOT_UNIT,                     f64::FOOT,                    "Foot"),
    (Name::Yard,                   YARD_UNIT,                     f64::YARD,                    "Yard"),
    (Name::Mile,                   MILE_UNIT,                     f64::MILE,                    "Mile"),
    (Name::NauticalMile,           NAUTICAL_MILE_UNIT,            f64::NAUTICAL_MILE,           "Nautical mile"),
    (Name::Fathom,                 FATHOM_UNIT,                   f64::FATHOM,                  "Fathom"),
    (Name::Mil,                    MIL_UNIT,                      f64::MIL,                     "Mil"),
    (Name::Point,                  POINT_UNIT,                    f64::POINT,                   "Point"),
    (Name::Textpoint,              TEXPOINT_UNIT,                 f64::TEXPOINT,                "Textpoint"),
    (Name::Micron,                 MICRON_UNIT,                   f64::MICRON,                  "Micron"),
    (Name::Angstrom,               ANGSTROM_UNIT,                 f64::ANGSTROM,                "Angstrom"),
    (Name::Hectare,                HECTARE_UNIT,                  f64::HECTARE,                 "Hectare"),
    (Name::Acre,                   ACRE_UNIT,                     f64::ACRE,                    "Acre"),
    (Name::Barn,                   BARN_UNIT,                     f64::BARN,                    "Barn"),
    (Name::Liter,                  LITER_UNIT,                    f64::LITER,                   "Liter"),
    (Name::UsGallon,               US_GALLON_UNIT,                f64::US_GALLON,               "US gallon"),
    (Name::Quart,                  QUART_UNIT,                    f64::QUART,                   "Quart"),
    (Name::Pint,                   PINT_UNIT,                     f64::PINT,                    "Pint"),
    (Name::Cup,                    CUP_UNIT,                      f64::CUP,                     "Cup"),
    (Name::FluidOunce,             FLUID_OUNCE_UNIT,              f64::FLUID_OUNCE,             "Fluid ounce"),
    (Name::Tablespoon,             TABLESPOON_UNIT,               f64::TABLESPOON,              "Tablespoon"),
    (Name::Teaspoon,               TEASPOON_UNIT,                 f64::TEASPOON,                "Teaspoon"),
    (Name::CanadianGallon,         CANADIAN_GALLON_UNIT,          f64::CANADIAN_GALLON,         "Canadian gallon"),
    (Name::UkGallon,               UK_GALLON_UNIT,                f64::UK_GALLON,               "UK gallon"),
    (Name::MilesPerHour,           MILES_PER_HOUR_UNIT,           f64::MILES_PER_HOUR,          "Miles per hour"),
    (Name::KilometersPerHour,      KILOMETERS_PER_HOUR_UNIT,      f64::KILOMETERS_PER_HOUR,     "Kilometers per hour"),
    (Name::Knot,                   KNOT_UNIT,                     f64::KNOT,                    "Knot"),
    (Name::Kilogram,               KILOGRAM_UNIT,                 f64::KILOGRAM,                "Kilogram"),
    (Name::PoundMass,              POUND_MASS_UNIT,               f64::POUND_MASS,              "Pound mass"),
    (Name::OunceMass,              OUNCE_MASS_UNIT,               f64::OUNCE_MASS,              "Ounce mass"),
    (Name::Ton,                    TON_UNIT,                      f64::TON,                     "Ton"),
    (Name::MetricTon,              METRIC_TON_UNIT,               f64::METRIC_TON,              "Metric ton"),
    (Name::UkTon,                  UK_TON_UNIT,                   f64::UK_TON,                  "UK ton"),
    (Name::TroyOunce,              TROY_OUNCE_UNIT,               f64::TROY_OUNCE,              "Troy ounce"),
    (Name::Carat,                  CARAT_UNIT,                    f64::CARAT,                   "Carat"),
    (Name::UnifiedAtomicMass,      UNIFIED_ATOMIC_MASS_UNIT,      f64::UNIFIED_ATOMIC_MASS,     "Unified atomic mass"),
    (Name::GramForce,              GRAM_FORCE_UNIT,               f64::GRAM_FORCE,              "Gram force"),
    (Name::PoundForce,             POUND_FORCE_UNIT,              f64::POUND_FORCE,             "Pound force"),
    (Name::KilopoundForce,         KILOPOUND_FORCE_UNIT,          f64::KILOPOUND_FORCE,         "Kilopound force"),
    (Name::Poundal,                POUNDAL_UNIT,                  f64::POUNDAL,                 "Poundal"),
    (Name::Calorie,                CALORIE_UNIT,                  f64::CALORIE,                 "Calorie"),
    (Name::Btu,                    BTU_UNIT,                      f64::BTU,                     "Btu"),
    (Name::Therm,                  THERM_UNIT,                    f64::THERM,                   "Therm"),
    (Name::Horsepower,             HORSEPOWER_UNIT,               f64::HORSEPOWER,              "Horsepower"),
    (Name::Bar,                    BAR_UNIT,                      f64::BAR,                     "Bar"),
    (Name::StdAtmosphere,          STD_ATMOSPHERE_UNIT,           f64::STD_ATMOSPHERE,          "STD atmosphere"),
    (Name::Torr,                   TORR_UNIT,                     f64::TORR,                    "Torr"),
    (Name::MeterOfMercury,         METER_OF_MERCURY_UNIT,         f64::METER_OF_MERCURY,        "Meter of mercury"),
    (Name::InchOfMercury,          INCH_OF_MERCURY_UNIT,          f64::INCH_OF_MERCURY,         "Inch of mercury"),
    (Name::InchOfWater,            INCH_OF_WATER_UNIT,            f64::INCH_OF_WATER,           "Inch of water"),
    (Name::Psi,                    PSI_UNIT,                      f64::PSI,                     "Psi"),
    (Name::Poise,                  POISE_UNIT,                    f64::POISE,                   "Poise"),
    (Name::Stokes,                 STOKES_UNIT,                   f64::STOKES,                  "Stokes"),
    (Name::Stilb,                  STILB_UNIT,                    f64::STILB,                   "Stilb"),
    (Name::Lumen,                  LUMEN_UNIT,                    f64::LUMEN,                   "Lumen"),
    (Name::Lux,                    LUX_UNIT,                      f64::LUX,                     "Lux"),
    (Name::Phot,                   PHOT_UNIT,                     f64::PHOT,                    "Phot"),
    (Name::Footcandle,             FOOTCANDLE_UNIT,               f64::FOOTCANDLE,              "Footcandle"),
    (Name::Lambert,                LAMBERT_UNIT,                  f64::LAMBERT,                 "Lambert"),
    (Name::Footlambert,            FOOTLAMBERT_UNIT,              f64::FOOTLAMBERT,             "Footlambert"),
    (Name::Curie,                  CURIE_UNIT,                    f64::CURIE,                   "Curie"),
    (Name::Roentgen,               ROENTGEN_UNIT,                 f64::ROENTGEN,                "Roentgen"),
    (Name::Rad,                    RAD_UNIT,                      f64::RAD,                     "Rad"),
    (Name::SolarMass,              SOLAR_MASS_UNIT,               f64::SOLAR_MASS,              "Solar mass"),
    (Name::BohrRadius,             BOHR_RADIUS_UNIT,              f64::BOHR_RADIUS,             "Bohr radius"),
    (Name::Newton,                 NEWTON_UNIT,                   f64::NEWTON,                  "Newton"),
    (Name::Dyne,                   DYNE_UNIT,                     f64::DYNE,                    "Dyne"),
    (Name::Joule,                  JOULE_UNIT,                    f64::JOULE,                   "Joule"),
    (Name::Erg,                    ERG_UNIT,                      f64::ERG,                     "Erg"),
    (Name::StefanBolzmannConstant, STEFAN_BOLTZMANN_CONSTANT_UNIT,f64::STEFAN_BOLTZMANN_CONSTANT,"STEFAN_BOLTZMANN_CONSTANT"),
    (Name::ThomsonCrossSection,    THOMSON_CROSS_SECTION_UNIT,    f64::THOMSON_CROSS_SECTION,   "THOMSON_CROSS_SECTION"),
    (Name::BohrMagneton,           BOHR_MAGNETON_UNIT,            f64::BOHR_MAGNETON,           "Bohr magneton"),
    (Name::NuclearMagneton,        NUCLEAR_MAGNETON_UNIT,         f64::NUCLEAR_MAGNETON,        "Nuclear magneton"),
    (Name::ElectronMagneticMoment, ELECTRON_MAGNETIC_MOMENT_UNIT, f64::ELECTRON_MAGNETIC_MOMENT,"Electron magnetic moment"),
    (Name::ProtonMagneticMoment,   PROTON_MAGNETIC_MOMENT_UNIT,   f64::PROTON_MAGNETIC_MOMENT,  "Proton magnetic moment"),
    (Name::Faraday,                FARADAY_UNIT,                  f64::FARADAY,                 "Faraday"),
    (Name::ElectronCharge,         ELECTRON_CHARGE_UNIT,          f64::ELECTRON_CHARGE,         "Electron charge"),
    (Name::VacuumPermittivity,     VACUUM_PERMITTIVITY_UNIT,      f64::VACUUM_PERMITTIVITY,     "VACUUM_PERMITTIVITY"),
    (Name::VacuumPermeability,     VACUUM_PERMEABILITY_UNIT,      f64::VACUUM_PERMITTIVITY,     "VACUUM_PERMITTIVITY"),
    (Name::Debye,                  DEBYE_UNIT,                    f64::DEBYE,                   "Debye"),
    (Name::Gauss,                  GAUSS_UNIT,                    f64::GAUSS,                   "Gauss"),
    ];


