use crate::generate_config;

generate_config!(
    #[doc = "hi"]
    { name = Freeze {
        #[doc = "hi"]
        chance: u8,
        #[doc = "hi"]
        duration: u32,
    }, is_general = false, is_cursable = true },

);
