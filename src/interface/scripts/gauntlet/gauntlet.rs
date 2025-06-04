use super::super::{
    map_info::map_info::reference,
    stage_info::{
        battlegrounds::battlegrounds,
        beginning::enemies_appearing,
        enemies_list::enemies_list,
        information::{base_hp, energy, max_enemies, stage_location, stage_name, width, xp},
        misc_information::{chapter, max_clears, star},
        restrictions::{restrictions_info, restrictions_section, rules},
        stage_info::get_stage_wiki_data,
        treasure::treasure,
    },
};
use crate::{
    game_data::{
        meta::stage::{map_id::MapID, stage_id::StageID},
        stage::parsed::{
            stage::Stage,
            stage_enemy::{MS_SIGN, Magnification, StageEnemy},
        },
    },
    interface::{config::Config, error_handler::InfallibleWrite},
    wiki_data::enemy_data::ENEMY_DATA,
    wikitext::{
        section::Section,
        tabber::{Tabber, TabberTab, TabberType},
        template::Template,
        text_utils::extract_link,
    },
};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use std::fmt::{Display, Write};

/// Get the template used for comparing [`TabInfo`].
fn template_check(stage: &Stage) -> Template {
    Template::named("Stage Info")
        .add_params(enemies_list(stage, true))
        .add_params(restrictions_info(stage))
        .add_params(width(stage))
        .add_params(max_enemies(stage))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Container for a tab's info.
///
/// If two stages have equivalent [`TabInfo`] then they can be put together into
/// one tab on the Gauntlet page.
struct TabInfo {
    enemies_appearing: String,
    infobox: String,
    rules: String,
    restrictions: String,
    battlegrounds: String,
}

/// Information about a tab on the Gauntlet page, and the ids of all the stages
/// in that tab.
type TabInfoWithStages = (Vec<u32>, TabInfo);

/// Get tab info for all stages.
fn stages_tab_info(stages: &[Stage]) -> Option<Vec<TabInfoWithStages>> {
    let mut containers: Vec<TabInfoWithStages> = vec![];

    for stage in stages {
        let container = TabInfo {
            enemies_appearing: enemies_appearing(stage),
            infobox: template_check(stage).to_string(),
            rules: rules(stage),
            restrictions: restrictions_section(stage),
            battlegrounds: battlegrounds(stage),
        };

        let i = stage.id.num();
        match containers.iter_mut().find(|item| item.1 == container) {
            Some(cont) => cont.0.push(i),
            None => containers.push((vec![i], container)),
            // add id to [`TabInfoWithStages`] or add new item to the container
            // vec if tabinfo not found
        }
    }

    if containers.iter().all(|s| s.0.len() == 1) {
        None
    } else {
        Some(containers)
    }
}

/// Range of stage ids.
pub struct StageRange {
    min: u32,
    max: u32,
}
impl StageRange {
    const fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }

    /// Convert stage id list to [`StageRange`] list.
    fn get_ranges(ids: &[u32]) -> Vec<Self> {
        let mut range_min = ids[0];
        let mut ranges = vec![];

        let mut iter = ids.iter().peekable();
        while let Some(id) = iter.next() {
            match iter.peek() {
                None => {
                    ranges.push(Self::new(range_min, *id));
                    // break;
                }
                Some(&&i) if i != id + 1 => {
                    ranges.push(Self::new(range_min, *id));
                    range_min = i;
                }
                Some(_) => (),
            }
        }

        ranges
    }
}
impl Display for StageRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.min == self.max {
            write!(f, "{}", self.min + 1)
        } else {
            write!(f, "{}~{}", self.min + 1, self.max + 1)
        }
    }
}

