#[macro_export]
/// Generate the enum for game abilities.
macro_rules! generate_config {
    (
        $({
            name = $variant:ident $( { $($field_name:ident : $field_type:ty),*$(,)? } )?,
            is_general  = $is_general:expr,
            is_cursable = $is_cursable:expr
        }),* $(,)?
    ) => {
        // Define the enum
        #[derive(Debug)]
        pub enum Ability {
            $(
                $variant $( { $($field_name: $field_type),* } )?,
            )*
        }

        impl Ability {
            pub fn is_general(&self) -> bool {
                match self {
                    $(
                        Self::$variant { .. } => $is_general,
                    )*
                }
            }
        }
    };
}

// #[derive(Debug)]
// pub enum Ability {}
// will need to be done better. Trait or macro I think. Check Donut's `multab`.
/*
/// Is it removed with curse.
is_cursable
/// Does this occur on every hit.
is_general
*/
