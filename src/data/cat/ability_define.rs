#[macro_export]
/// Generate the enum for game abilities.
macro_rules! generate_ability_enum {
    (
        $(#[doc = $enum_doc:expr])?
        name = $enum_name:ident,

        $(
            $(#[doc = $variant_doc:expr])?
            {
                name = $variant:ident $({$(
                    $(#[doc = $field_doc:expr])?
                    $field_name:ident : $field_type:ty
                ),* $(,)?})?,
                config = $config:expr
            }
        ),* $(,)?
    ) => {

        #[derive(Debug)]
        $(#[doc = $enum_doc])?
        pub enum $enum_name {
            $(
                $(#[doc = $variant_doc])?
                $variant $({$(
                    $(#[doc = $field_doc])?
                    $field_name: $field_type
                ),*})?,
            )*
        }

        impl $enum_name {
            /// Does the ability apply on every hit regardless?
            pub fn is_general(&self)  -> bool { match self { $( Self::$variant { .. } => $config.is_general, )* } }
            /// Is the ability removed by curse?
            pub fn is_cursable(&self) -> bool { match self { $( Self::$variant { .. } => $config.is_cursable, )* } }
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
