//! Defines enemy object.

use crate::game_data::{
    enemy::{parsed::stats::EnemyStats, raw::stats::TUnitContainer},
    unit::anim::{Anim, AnimDataError, get_maanim_data},
    version::lang::{MultiLangVersionContainer, VersionLanguage},
};

#[derive(Debug)]
/// Parsed enemy object.
pub struct Enemy {
    /// ERO id.
    pub id: u32,
    /// Enemy stats.
    pub stats: EnemyStats,
    /// Animation data.
    pub anims: Anim,
}

#[derive(Debug)]
/// Error when getting enemy data.
pub enum EnemyDataError {
    /// Unit's stats not found in `t_unit.csv`.
    StatsNotFound,
    /// Error with unit animations.
    AnimationError {
        /// Exact error that occurred.
        source: AnimDataError,
    },
}

impl Enemy {
    /// Get enemy from wiki id.
    pub fn from_wiki_id<T: MultiLangVersionContainer>(
        wiki_id: u32,
        version_cont: &T,
    ) -> Result<Self, EnemyDataError> {
        let id = wiki_id;
        let stats = Self::get_stats(id, version_cont)?;
        let anims = Self::get_anims(id, version_cont)?;

        Ok(Self { id, stats, anims })
    }

    fn get_anims<T: MultiLangVersionContainer>(
        wiki_id: u32,
        version_cont: &T,
    ) -> Result<Anim, EnemyDataError> {
        let animfile = format!("{wiki_id:03}_e.maanim");
        let get = |ver| get_maanim_data(&animfile, ver);

        let anims = get(version_cont.get_lang(VersionLanguage::EN))
            // .or_else(|_| get(version_cont.get_lang(VersionLanguage::KR)))
            // .or_else(|_| get(version_cont.get_lang(VersionLanguage::TW)))
            .or_else(|_| get(version_cont.get_lang(VersionLanguage::JP)))
            .or_else(|_| get(version_cont.get_lang(VersionLanguage::Fallback)));

        let anims = match anims {
            Ok(a) => a,
            Err(source) => {
                return Err(EnemyDataError::AnimationError { source });
            }
        };

        Ok(anims)
    }

    fn get_stats<T: MultiLangVersionContainer>(
        wiki_id: u32,
        version_cont: &T,
    ) -> Result<EnemyStats, EnemyDataError> {
        let t_unit = version_cont
            .lang_default()
            .get_cached_file::<TUnitContainer>();
        let stats = t_unit
            .get_unit(wiki_id)
            .ok_or(EnemyDataError::StatsNotFound)?;

        Ok(EnemyStats::from_raw(stats))
    }
}