/// Get ids of all enemies in the stage.
fn get_enemies_by_id(stage: &Stage) -> Vec<u32> {
    // if let Some(rewards) = stage1.rewards
    if stage.rewards.is_some() && !stage.rewards.as_ref().unwrap().score_rewards.is_empty() {
        todo!(
            "Stage {id} has score rewards which are currently not supported with gauntlets",
            id = stage.id
        );
        // TODO something about score rewards
        // TODO if let chain
    }

    let mut enemies_by_id = vec![];
    for enemy in &stage.enemies {
        // needs to be kept in line with `enemy_mag_lines`
        if enemy.id == MS_SIGN {
            continue;
        }
        if !enemies_by_id.contains(&enemy.id) {
            enemies_by_id.push(enemy.id);
        }
    }
    enemies_by_id
}

/// Get a list of unique magnifications for each enemy.
///
/// If `enemies_by_id` is `[35, 42]` and `enemies` has enemy 35 at 300% and
/// enemy 42 at 400% and 500% then this will give you `[[300], [400, 500]]`.
/// Unfortunately [`StageEnemy`] does way too much for this to be a viable
/// doctest.
// TODO create a simpler subset of stageenemy that can be doctested, and fix
// filter duplication.
fn enemy_mag_lines(enemies_by_id: &[u32], enemies: &[StageEnemy]) -> Vec<Vec<Magnification>> {
    let mut mags = vec![Vec::new(); enemies_by_id.len()];
    for enemy in enemies {
        // needs to be kept in line with `get_enemies_by_id`
        if enemy.id == MS_SIGN {
            continue;
        }
        let pos = enemies_by_id
            .iter()
            .position(|eid| *eid == enemy.id)
            .unwrap();

        if !mags[pos].contains(&enemy.magnification) {
            mags[pos].push(enemy.magnification);
        }
    }
    mags
}

/// Write a single enemy magnification in the table.
fn write_single_mag(buf: &mut String, mag: &Magnification) {
    match mag {
        Left(n) => {
            buf.write_formatted(n, &Locale::en).infallible_write();
            buf.write_str("%").infallible_write();
        }
        Right((hp, ap)) => {
            buf.write_formatted(hp, &Locale::en).infallible_write();
            buf.write_str("% HP/").infallible_write();
            buf.write_formatted(ap, &Locale::en).infallible_write();
            buf.write_str("% AP").infallible_write();
        }
    }
}

/// Write a single data row to the table.
fn write_table_row(line_buf: &mut String, enemies_by_id: &[u32], stage: &Stage) {
    for mag_line in enemy_mag_lines(enemies_by_id, &stage.enemies) {
        line_buf.write_str("|").infallible_write();

        let mut mags_iter = mag_line.iter();
        write_single_mag(line_buf, mags_iter.next().unwrap());
        for mag in mags_iter {
            *line_buf += ", ";
            write_single_mag(line_buf, mag);
        }
        line_buf.write_str("\n").infallible_write();
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

/// Get gauntlet scale table.
fn get_table(stages: &[Stage], ranges: &[StageRange]) -> String {
    let stage1 = &stages[ranges[0].min as usize];
    let enemies_by_id = get_enemies_by_id(stage1);

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
        for i in range.min..=range.max {
            write!(table, "|-\n! scope=\"row\" |{}\n", i + 1).unwrap();
            // new row, add stage number marker
            let stage = &stages[i as usize];
            write_table_row(&mut table, &enemies_by_id, stage);
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
            Ok(stage) => stage,
            Err(_) => break,
        };
        stages.push(stage);
    }

    stages
}

/// Get full tabber for gauntlet map.
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
        let ranges = StageRange::get_ranges(&tab.0);
        let tab_stage0 = &stages[ranges[0].min as usize];

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
            0 => unreachable!(),
            1 => ranges[0].to_string(),
            _ => ranges
                .iter()
                .map(|range: &StageRange| range.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        };

        let title = {
            let map = data.stage_map.get(tab_stage0.id.num()).unwrap();
            let link = extract_link(&map.name);

            if let Some(pos) = link.find('#') {
                link[pos + 1..].to_string()
            } else {
                if len > 1 {
                    log::warn!("`#` character not found in gauntlet name: {link:?}");
                }
                format!("Level {range_str}")
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

        tabber.content.push(tab);
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

// TODO test with Ms. Sign in stage
