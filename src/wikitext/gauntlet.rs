//! Gaunlet page script.

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
    data::stage::parsed::{
        stage::Stage,
        stage_enemy::{Magnification, StageEnemy},
    },
    meta::stage::{map_id::MapID, stage_id::StageID},
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
        wiki_utils::extract_link,
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

/// Get a list of unique magnifications for each enemy.
///
/// If `enemies_by_id` is `[35, 42]` and `enemies` has enemy 35 at 300% and
/// enemy 42 at 400% and 500% then this will give you `[[300], [400, 500]]`.
/// Unfortunately [`StageEnemy`] does way too much for this to be a viable
/// doctest.
fn enemy_mag_lines(enemies_by_id: &Vec<u32>, enemies: &[StageEnemy]) -> Vec<Vec<Magnification>> {
    let mut mags = vec![Vec::new(); enemies_by_id.len()];
    for enemy in enemies {
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
    mags
}

fn write_table_line(line_buf: &mut String, enemies_by_id: &Vec<u32>, stage: &Stage) {
    let mag_lines = enemy_mag_lines(enemies_by_id, &stage.enemies);

    for mag_line in mag_lines {
        line_buf.write_str("|").unwrap();

        let mut mags_iter = mag_line.iter();
        write_single_mag(line_buf, mags_iter.next().unwrap());
        for mag in mags_iter {
            *line_buf += ", ";
            write_single_mag(line_buf, mag);
        }
        line_buf.write_str("\n").unwrap();
    }
    // |x%, y% HP/z% AP etc.

    let rewards = match treasure(stage) {
        Some(t) => {
            let mut c = t.value.as_ref();
            c = c.strip_prefix("- ").unwrap();
            c = c.strip_suffix(" (100%, 1 time)").unwrap();
            // "- {} (100%, 1 time)", capture the "{}" bit
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
    // this could potentially use a proper struct but tables are so un-ergonomic
    // anyway

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
    // row 1
    // mags and rewards have sub-columns so have colspan but not rowspan, others
    // have no sub-columns so no colspan but they need to take up 2 rows e.g.
    /*
    | Stage |           Mags          | HP  | Cost |    Rewards    |
    |       | Enemy 1 | Enemy 2 | ... |     |      | Treasure | XP |

    Stage, HP and cost take up 2 rows here, mags takes up 3 cols and rewards
    takes up 2 cols.
    */

    const SCOPE: &str = "scope=\"col\"";
    for enemy in &enemies_by_id {
        let name = &ENEMY_DATA.get_names(*enemy).name;
        writeln!(table, "! {SCOPE} |{name}").unwrap();
    }
    // names of each enemy

    write!(
        table,
        "! style={CENTER} |Treasure\n\
        ! style={CENTER} |Base XP\n"
    )
    .unwrap();
    // final two cols in second row

    for range in ranges {
        for i in range.0..=range.1 {
            write!(table, "|-\n! scope=\"row\" |{}\n", i + 1).unwrap();
            // new row, add stage number marker
            let stage = &stages[i as usize];
            write_table_line(&mut table, &enemies_by_id, stage);
            // other cols in line
            table.write_str("\n").unwrap();
        }
    }

    table.write_str("|}").unwrap();
    table
}

/// Get all valid stages in map.
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

fn map_tabber(map_id: &MapID, config: &Config) -> Tabber {
    let stages = get_stages(map_id, config);
    let gauntlet_tabs = stages_tab_info(&stages).unwrap_or_default();
    let len = gauntlet_tabs.len();
    // amount of tabs, useful to know for the logging warning below
    let stage0 = &stages[0];
    let data = get_stage_wiki_data(&stage0.id);

    let sname = stage_name(stage0, config.version.lang());
    let sloc = stage_location(stage0, config.version.lang());
    let schap = chapter(stage0, &data);

    let mut tabber = Tabber::new(TabberType::Tabber, vec![]);

    for tab in gauntlet_tabs {
        let ranges = get_ranges(&tab.0);
        let tab_stage0 = &stages[ranges[0].0 as usize];

        let template = Template::named("Stage Info")
            .add_params(sname.clone())
            .add_params(sloc.clone())
            .add_params(enemies_list(tab_stage0, true))
            .add_params(restrictions_info(tab_stage0))
            .add_params(width(tab_stage0))
            .add_params(max_enemies(tab_stage0))
            .add_const(&[("jpname", "?"), ("script", "?"), ("romaji", "?")])
            .add_params(star(tab_stage0))
            .add_params(schap.clone())
            .add_params(max_clears(tab_stage0));

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

        let title = {
            let map = data.stage_map.get(tab_stage0.id.num()).unwrap();
            let link = extract_link(&map.name);

            match link.find("#") {
                Some(pos) => link[pos + 1..].to_string(),
                None => {
                    if len > 1 {
                        log::warn!("`#` character not found in gauntlet name: {link:?}")
                    }
                    format!("Level {range_str}")
                }
            }
        };

        let tab = TabberTab {
            title,
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

/// Get all gauntlet stages for a map.
pub fn map_gauntlet(map_id: &MapID, config: &Config) -> String {
    let tabber = map_tabber(map_id, config);
    let mut buf = match tabber.content.len() {
        // 0 => panic!(),
        1 => tabber.content[0].content.clone(),
        _ => tabber.to_string(),
    };

    let dbref = Section::h2("Reference", format!("*{}", reference(map_id)));
    write!(buf, "\n\n{dbref}").unwrap();

    buf
}
