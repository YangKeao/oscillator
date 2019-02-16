use phf::{phf_map, phf_set};

pub static MOD_MAP: phf::Map<u32, &'static str> = phf_map! {
    1u32 => "s",
    2u32 => "l",
    4u32 => "c",
    8u32 => "m1",
    16u32 => "m2",
    32u32 => "m3",
    64u32 => "m4",
    128u32 => "m5",
};
