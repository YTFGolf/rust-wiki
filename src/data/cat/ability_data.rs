use crate::generate_ability_enum;

struct Config {
    is_general: bool,
    is_cursable: bool,
}

generate_ability_enum!(
    #[doc = "Cat or enemy ability."]
    name = Ability,

    #[doc = "Freeze the enemy."]
    {
        name = Freeze {
            #[doc = "Chance to freeze the enemy."]
            chance: u8,
            #[doc = "Duration of freeze in frames."]
            duration: u32,
        },
        config = Config {
            is_general: false,
            is_cursable: true,
        }
    },

    #[doc = "Double money collected when defeating the enemy."]
    { name = DoubleBounty, config = Config { is_general: true, is_cursable: false } }
);

// Check Donut's `multab`.
// I'm not sure this is a good idea tbh.

impl Ability {
    /// Does the ability have targets when used on a cat. This is equivalent to
    /// [`Ability::is_cursable`].
    pub fn has_targets(&self) -> bool {
        self.is_cursable()
    }
}
