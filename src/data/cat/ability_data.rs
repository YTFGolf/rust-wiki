use crate::generate_config;

struct Config {
    is_general: bool,
    is_cursable: bool,
}

generate_config!(
    #[doc = "hi"]
    {
        name = Freeze {
            #[doc = "hi"]
            chance: u8,
            #[doc = "hi"]
            duration: u32,
        }, config = Config { is_general : false, is_cursable : true }
    },
    #[doc = "nothing"]
    { name = DoubleBounty, config = Config {is_general: true, is_cursable: false}}
);
