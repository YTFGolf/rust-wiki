use super::{ability::Ability, raw::CombinedCatData};
use std::rc::Rc;

#[derive(Debug)]
pub enum EnemyType {
    Red,
    Float,
    Black,
    Metal,
    Traitless,
    Angel,
    Alien,
    Zombie,
    Relic,
    Aku,
}
impl EnemyType {
    pub fn get_all_targets(combined: &CombinedCatData) -> Vec<EnemyType> {
        let (fixed, variable) = combined;
        let mut targets = vec![];

        if bool(fixed.targ_red).unwrap() {
            targets.push(Self::Red)
        }
        if bool(fixed.targ_float).unwrap() {
            targets.push(Self::Float)
        }
        if bool(fixed.targ_black).unwrap() {
            targets.push(Self::Black)
        }
        if bool(fixed.targ_metal).unwrap() {
            targets.push(Self::Metal)
        }
        if bool(fixed.targ_traitless).unwrap() {
            targets.push(Self::Traitless)
        }
        if bool(fixed.targ_angel).unwrap() {
            targets.push(Self::Angel)
        }
        if bool(fixed.targ_alien).unwrap() {
            targets.push(Self::Alien)
        }
        if bool(fixed.targ_zombie).unwrap() {
            targets.push(Self::Zombie)
        }
        if bool(variable.targ_relic).unwrap() {
            targets.push(Self::Relic)
        }
        if bool(variable.targ_aku).unwrap() {
            targets.push(Self::Aku)
        }

        targets
    }
}
fn bool(value: u8) -> Result<bool, String> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        x => Err(format!("{x} is not a valid boolean number!")),
    }
}

#[derive(Debug)]
pub enum AttackRange {
    Normal,
    LD { base: i16, distance: i16 },
    Omni { base: i16, distance: i16 },
}
impl AttackRange {
    fn new(base: i16, distance: i16) -> Self {
        if base == 0 {
            AttackRange::Normal
        } else if distance > 0 {
            AttackRange::LD {
                base,
                distance: base + distance,
            }
        } else {
            AttackRange::Omni {
                base,
                distance: base + distance,
            }
        }
    }
}

#[derive(Debug)]
pub struct AttackHit {
    active_ability: bool,
    damage: u32,
    range: AttackRange,
    foreswing: u16,
}

#[derive(Debug)]
pub enum AttackHits {
    Single(AttackHit),
    Double([AttackHit; 2]),
    Triple([AttackHit; 3]),
}
impl AttackHits {
    fn from_combined(combined: &CombinedCatData) -> AttackHits {
        let (_, var) = combined;
        if var.mhit_atk2 == 0 {
            Self::Single(Self::single(combined))
        } else if var.mhit_atk3 == 0 {
            Self::Double([Self::get_hit1(combined), Self::get_hit2(combined)])
        } else {
            Self::Triple([
                Self::get_hit1(combined),
                Self::get_hit2(combined),
                Self::get_hit3(combined),
            ])
        }
    }

    /// Only one attack hit.
    fn single(combined: &CombinedCatData) -> AttackHit {
        let (fixed, _) = combined;
        let active_ability = true;
        // assumption that it doesn't really matter here, might do some logging
        let damage = fixed.atk;
        let range = AttackRange::new(fixed.ld_base, fixed.ld_range);
        let foreswing = fixed.foreswing;
        AttackHit {
            active_ability,
            damage,
            range,
            foreswing,
        }
    }

    /// Get the first attack hit. This is almost exactly the same as
    /// [`Self::single`], but it also takes into account the `proc_on_hit1`
    /// flag.
    fn get_hit1(combined: &CombinedCatData) -> AttackHit {
        let (fixed, variable) = combined;
        let active_ability = bool(variable.proc_on_hit1).unwrap();
        let damage = fixed.atk;
        let range = AttackRange::new(fixed.ld_base, fixed.ld_range);
        let foreswing = fixed.foreswing;
        AttackHit {
            active_ability,
            damage,
            range,
            foreswing,
        }
    }

    fn get_hit2(combined: &CombinedCatData) -> AttackHit {
        let (_, variable) = combined;
        let active_ability = bool(variable.proc_on_hit2).unwrap();
        let damage = variable.mhit_atk2;
        let range = AttackRange::new(variable.second_ld_base, variable.second_ld_range);
        let foreswing = variable.mhit_atk2_fswing;
        AttackHit {
            active_ability,
            damage,
            range,
            foreswing,
        }
    }

    fn get_hit3(combined: &CombinedCatData) -> AttackHit {
        let (_, variable) = combined;
        let active_ability = bool(variable.proc_on_hit3).unwrap();
        let damage = variable.mhit_atk3;
        let range = AttackRange::new(variable.third_ld_base, variable.third_ld_range);
        let foreswing = variable.mhit_atk3_fswing;
        AttackHit {
            active_ability,
            damage,
            range,
            foreswing,
        }
    }
}

#[derive(Debug)]
pub enum AreaOfEffect {
    SingleAttack,
    AreaAttack,
}

#[derive(Debug)]
pub struct Attack {
    hits: AttackHits,
    aoe: AreaOfEffect,
    standing_range: u16,
    tba: u16,
    // this is an interval, so cycle is foreswing + max(backswing, 2 * tba - 1)
    // backswing is not a stat, it is the length of the unit's animation
}
impl Attack {
    fn from_combined(combined: &CombinedCatData) -> Self {
        let (fixed, _) = combined;
        let aoe = match fixed.is_area {
            0 => AreaOfEffect::SingleAttack,
            1 => AreaOfEffect::AreaAttack,
            _ => unreachable!(),
        };
        Self {
            hits: AttackHits::from_combined(combined),
            aoe,
            standing_range: fixed.range,
            tba: fixed.tba,
        }
    }
}

#[derive(Debug)]
/// Stats at level 1 with no treasures.
pub struct CatStats {
    /// Unit HP.
    pub hp: u32,
    /// HP knockbacks.
    pub kb: u16,
    /// Death soul animation, more testing needs to be done.
    pub death_anim: i8,
    /// Speed (distance travelled every frame).
    pub speed: u8,
    /// EoC1 cost.
    pub price: u16,
    /// Respawn frames / 2.
    pub respawn_half: u16,
    /// Unit attack.
    pub attack: Attack,
    /// All unit's abilities.
    pub abilities: Rc<[Ability]>,
    /// Enemy types the unit targets.
    pub targets: Rc<[EnemyType]>,
}

impl CatStats {
    /// Get unit stats from the combined stat data.
    pub fn from_combined(combined: &CombinedCatData) -> Self {
        let (fixed, var) = combined;
        Self {
            hp: fixed.hp,
            kb: fixed.kb,
            death_anim: var.death,
            speed: fixed.speed,
            price: fixed.price,
            respawn_half: fixed.respawn,
            attack: Attack::from_combined(combined),
            abilities: Ability::get_all_abilities(combined).into(),
            targets: EnemyType::get_all_targets(combined).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::TEST_CONFIG, data::cat::raw::read_data_file};

    #[test]
    fn tmp() {
        #[allow(unused_variables)]
        let cond = true;
        let cond = false;
        if cond {
            return;
        }
        let file_name = "unit136.csv";
        let version = TEST_CONFIG.version.current_version();
        panic!(
            "{:#?}",
            read_data_file(file_name, version)
                .map(|comb| CatStats::from_combined(&comb))
                .collect::<Vec<_>>()
        )
    }
}
