use phf::{phf_map, phf_set};

pub static MOD_MAP: phf::Map<u32, &'static str> = phf_map! {
    1u32 => "S",
    2u32 => "L",
    4u32 => "C",
    8u32 => "M1",
    16u32 => "M2",
    32u32 => "M3",
    64u32 => "M4",
    128u32 => "M5",
};
