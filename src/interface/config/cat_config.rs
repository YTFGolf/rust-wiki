//! Deals with the config for cat info.

use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use strum::EnumIter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
/// Which version of the stats template to use.
pub enum StatsTemplateVersion {
    #[default]
    /// Latest version.
    Current,
    /// Manual template.
    Manual,
    /// Version 0.1.
    Ver0o1,
    /// Version 0.2.
    Ver0o2,
    /// Version 1.0.
    Ver1o0,
    /// Version 1.1.
    Ver1o1,
}
const POSSIBLE_VALUES: [StatsTemplateVersion; 6] = [
    StatsTemplateVersion::Current,
    StatsTemplateVersion::Manual,
    StatsTemplateVersion::Ver0o1,
    StatsTemplateVersion::Ver0o2,
    StatsTemplateVersion::Ver1o0,
    StatsTemplateVersion::Ver1o1,
];
impl StatsTemplateVersion {
    /// Get string representation of template version.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Manual => "manual",
            Self::Ver0o1 => "0.1",
            Self::Ver0o2 => "0.2",
            Self::Ver1o0 => "1.0",
            Self::Ver1o1 => "1.1",
        }
    }
}
impl Display for StatsTemplateVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl FromStr for StatsTemplateVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "current" => Ok(Self::Current),
            "manual" => Ok(Self::Manual),
            "0.1" => Ok(Self::Ver0o1),
            "0.2" => Ok(Self::Ver0o2),
            "1.0" => Ok(Self::Ver1o0),
            "1.1" => Ok(Self::Ver1o1),
            _ => Err(()),
        }
    }
}

mod do_extra_stuff {
    use super::*;
    impl Serialize for StatsTemplateVersion {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> Deserialize<'de> for StatsTemplateVersion {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct VisitorThingy;

            impl<'de> serde::de::Visitor<'de> for VisitorThingy {
                type Value = StatsTemplateVersion;

                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("a valid stats template version")
                }

                fn visit_str<E>(self, v: &str) -> Result<StatsTemplateVersion, E>
                where
                    E: serde::de::Error,
                {
                    const FIELDS: &[&str] = &[];
                    StatsTemplateVersion::from_str(v)
                        .map_err(|_| serde::de::Error::unknown_field(v, FIELDS))
                }
            }

            deserializer.deserialize_str(VisitorThingy)
        }
    }

    impl clap::ValueEnum for StatsTemplateVersion {
        fn value_variants<'a>() -> &'a [Self] {
            &POSSIBLE_VALUES
        }

        fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
            Some(clap::builder::PossibleValue::new(self.as_str()))
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
/// Config for cat info.
pub struct CatConfig {
    /// Which version of stats template to use.
    pub stats_template_version: StatsTemplateVersion,

    /// Do you hide stats validation.
    pub stats_hide_validation: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn as_str_is_from_str() {
        for value in POSSIBLE_VALUES {
            let as_str = value.as_str();
            let parsed_value = as_str.parse().unwrap();
            assert_eq!(value, parsed_value);
            assert_eq!(as_str, parsed_value.as_str());
        }
    }

    #[test]
    fn possible_values_is_correct() {
        let collected = StatsTemplateVersion::iter().collect::<Vec<_>>();
        assert_eq!(&collected, &POSSIBLE_VALUES);
    }
}
