//! The variant (e.g. SoL, main chapters etc.) of the stage.

use strum::{EnumIter, FromRepr};

const _: () = assert!(std::mem::size_of::<StageVariantID>() == std::mem::size_of::<VariantSize>());

/// Size of variant.
pub type VariantSize = u32;

#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, FromRepr, EnumIter, PartialEq)]
/// The variant (e.g. SoL, main chapters etc.) of the stage.
pub enum StageVariantID {
    // TrueFormUnlocks = 28,
    // TapjoyPopups = 29,
    // cargo fmt works weirdly with comments so leaving these 2 up here
    /// Technically also used for login stamps from maps 900-999.
    SoL = 0,
    Event = 1,
    Collab = 2,
    /// Maps 0-2 are EoC, 3-5 are ItF, 6-8 are CotC.
    MainChapters = 3,
    /// Continuation and one-time invasion stages.
    Extra = 4,
    // Gamatoto = 5,
    Dojo = 6,
    Tower = 7,
    // WeeklyMissions = 8,
    // EventMissions = 9,
    // Unknown1 = 10,
    RankingDojo = 11,
    Challenge = 12,

    UL = 13,
    Catamin = 14,
    // PermanentMissions = 15,
    // LegendQuest = 16,
    // MonthlyMissions = 17,
    // WildcatSlots = 18,
    // TalkingCat = 19, // ?
    EocOutbreak = 20,
    ItfOutbreak = 21,
    CotcOutbreak = 22,
    Filibuster = 23,
    Gauntlet = 24,
    Enigma = 25,
    // SpecialEnigmaSelection = 26, // ?
    CollabGauntlet = 27,

    AkuRealms = 30,
    Behemoth = 31,
    // Unknown2 = 32,
    Labyrinth = 33,
    ZL = 34,
    // LoginStamps2 = 35,
    Colosseum = 36,
    Championships = 37,
    FilibusterOutbreak = 38,
}

impl From<VariantSize> for StageVariantID {
    fn from(value: VariantSize) -> Self {
        Self::from_repr(value).unwrap_or_else(|| panic!("Unexpected stage number: {value}!"))
    }
}

// Simple methods on self.
impl StageVariantID {
    /// Get variant number.
    pub const fn num(&self) -> VariantSize {
        *self as VariantSize
    }

    /// Is variant a main chapter?
    pub const fn is_main(&self) -> bool {
        matches!(
            self,
            Self::MainChapters | Self::Filibuster | Self::AkuRealms | Self::FilibusterOutbreak
        ) || self.is_outbreak()
    }

    /// Is variant a Zombie Outbreak?
    pub const fn is_outbreak(&self) -> bool {
        matches!(
            self,
            Self::EocOutbreak | Self::ItfOutbreak | Self::CotcOutbreak | Self::FilibusterOutbreak
        )
    }

    /// Is variant a Legend Stage?
    pub const fn is_legend_stage(&self) -> bool {
        matches!(self, Self::SoL | Self::UL | Self::ZL)
    }

    /// Is variant a collab?
    pub const fn is_collab(&self) -> bool {
        matches!(self, Self::Collab | Self::CollabGauntlet)
    }

    /// Is variant a gauntlet?
    pub const fn is_gauntlet(&self) -> bool {
        matches!(self, Self::Gauntlet | Self::CollabGauntlet)
    }

    /// Does variant only have a single stage?
    pub const fn has_single_stage(&self) -> bool {
        matches!(
            self,
            Self::Challenge | Self::Filibuster | Self::FilibusterOutbreak
        )
    }

    /// Does variant only have a single map but multiple stages?
    pub const fn has_single_map(&self) -> bool {
        matches!(self, Self::AkuRealms | Self::Labyrinth)
    }

    // no dojo because championships are ambiguous
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    /// Make sure that all variants are converted properly.
    fn test_variants() {
        for variant in StageVariantID::iter() {
            // println!("{variant:?}, {}", variant.num());
            assert_eq!(variant, variant.num().into());
        }
        // panic!()
    }
}
