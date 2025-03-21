//! Defines a procedural macro for [`super::ability_data`].

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
            pub const fn is_general(&self)  -> bool { match self { $( Self::$variant { .. } => {const ANS: bool = $config.is_general; ANS} )* } }
            /// Is the ability removed by curse?
            pub const fn is_cursable(&self) -> bool { match self { $( Self::$variant { .. } => {const ANS: bool = $config.is_cursable; ANS} )* } }
        }
    };
}
