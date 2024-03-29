pub const INCH: f32 = 25.4;
pub const PPI: f32 = 72.0;
pub const DPI_F: f32 = 5.;
pub const DPI: f32 = 72.0 * DPI_F;
pub const SPATIUM20: f32 = 5.0 * (DPI / 72.0);
pub const DPMM: f32 = DPI / INCH;

/// Quarter Note duration
pub const DIVISION: i32 = 480;

pub const VOICES: usize = 4;

pub const DEFAULT_DPI: f32 = 96.;

pub const PI2: f32 = std::f32::consts::PI * 2.;
pub const PI_HALF: f32 = std::f32::consts::PI / 2.;