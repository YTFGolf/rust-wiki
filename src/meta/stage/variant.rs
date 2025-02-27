//! The variant (e.g. SoL, main chapters etc.) of the stage.

const _: () = assert!(std::mem::size_of::<StageVariant>() == std::mem::size_of::<VariantSize>());

/// Size of variant.
type VariantSize = u32;

#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
/// The variant (e.g. SoL, main chapters etc.) of the stage.
pub enum StageVariant {
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
}

// Simple methods on self.
impl StageVariant {
    /// Get variant number.
    pub const fn num(&self) -> VariantSize {
        *self as VariantSize
    }

    /// Is variant a main chapter?
    pub fn is_main(&self) -> bool {
        matches!(
            self,
            Self::MainChapters | Self::Filibuster | Self::AkuRealms
        ) || self.is_outbreak()
    }

    /// Is variant a Zombie Outbreak?
    pub fn is_outbreak(&self) -> bool {
        matches!(
            self,
            Self::EocOutbreak | Self::ItfOutbreak | Self::CotcOutbreak
        )
    }

    /// Is variant a Legend Stage?
    pub fn is_legend_stage(&self) -> bool {
        matches!(self, Self::SoL | Self::UL | Self::ZL)
    }

    /// Is variant a collab?
    pub fn is_collab(&self) -> bool {
        matches!(self, Self::Collab | Self::CollabGauntlet)
    }

    /// Is variant a gauntlet?
    pub fn is_gauntlet(&self) -> bool {
        matches!(self, Self::Gauntlet | Self::CollabGauntlet)
    }

    // no dojo because championships are ambiguous
}

// Conversions.
impl StageVariant {}
