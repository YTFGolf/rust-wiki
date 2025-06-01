use crate::{
    game_data::cat::{
        ability::Ability,
        parsed::stats::form::{AttackHits, CatFormStats, EnemyType},
    },
    interface::error_handler::InfallibleWrite,
};
use std::fmt::Write;

fn get_targets(targets: &[EnemyType]) -> String {
    let len = targets.len();
    if len == 0 {
        return String::new();
    }
    if len == 1 {
        let target = &targets[0];
        return format!("[[:Category:{target} Enemies|{target}]]");
    }

    let mut buf = String::new();
    let mut iter = targets.iter().peekable();
    let first = iter.next().expect("already checked len >= 2");
    write!(buf, "[[:Category:{first} Enemies|{first}]]").infallible_write();

    while let Some(target) = iter.next() {
        let separator = match iter.peek() {
            Some(_) => ",",
            None => " and",
        };
        write!(buf, "{separator} [[:Category:{target} Enemies|{target}]]").infallible_write();
    }

    buf
}

/// Get display for abilities when a cat has multiple hits.
/// ```
/// # use rust_wiki::game_data::cat::parsed::stats::form::{AttackHit, AttackHits};
/// # use rust_wiki::interface::scripts::cat_info::abilities::get_multiple_hit_abilities;
/// let normal = AttackHits::Single([AttackHit { active_ability: true, ..Default::default() }]);
/// let multab = get_multiple_hit_abilities;
/// assert_eq!(multab(normal), "");
/// ```
fn get_multiple_hit_abilities(hits: &AttackHits) -> &'static str {
    match hits {
        AttackHits::Single([hit1]) => match hit1.active_ability {
            true => "",
            false => unreachable!(),
        },
        AttackHits::Double([hit1, hit2]) => todo!(),
        AttackHits::Triple([hit1, hit2, hit3]) => todo!(),
    }
}

fn get_ability(link: &str, display: &str) -> String {
    format!("[[Special Abilities#{link}|{display}]]")
}

/// DOES NOT DO MULTIHIT
pub fn get_abilities(stats: &CatFormStats) -> String {
    let mut abilities = vec![];
    // start: multihit, ld, omni
    let mut immunities = vec![];

    let targets = get_targets(&stats.targets);
    let multab = get_multiple_hit_abilities(&stats.attack.hits);

    let abil = get_ability;

    for ability in &stats.abilities {
        match ability {
            Ability::StrongAgainst => abilities.push(format!(
                "{strong} against {targets} enemies (Deals 1.5x damage, only takes 1/2 damage)",
                strong = abil("Strong Against", "Strong")
            )),
            Ability::Knockback { chance } => abilities.push(format!(
                "{chance}% chance to {knockback} {targets} enemies{multab}",
                knockback = abil("Knockback", "knockback")
            )),
            Ability::Weaken {
                chance,
                duration,
                multiplier,
            } => todo!(),
            Ability::Strengthen { hp, multiplier } => todo!(),
            Ability::Survives { chance } => todo!(),
            Ability::Metal => todo!(),

            Ability::WaveBlocker => todo!(),

            Ability::ZombieKiller => todo!(),
            Ability::WitchKiller => todo!(),

            Ability::Kamikaze => todo!(),
            Ability::BarrierBreaker { chance } => todo!(),

            Ability::EvaAngelKiller => todo!(),

            Ability::InsaneResist => todo!(),
            Ability::InsaneDamage => todo!(),
            Ability::SavageBlow { chance, damage } => todo!(),
            Ability::Dodge { chance, duration } => todo!(),
            Ability::Surge(surge) => todo!(),

            Ability::Curse { chance, duration } => todo!(),
            Ability::ShieldPierce { chance } => todo!(),
            Ability::ColossusSlayer => todo!(),
            Ability::Soulstrike => todo!(),
            Ability::BehemothSlayer {
                dodge_chance,
                dodge_duration,
            } => todo!(),
            Ability::CounterSurge => todo!(),
            Ability::ConjureUnit { id } => todo!(),
            Ability::SageSlayer => todo!(),
            Ability::MetalKiller { damage } => todo!(),
            Ability::Explosion { chance, spawn_quad } => todo!(),

            Ability::ImmuneToWave => immunities.push("Waves"),
            Ability::ImmuneToKB => immunities.push("Knockback"),
            Ability::ImmuneToFreeze => immunities.push("Freeze"),
            Ability::ImmuneToSlow => immunities.push("Slow"),
            Ability::ImmuneToWeaken => immunities.push("Weaken"),
            Ability::ImmuneToBossShockwave => immunities.push("Boss Shockwave"),
            Ability::ImmuneToWarp => immunities.push("Warp"),
            Ability::ImmuneToCurse => immunities.push("Curse"),
            Ability::ImmuneToToxic => immunities.push("Toxic"),
            Ability::ImmuneToSurge => immunities.push("Surge"),
            Ability::ImmuneToExplosion => immunities.push("Explosions"),
        }
    }

    if !immunities.is_empty() {
        abilities.push(String::from("Some immunities"));
    }

    abilities.join("<br>\n")
}
