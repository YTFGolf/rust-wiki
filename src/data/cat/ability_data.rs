use crate::generate_config;

generate_config!(
    { name = Freeze { chance: u8, duration: u32, }, is_general = false, is_cursable = true },
);
