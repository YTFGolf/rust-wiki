use crate::{
    game_data::cat::{
        ability::Ability,
        parsed::stats::form::{CatFormStats, EnemyType},
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

fn get_ability(link: &str, display: &str) -> String {
    format!("[[Special Abilities#{link}|{display}]]")
}

/// DOES NOT DO MULTIHIT
pub fn get_abilities(stats: &CatFormStats) -> String {
    let mut abilities = vec![];
    let mut immunities = vec![];

    let targets = get_targets(&stats.targets);

    for ability in &stats.abilities {
        match ability {
            Ability::StrongAgainst => abilities.push(format!(
                "{strong} against {targets} enemies (Deals 1.5x damage, only takes 1/2 damage)",
                strong = get_ability("Strong Against", "Strong")
            )),
            Ability::Knockback { chance } => todo!(),
            Ability::Freeze { chance, duration } => todo!(),
            Ability::Slow { chance, duration } => todo!(),
            Ability::Resist => todo!(),
            Ability::MassiveDamage => todo!(),
            Ability::Crit { chance } => todo!(),
            Ability::TargetsOnly => todo!(),
            Ability::DoubleBounty => todo!(),
            Ability::BaseDestroyer => todo!(),
            Ability::Wave(wave) => todo!(),
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
