use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};

use super::{
    stage_info::{
        enemies_list::enemies_list,
        information::{max_enemies, stage_location, stage_name, width},
        misc_information::{chapter, max_clears, star},
        restrictions::restrictions_info,
    },
    tabber::{Tabber, TabberTab, TabberType},
    template::Template,
};
use crate::{
    config::Config,
    data::stage::parsed::{stage::Stage, stage_enemy::Magnification},
    meta::stage::{map_id::MapID, stage_id::StageID, variant::StageVariantID as T},
    wikitext::{
        data_files::enemy_data::ENEMY_DATA,
        map_info::reference,
        section::Section,
        stage_info::{
            battlegrounds::battlegrounds,
            beginning::enemies_appearing,
            get_stage_wiki_data,
            information::{base_hp, energy, xp},
            restrictions::{restrictions_section, rules},
            treasure::treasure,
        },
    },
};
use std::fmt::Write;

fn template_check(stage: &Stage) -> Template {
    Template::named("Stage Info")
        .add_params(enemies_list(stage, true))
        .add_params(restrictions_info(stage))
        .add_params(width(stage))
        .add_params(max_enemies(stage))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Container for a tab's info.
struct TabInfo {
    enemies_appearing: String,
    infobox: String,
    rules: String,
    restrictions: String,
    battlegrounds: String,
}

/// Information about a tab with stage ids.
type TabInfoWithStages = (Vec<u32>, TabInfo);

fn stages_tab_info(stages: &[Stage]) -> Option<Vec<TabInfoWithStages>> {
    let mut containers: Vec<TabInfoWithStages> = vec![];

    for stage in stages {
        let container = TabInfo {
            enemies_appearing: enemies_appearing(&stage),
            infobox: template_check(&stage).to_string(),
            rules: rules(&stage),
            restrictions: restrictions_section(&stage),
            battlegrounds: battlegrounds(&stage),
        };

        let i = stage.id.num();
        match containers.iter_mut().find(|item| (**item).1 == container) {
            Some(cont) => cont.0.push(i),
            None => containers.push((vec![i], container)),
        }
    }

    if containers.iter().all(|s| s.0.len() == 1) {
        None
    } else {
        Some(containers)
    }
}

/// Range of stage ids.
type StageRange = (u32, u32);

fn get_ranges(ids: &[u32]) -> Vec<StageRange> {
    let mut range_min = ids[0];
    let mut ranges = vec![];

    let mut iter = ids.iter().peekable();
    while let Some(id) = iter.next() {
        match iter.peek() {
            None => {
                ranges.push((range_min, *id));
                continue;
            }
            Some(&&i) if i != id + 1 => {
                ranges.push((range_min, *id));
                range_min = i
            }
            Some(_) => (),
        }
    }

    ranges
}

/// Get text representation of a range.
fn get_range_repr(range: StageRange) -> String {
    if range.0 == range.1 {
        (range.0 + 1).to_string()
    } else {
        format!("{}~{}", range.0 + 1, range.1 + 1)
    }
}

fn get_enemies_by_id(stages: &[Stage], ranges: &[(u32, u32)]) -> Vec<u32> {
    let stage1 = &stages[ranges[0].0 as usize];

    // if let Some(rewards) = stage1.rewards
    if stage1.rewards.is_some() && stage1.rewards.as_ref().unwrap().score_rewards.len() != 0 {
        todo!(
            "Stage {id} has score rewards which are currently not supported with gauntlets",
            id = stage1.id
        );
        // TODO something about score rewards
    }

    let mut enemies_by_id = vec![];
    for enemy in stage1.enemies.iter() {
        if enemies_by_id
            .iter()
            .position(|eid| *eid == enemy.id)
            .is_none()
        {
            enemies_by_id.push(enemy.id)
        }
    }
    enemies_by_id
}

fn write_single_mag(buf: &mut String, mag: &Magnification) {
    match mag {
        Left(n) => {
            buf.write_formatted(n, &Locale::en).unwrap();
            buf.write_str("%").unwrap();
        }
        Right((hp, ap)) => {
            buf.write_formatted(hp, &Locale::en).unwrap();
            buf.write_str("% HP/").unwrap();
            buf.write_formatted(ap, &Locale::en).unwrap();
            buf.write_str("% AP").unwrap();
        }
    }
}

fn write_table_line(line_buf: &mut String, enemies_by_id: &Vec<u32>, stage: &Stage) {
    let mut mags = vec![Vec::new(); enemies_by_id.len()];
    for enemy in &stage.enemies {
        let pos = enemies_by_id
            .iter()
            .position(|eid| *eid == enemy.id)
            .unwrap();

        if mags[pos]
            .iter()
            .position(|mag| enemy.magnification == *mag)
            .is_none()
        {
            mags[pos].push(enemy.magnification);
        }
    }

    for mag1 in mags {
        line_buf.write_str("|").unwrap();

        let mut mag_iter = mag1.iter();
        write_single_mag(line_buf, mag_iter.next().unwrap());
        for mag in mag_iter {
            *line_buf += ", ";
            write_single_mag(line_buf, mag);
        }
        line_buf.write_str("\n").unwrap();
    }

    let rewards = match treasure(stage) {
        Some(t) => {
            let mut c = t.value.as_ref();
            c = c.strip_prefix("- ").unwrap();
            c = c.strip_suffix(" (100%, 1 time)").unwrap();
            c.to_string()
        }
        None => "None".to_string(),
    };
    write!(
        line_buf,
        "|{base_hp}\n|{energy}\n|{rewards}\n|{xp}",
        base_hp = base_hp(stage)[0].value,
        energy = energy(stage).unwrap().value,
        rewards = rewards,
        xp = xp(stage).unwrap().value,
    )
    .unwrap();
}

fn get_table(stages: &[Stage], ranges: &[StageRange]) -> String {
    let enemies_by_id = get_enemies_by_id(stages, ranges);

    let colspan = enemies_by_id.len();
    const CENTER: &str = "\"text-align: center;\"";
    const START: &str =
        "{| class=\"article-table\" border=\"0\" cellpadding=\"1\" cellspacing=\"1\"";

    let mut table = format!(
        "{START}\n|-\n\
        ! rowspan=\"2\" style={CENTER} |Stage\n\
        ! colspan=\"{colspan}\" style={CENTER} |Strength Magnifications\n\
        ! rowspan=\"2\" style={CENTER} |Base HP\n\
        ! rowspan=\"2\" style={CENTER} |Energy Cost\n\
        ! colspan=\"2\" style={CENTER} |Rewards\n\
        |-\n\
        "
    );

    const SCOPE: &str = "scope=\"col\"";
    for enemy in &enemies_by_id {
        let name = &ENEMY_DATA.get_names(*enemy).name;
        writeln!(table, "! {SCOPE} |{name}").unwrap();
    }

    writeln!(
        table,
        "! style={CENTER} |Treasure\n\
        ! style={CENTER} |Base XP"
    )
    .unwrap();

    for range in ranges {
        for i in range.0..=range.1 {
            writeln!(table, "|-\n! scope=\"row\" |{}", i + 1).unwrap();
            let stage = &stages[i as usize];
            write_table_line(&mut table, &enemies_by_id, stage);
            table.write_str("\n").unwrap();
        }
    }

    table.write_str("|}").unwrap();
    table
}

fn get_stages(map_id: &MapID, config: &Config) -> Vec<Stage> {
    let mut stages = vec![];
    for i in 0..100 {
        let id = StageID::from_map(map_id.clone(), i);
        let stage = match Stage::from_id(id, config.version.current_version()) {
            Some(stage) => stage,
            None => break,
        };
        stages.push(stage);
    }

    stages
}

fn do_thing_single(map_id: &MapID, config: &Config) -> Tabber {
    let stages = get_stages(map_id, config);
    let containers = stages_tab_info(&stages).unwrap_or_default();
    let stage1 = &stages[0];
    let data = get_stage_wiki_data(&stage1.id);

    let sname = stage_name(stage1, config.version.lang());
    let sloc = stage_location(stage1, config.version.lang());
    let schap = chapter(stage1, &data);

    let mut tabber = Tabber::new(TabberType::Tabber, vec![]);

    for tab in containers {
        let ranges = get_ranges(&tab.0);
        let stage_first = &stages[ranges[0].0 as usize];
        let template = Template::named("Stage Info")
            .add_params(sname.clone())
            .add_params(sloc.clone())
            .add_params(enemies_list(stage_first, true))
            .add_params(restrictions_info(stage_first))
            .add_params(width(stage_first))
            .add_params(max_enemies(stage_first))
            .add_const(&[("jpname", "?"), ("script", "?"), ("romaji", "?")])
            .add_params(star(stage_first))
            .add_params(schap.clone())
            .add_params(max_clears(stage_first));

        let table = get_table(&stages, &ranges);

        let cont = tab.1;
        let sections = [
            Section::blank(cont.enemies_appearing),
            Section::blank(template.to_string()),
            Section::h2("Rules", cont.rules),
            Section::h2("Restrictions", cont.restrictions),
            Section::h2("Battleground", cont.battlegrounds),
            Section::h2("Details", table),
            Section::h2("Strategy", "-"),
        ];

        let range_str = match ranges.len() {
            1 => get_range_repr(ranges[0]),
            _ => ranges
                .iter()
                .map(|range: &StageRange| get_range_repr(*range))
                .collect::<Vec<_>>()
                .join(", "),
        };

        let tab = TabberTab {
            title: format!("Level {range_str}"),
            content: sections
                .iter()
                .filter_map(|s| {
                    if s.content.is_empty() {
                        None
                    } else {
                        Some(s.to_string())
                    }
                })
                .collect::<Vec<_>>()
                .join("\n\n"),
        };

        tabber.content.push(tab)
    }
    tabber
}

pub fn do_thing(config: &Config) {
    let map_ids = [
        MapID::from_components(T::Gauntlet, 0),
        // baron
        MapID::from_components(T::Gauntlet, 19),
        // sbc
        MapID::from_components(T::CollabGauntlet, 7),
        // heralds of the end
        MapID::from_components(T::CollabGauntlet, 22),
        // baki gauntlet
    ];
    for map_id in map_ids {
        let tabber = do_thing_single(&map_id, config);
        let mut buf = match tabber.content.len() {
            // 0 => panic!(),
            1 => tabber.content[0].content.clone(),
            _ => tabber.to_string(),
        };

        let dbref = Section::h2("Reference", format!("*{}", reference(&map_id)));
        write!(buf, "\n\n{dbref}").unwrap();

        println!("{buf}");
    }

    panic!();
    // panic!("{stages2:?}");
}
