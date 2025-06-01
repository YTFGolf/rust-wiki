use crate::{
    game_data::cat::{
        ability::{Ability, Wave, WaveType},
        parsed::stats::form::{AttackHits, CatFormStats, EnemyType},
    },
    interface::error_handler::InfallibleWrite,
    wikitext::number_utils::{plural_f, time_repr},
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

/// Get display for ability procs when a cat has multiple hits.
/// ```
/// # use rust_wiki::game_data::cat::parsed::stats::form::{AttackHit, AttackHits};
/// # use rust_wiki::get_multiple_hit_abilities;
/// let multab = get_multiple_hit_abilities;
/// let normal = AttackHits::Single([AttackHit { active_ability: true, ..Default::default() }]);
/// assert_eq!(multab(&normal), "");
/// let unique = AttackHits::Triple([AttackHit { active_ability: false, ..Default::default() },AttackHit { active_ability: true, ..Default::default() },AttackHit { active_ability: true, ..Default::default() }]);
/// assert_eq!(multab(&unique), " on 2nd and 3rd hits");
/// ```
pub fn get_multiple_hit_abilities(hits: &AttackHits) -> &'static str {
    // TODO should really be a normal test
    match hits {
        AttackHits::Single([hit1]) => match hit1.active_ability {
            true => "",
            false => unreachable!(),
        },
        AttackHits::Double([hit1, hit2]) => match [hit1.active_ability, hit2.active_ability] {
            [true, true] => "",
            [true, false] => " on 1st hit",
            [false, true] => " on 2nd hit",
            [false, false] => unreachable!(),
        },
        AttackHits::Triple([hit1, hit2, hit3]) => match [
            hit1.active_ability,
            hit2.active_ability,
            hit3.active_ability,
        ] {
            [true, true, true] => "",
            [true, true, false] => " on 1st and 2nd hits",
            [true, false, true] => " on 1st and 3rd hits",
            [false, true, true] => " on 2nd and 3rd hits",
            [true, false, false] => " on 1st hit",
            [false, true, false] => " on 2nd hit",
            [false, false, true] => " on 3rd hit",
            [false, false, false] => unreachable!(),
        },
    }
}

fn get_duration_repr(duration: u32) -> String {
    let (dur_f, dur_s) = time_repr(duration);
    format!(
        "{dur_f}f <sub>{dur_s} {seconds}</sub>",
        seconds = plural_f(duration, "second", "seconds")
    )
}

fn get_ability(link: &str, display: &str) -> String {
    format!("[[Special Abilities#{link}|{display}]]")
}
fn get_ability_link_display(link_display: &str) -> String {
    format!("[[Special Abilities#{link_display}|{link_display}]]")
}
fn get_enemy_category(link: &str, display: &str) -> String {
    format!("[[:Category:{link} Enemies|{display}]]")
}

