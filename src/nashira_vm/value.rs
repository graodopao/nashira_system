pub const FRAC_BITS: i32 = 8;
pub const SCALE: i32 = 1 << FRAC_BITS;
pub type Fixed32 = i32;

pub enum VALUE {
    NUM(i32),
    BOOL(bool),
}

pub fn fixed_from_int(value: i32) -> Fixed32 {
    value << FRAC_BITS
}

pub fn fixed_from_float(value: f32) -> Fixed32 {
    (value * SCALE as f32).round() as Fixed32
}

pub fn int_from_fixed(value: Fixed32) -> i32 {
    value >> FRAC_BITS
}

pub fn float_from_fixed(value: Fixed32) -> f32 {
    value as f32 / SCALE as f32
}