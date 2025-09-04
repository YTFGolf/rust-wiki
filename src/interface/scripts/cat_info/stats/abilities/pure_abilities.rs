use super::util::{get_ability, get_ability_single, get_duration_repr, get_enemy_category};
use crate::{
    game_data::cat::{
        ability::{Ability, Surge, SurgeType, Wave, WaveType},
        parsed::stats::form::{AttackHits, EnemyType, LATEST_ENEMY_TYPE},
    },
    interface::error_handler::InfallibleWrite,
    wikitext::number_utils::get_formatted_float,
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

    let max_type_index = LATEST_ENEMY_TYPE as usize;
    if len == max_type_index + 1 {
        return "all".to_string();
    }
    if len == max_type_index && !targets.contains(&EnemyType::Floating) {
        return "non-[[:Category:Floating Enemies|Floating]]".to_string();
    }
    if len == max_type_index && !targets.contains(&EnemyType::Metal) {
        return "non-[[:Category:Metal Enemies|Metal]]".to_string();
    }
    if len == max_type_index && !targets.contains(&EnemyType::Traitless) {
        return "traited".to_string();
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
#[cfg(test)]
mod get_targets_tests {
    use super::*;
    use crate::game_data::cat::parsed::stats::form::EnemyType as E;

    #[test]
    fn single() {
        let targets = [E::Aku];
        assert_eq!(get_targets(&targets), "[[:Category:Aku Enemies|Aku]]");
    }

    #[test]
    fn double() {
        let targets = [E::Traitless, E::Relic];
        assert_eq!(
            get_targets(&targets),
            "[[:Category:Traitless Enemies|Traitless]] and [[:Category:Relic Enemies|Relic]]"
        );
    }

    #[test]
    fn triple() {
        let targets = [E::Red, E::Floating, E::Angel];
        assert_eq!(
            get_targets(&targets),
            "[[:Category:Red Enemies|Red]], [[:Category:Floating Enemies|Floating]] and [[:Category:Angel Enemies|Angel]]"
        );
    }
}

fn get_multiple_hit_abilities(hits: &AttackHits) -> &'static str {
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
#[cfg(test)]
mod multiple_ability_tests {
    use super::*;
    use crate::game_data::cat::parsed::stats::form::AttackHit;

    #[test]
    fn basic() {
        let normal = AttackHits::Single([AttackHit {
            active_ability: true,
            ..Default::default()
        }]);
        assert_eq!(get_multiple_hit_abilities(&normal), "");
    }

    #[test]
    fn not_first() {
        let unique = AttackHits::Triple([
            AttackHit {
                active_ability: false,
                ..Default::default()
            },
            AttackHit {
                active_ability: true,
                ..Default::default()
            },
            AttackHit {
                active_ability: true,
                ..Default::default()
            },
        ]);
        assert_eq!(get_multiple_hit_abilities(&unique), " on 2nd and 3rd hits");
    }
}

/// Deals with pure abilities, i.e. not multihit, LD or Omni.
pub fn get_pure_abilities(
    hits: &AttackHits,
    cat_abilities: &[Ability],
    targets: &[EnemyType],
) -> Vec<String> {
    let mut abilities = vec![];
    let mut immunities = vec![];

    let targets = get_targets(targets);
    let multab = get_multiple_hit_abilities(hits);

    let abil = get_ability;
    let abil2 = get_ability_single;
    let enemy = get_enemy_category;
    let enemy2 = |ld| enemy(ld, ld);
    // shorthand makes rest look readable

    for ability in cat_abilities {
        let icon = String::from("{{AbilityIcon|") + ability.name() + "}}";
        match ability {
            Ability::StrongAgainst => abilities.push(format!(
                "{icon} {strong} against {targets} enemies (Deals 1.5x damage, only takes 1/2 damage)",
                strong = abil("Strong Against", "Strong")
            )),
            Ability::Knockback { chance } => abilities.push(format!(
                "{icon} {chance}% chance to {knockback} {targets} enemies{multab}",
                knockback = abil("Knockback", "knockback")
            )),
            Ability::Freeze { chance, duration } => abilities.push(format!(
                "{icon} {chance}% chance to {freeze} {targets} enemies for {duration}{multab}",
                freeze = abil("Freeze", "freeze"),
                duration = get_duration_repr(u32::from(*duration))
            )),
            Ability::Slow { chance, duration } => abilities.push(format!(
                "{icon} {chance}% chance to {slow} {targets} enemies for {duration}{multab}",
                slow = abil("Slow", "slow"),
                duration = get_duration_repr(u32::from(*duration))
            )),
            Ability::Resist => abilities.push(format!(
                "{icon} {resistant} to {targets} enemies",
                resistant = abil2("Resistant")
            )),
            Ability::MassiveDamage => abilities.push(format!(
                "{icon} Deals {damage} to {targets} enemies",
                damage = abil("Massive Damage", "massive damage")
            )),
            Ability::Crit { chance } => abilities.push(format!(
                "{icon} {chance}% chance to perform a [[Critical Hit]]{multab}"
            )),
            Ability::TargetsOnly => abilities.push(format!(
                "{icon} {attacks} {targets} enemies",
                attacks = abil("Attacks Only", "Attacks only")
            )),
            Ability::DoubleBounty => abilities.push(format!(
                "{icon} {money} gained when defeating enemies",
                money = abil("Extra Money", "Double money")
            )),
            Ability::BaseDestroyer => abilities.push(icon + " " + &abil2("Base Destroyer")),
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
                    "{icon} {chance}% chance to create a level {level} {wave}{multab}"
                ));
            }

            Ability::Weaken {
                chance,
                duration,
                multiplier,
            } => abilities.push(format!(
                "{icon} {chance}% chance to {weaken} {targets} enemies \
                to {multiplier}% for {duration}{multab}",
                weaken = abil("Weaken", "weaken"),
                duration = get_duration_repr(u32::from(*duration))
            )),
            Ability::Strengthen { hp, multiplier } => abilities.push(format!(
                "{icon} {strengthens} by {multiplier}% at {hp}% health",
                strengthens = abil("Strengthen", "Strengthens")
            )),
            Ability::Survives { chance } => abilities.push(format!(
                "{icon} {chance}% chance to {survive} a lethal strike",
                survive = abil("Survive", "survive")
            )),
            Ability::Metal => abilities.push(format!(
                "{icon} {metal} (Only takes 1 damage from non-\
                [[Critical Hit|Critical]] or [[Toxic]] attacks)",
                metal = abil2("Metal")
            )),
            Ability::WaveBlocker => abilities.push(icon + " " + &abil2("Wave Shield")),
            Ability::ZombieKiller => abilities.push(format!(
                "{icon} {killer} (stops {zombies} from reviving)",
                zombies = enemy("Zombie", "Zombies"),
                killer = abil2("Zombie Killer")
            )),
            Ability::WitchKiller => abilities.push(format!(
                "{icon} {killer} (Deals 5x damage to {witches}, only takes 1/10 damage)",
                witches = enemy("Witch", "Witches"),
                killer = abil2("Witch Killer")
            )),

            Ability::Kamikaze => abilities.push(format!(
                "{icon} {kamikaze} (Attacks once, then disappears from the battlefield)",
                kamikaze = abil2("Kamikaze")
            )),
            Ability::BarrierBreaker { chance } => abilities.push(format!(
                "{icon} {chance}% chance to {break} [[Barrier]]s{multab}",
                r#break = abil("Barrier Breaker", "break"),
            )),
            Ability::EvaAngelKiller => abilities.push(format!(
                "{icon} {killer} (Deals 5x damage to {angels}, only takes 1/5 damage)",
                killer = abil2("Eva Angel Killer"),
                angels = enemy("Eva Angel", "Eva Angels")
            )),
            Ability::InsaneResist => abilities.push(format!(
                "{icon} {tough} against {targets} enemies",
                tough = abil("Insanely Tough", "Insanely tough")
            )),
            Ability::InsaneDamage => abilities.push(format!(
                "{icon} Deals {damage} to {targets} enemies",
                damage = abil("Insane Damage", "insane damage")
            )),
            Ability::SavageBlow { chance, damage } => abilities.push(format!(
                "{icon} {chance}% chance to land a {blow} for +{damage}% \
                damage to non-{metal} enemies{multab}",
                blow = abil("Savage Blow", "savage blow"),
                metal = enemy2("Metal")
            )),
            Ability::Dodge { chance, duration } => abilities.push(format!(
                "{icon} {chance}% chance to {dodge} attacks from {targets} enemies for {duration}",
                dodge = abil("Dodge Attack", "dodge"),
                duration = get_duration_repr(u32::from(*duration))
            )),

            Ability::Surge(Surge {
                stype,
                chance,
                spawn_quad,
                range_quad,
                level,
            }) => {
                let surge = match stype {
                    SurgeType::Surge => "[[Surge Attack]]",
                    SurgeType::MiniSurge => "[[Surge Attack#Mini-Surge|Mini-Surge]]",
                };

                let at_position = {
                    let min_range = f64::from(*spawn_quad) / 4.0;
                    let max_range = min_range + f64::from(*range_quad) / 4.0;
                    if *range_quad == 0 {
                        format!("at {fmt} range", fmt = get_formatted_float(min_range, 2))
                        // where min = max, just comparing floats is a bit
                        // cringe and clippy complains
                    } else {
                        format!(
                            "between {min_fmt} and {max_fmt} range",
                            min_fmt = get_formatted_float(min_range, 2),
                            max_fmt = get_formatted_float(max_range, 2)
                        )
                    }
                };

                abilities.push(format!(
                    "{icon} {chance}% chance to create a level {level} {surge} {at_position}{multab}"
                ));
            }

            Ability::Curse { chance, duration } => abilities.push(format!(
                "{icon} {chance}% chance to {curse} {targets} enemies for {duration}{multab}",
                curse = abil("Curse", "curse"),
                duration = get_duration_repr(u32::from(*duration))
            )),
            Ability::ShieldPierce { chance } => abilities.push(format!(
                "{icon} {chance}% chance to instantly {pierce} [[Shield]]s{multab}",
                pierce = abil("Shield Piercing", "pierce")
            )),
            Ability::ColossusSlayer => abilities.push(format!(
                "{icon} {slayer} (Deals 1.6x damage to {colossus} enemies, only takes 0.7x damage)",
                slayer = abil2("Colossus Slayer"),
                colossus = "[[:Category:Colossus Enemies|Colossus]]"
            )),
            Ability::Soulstrike => abilities.push(icon + " " + &abil2("Soulstrike")),
            Ability::BehemothSlayer {
                dodge_chance,
                dodge_duration,
            } => abilities.push(format!(
                "{icon} {slayer} ({dodge_chance}% chance to dodge \
                {behemoth} enemies' attacks for {duration})",
                slayer = abil2("Behemoth Slayer"),
                behemoth = enemy2("Behemoth"),
                duration = get_duration_repr(u32::from(*dodge_duration))
            )),

            Ability::CounterSurge => {
                abilities.push(icon + " [[Surge Attack#Counter-Surge|Counter-Surge]]");
            }
            Ability::ConjureUnit { id } => abilities.push(format!(
                "{icon} When on the battlefield, tap icon again to \
                {conjure} unit {id:03} as its [[#Spirit|spirit]]",
                conjure = abil2("Conjure")
            )),
            Ability::SageSlayer => abilities.push(icon + " " + &abil2("Sage Slayer")),
            Ability::MetalKiller { damage } => abilities.push(format!(
                "{icon} {killer} (Deals {damage}% of {metal} enemies' current HP on hit)",
                killer = abil2("Metal Killer"),
                metal = enemy2("Metal")
            )),
            Ability::Explosion { chance, spawn_quad } => {
                let position = f64::from(*spawn_quad) / 4.0;
                abilities.push(format!(
                    "{icon} {chance}% chance to create an [[Explosion]] at {range} range{multab}",
                    range = get_formatted_float(position, 2)
                ));
            }

            Ability::ImmuneToWave => immunities.push((ability, "Waves")),
            Ability::ImmuneToKB => immunities.push((ability, "Knockback")),
            Ability::ImmuneToFreeze => immunities.push((ability, "Freeze")),
            Ability::ImmuneToSlow => immunities.push((ability, "Slow")),
            Ability::ImmuneToWeaken => immunities.push((ability, "Weaken")),
            Ability::ImmuneToBossShockwave => immunities.push((ability, "Boss Shockwave")),
            Ability::ImmuneToWarp => immunities.push((ability, "Warp")),
            Ability::ImmuneToCurse => immunities.push((ability, "Curse")),
            Ability::ImmuneToToxic => immunities.push((ability, "Toxic")),
            Ability::ImmuneToSurge => immunities.push((ability, "Surge")),
            Ability::ImmuneToExplosion => immunities.push((ability, "Explosions")),
        }
    }

    if !immunities.is_empty() {
        let mut icons_buf = String::new();
        let mut text_buf = String::new();
        let mut iter = immunities.into_iter().peekable();

        let (icon, first) = iter.next().expect("already check is_empty");
        write!(icons_buf, "{{{{AbilityIcon|{i}}}}}", i = icon.name()).infallible_write();
        write!(
            text_buf,
            "[[Special Abilities#Immune to {first}|Immune to {first}]]"
        )
        .infallible_write();

        while let Some((icon, immunity)) = iter.next() {
            write!(icons_buf, "{{{{AbilityIcon|{i}}}}}", i = icon.name()).infallible_write();

            let separator = match iter.peek() {
                Some(_) => ",",
                None => " and",
            };
            write!(
                text_buf,
                "{separator} [[Special Abilities#Immune to {immunity}|{immunity}]]"
            )
            .infallible_write();
        }

        abilities.push(icons_buf + " " + &text_buf);
    }

    abilities
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_data::cat::parsed::stats::form::AttackHit;
    use strum::IntoEnumIterator;

    #[test]
    fn test_multab_applies() {
        let cat_abilities = Ability::iter().collect::<Vec<_>>();
        let targets = [EnemyType::Red];
        let hits = AttackHits::Triple([
            AttackHit {
                active_ability: false,
                ..Default::default()
            },
            AttackHit {
                active_ability: true,
                ..Default::default()
            },
            AttackHit {
                active_ability: true,
                ..Default::default()
            },
        ]);

        let abilities = get_pure_abilities(&hits, &cat_abilities, &targets);

        let mut raw_iter = cat_abilities.into_iter();
        let mut repr_iter = abilities.iter();

        let mut counter = 0;

        'outer: while let (Some(mut raw), Some(repr)) = (raw_iter.next(), repr_iter.next()) {
            while raw.is_immunity() {
                match raw_iter.next() {
                    Some(r) => raw = r,
                    None => break 'outer,
                };
            }
            counter += 1;

            let ends_in_multab = repr.ends_with(" on 2nd and 3rd hits");
            if !raw.is_passive() {
                assert!(
                    ends_in_multab,
                    "Ability {raw:?} does not take into account MultiHit with variable activation"
                );
            } else {
                assert!(
                    !ends_in_multab,
                    "Ability {raw:?} takes into account MultiHit with variable activation when it shouldn't"
                );
            }
        }

        assert_eq!(counter + 1, abilities.len());
        // i.e. gone over everything but last item in abilities

        const FINAL_IMMUNITY: &str = "{{AbilityIcon|Immune to Waves|Immune to Knockback|Immune to Freeze|Immune to Slow|Immune to Weaken|Immune to Boss Shockwave|Immune to Warp|Immune to Curse|Immune to Toxic|Immune to Surge|Immune to Explosions}} [[Special Abilities#Immune to Waves|Immune to Waves]], [[Special Abilities#Immune to Knockback|Knockback]], [[Special Abilities#Immune to Freeze|Freeze]], [[Special Abilities#Immune to Slow|Slow]], [[Special Abilities#Immune to Weaken|Weaken]], [[Special Abilities#Immune to Boss Shockwave|Boss Shockwave]], [[Special Abilities#Immune to Warp|Warp]], [[Special Abilities#Immune to Curse|Curse]], [[Special Abilities#Immune to Toxic|Toxic]], [[Special Abilities#Immune to Surge|Surge]] and [[Special Abilities#Immune to Explosions|Explosions]]";
        assert_eq!(abilities[counter], FINAL_IMMUNITY);
    }

    #[test]
    fn test_icon_in_all() {
        let cat_abilities = Ability::iter().collect::<Vec<_>>();
        let targets = [EnemyType::Red];
        let hits = AttackHits::Triple([
            AttackHit {
                active_ability: false,
                ..Default::default()
            },
            AttackHit {
                active_ability: true,
                ..Default::default()
            },
            AttackHit {
                active_ability: true,
                ..Default::default()
            },
        ]);

        let abilities = get_pure_abilities(&hits, &cat_abilities, &targets);
        for ability in abilities {
            assert!(
                ability.contains("{{AbilityIcon|"),
                "{ability:?} does not contain the AbilityIcon template"
            );
            assert!(ability.contains("}} "),);
        }
    }
}