/// DOES NOT DO MULTIHIT
pub fn get_abilities(stats: &CatFormStats) -> String {
    let mut abilities = vec![];
    // start: multihit, ld, omni
    let mut immunities = vec![];

    let targets = get_targets(&stats.targets);
    let multab = get_multiple_hit_abilities(&stats.attack.hits);

    let abil = get_ability;
    let abil2 = get_ability_link_display;
    let enemy = get_enemy_category;
    // shorthand makes rest look readable

    for ability in &stats.abilities {
        // TODO remove multab here, instead use ability methods?
        // use strum::EnumIter to make assertions, first check is_immunity
        // weaken will intentionally fail the test
        match ability {
            Ability::StrongAgainst => abilities.push(format!(
                "{strong} against {targets} enemies (Deals 1.5x damage, only takes 1/2 damage)",
                strong = abil("Strong Against", "Strong")
            )),
            Ability::Knockback { chance } => abilities.push(format!(
                "{chance}% chance to {knockback} {targets} enemies{multab}",
                knockback = abil("Knockback", "knockback")
            )),
            Ability::Freeze { chance, duration } => abilities.push(format!(
                "{chance}% chance to {freeze} {targets} enemies for {duration}{multab}",
                freeze = abil("Freeze", "freeze"),
                duration = get_duration_repr(u32::from(*duration))
            )),
            Ability::Slow { chance, duration } => abilities.push(format!(
                "{chance}% chance to {slow} {targets} enemies for {duration}{multab}",
                slow = abil("Slow", "slow"),
                duration = get_duration_repr(u32::from(*duration))
            )),
            Ability::Resist => abilities.push(format!(
                "{resistant} to {targets} enemies",
                resistant = abil2("Resistant")
            )),
            Ability::MassiveDamage => abilities.push(format!(
                "Deals {damage} to {targets} enemies",
                damage = abil("Massive Damage", "massive damage")
            )),
            Ability::Crit { chance } => abilities.push(format!(
                "{chance}% chance to perform a {crit}{multab}",
                crit = abil("Critical", "critical hit")
            )),
            Ability::TargetsOnly => abilities.push(format!(
                "{attacks} {targets} enemies",
                attacks = abil("Attacks Only", "Attacks only")
            )),
            Ability::DoubleBounty => abilities.push(format!(
                "{money} gained when defeating enemies",
                money = abil("Extra Money", "Double money")
            )),
            Ability::BaseDestroyer => abilities.push(abil2("Base Destroyer")),
            Ability::Wave(Wave {
                wtype,
                chance,
                level,
            }) => {
                let wave = match wtype {
                    WaveType::Wave => "[[Wave Attack]]",
                    WaveType::MiniWave => "[[Wave Attack#Mini-Wave|Mini-Wave]]",
                };
                abilities.push(format!(
                    "{chance}% chance to create a level {level} {wave}{multab}"
                ))
            }
            Ability::Weaken {
                chance,
                duration,
                multiplier,
            } => abilities.push(format!(
                "{chance}% chance to {weaken} {targets} enemies \
                to {multiplier}% for {duration}",
                weaken = abil("Weaken", "weaken"),
                duration = get_duration_repr(u32::from(*duration))
            )),
            Ability::Strengthen { hp, multiplier } => abilities.push(format!(
                "{strengthens} by {multiplier}% at {hp}% health",
                strengthens = abil("Strengthen", "Strengthens")
            )),
            Ability::Survives { chance } => abilities.push(format!(
                "{chance}% chance to {survive} a lethal strike",
                survive = abil("Survive", "survive")
            )),
            Ability::Metal => abilities.push(format!(
                "{metal} (Only takes 1 damage from non-\
                [[Critical Hit|Critical]] or [[Toxic]] attacks)",
                metal = abil2("Metal")
            )),
            Ability::WaveBlocker => abilities.push(abil2("Wave Shield")),
            Ability::ZombieKiller => abilities.push(format!(
                "{killer} (stops {zombies} from reviving)",
                zombies = enemy("Zombie", "Zombies"),
                killer = abil2("Zombie Killer")
            )),
            Ability::WitchKiller => abilities.push(format!(
                "{killer} (Deals 5x damage to {witches}, only takes 1/10 damage)",
                witches = enemy("Witch", "Witches"),
                killer = abil2("Witch Killer")
            )),

            Ability::Kamikaze => abilities.push(format!(
                "{kamikaze} (Attacks once, then disappears from the battlefield)",
                kamikaze = abil2("Kamikaze")
            )),
            Ability::BarrierBreaker { chance } => abilities.push(format!(
                "{chance}% chance to {break} {barriers}",
                r#break = abil("Barrier Breaker", "break"),
                barriers = abil("Barrier", "barriers"),
            )),
            Ability::EvaAngelKiller => abilities.push(format!(
                "{killer} (Deals 5x damage to {angels}, only takes 1/5 damage)",
                killer = abil2("Eva Angel Killer"),
                angels = enemy("Eva Angel", "Eva Angels")
            )),
            Ability::InsaneResist => abilities.push(format!(
                "{tough} against {targets} enemies",
                tough = abil("Insanely Tough", "Insanely tough")
            )),
            Ability::InsaneDamage => abilities.push(format!(
                "Deals {damage} to {targets} enemies",
                damage = abil("Insane Damage", "insane damage")
            )),

            Ability::SavageBlow { chance, damage } => abilities.push(format!(
                "{chance}% chance to land a {blow} for +{damage}% damage to non-{metal} enemies",
                blow = abil("Savage Blow", "savage blow"),
                metal = enemy2("Metal")
            )),
            Ability::Dodge { chance, duration } => abilities.push(format!(
                "{chance}% chance to {dodge} attacks from {targets} enemies for {duration}",
                dodge = abil("Dodge Attack", "dodge"),
                duration = get_duration_repr(u32::from(*duration))
            )),
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
