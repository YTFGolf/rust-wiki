#![allow(missing_docs)]

use std::fmt::Display;

use crate::{
    config::Config,
    data::stage::parsed::stage::Stage,
    meta::stage::{map_id::MapID, stage_id::StageID, variant::StageVariantID as T},
    wikitext::stage_info::{
        battlegrounds::battlegrounds,
        beginning::enemies_appearing,
        get_stage_wiki_data,
        restrictions::{restrictions_section, rules},
    },
};

use super::{
    stage_info::{
        StageWikiDataContainer,
        enemies_list::enemies_list,
        information::{max_enemies, stage_location, stage_name, width},
        misc_information::{chapter, max_clears, star},
        restrictions::restrictions_info,
    },
    template::Template,
};

fn template_check(stage: &Stage) -> Template {
    Template::named("Stage Info")
        // .add_params(stage_name(stage, config.version.lang()))
        // .add_params(stage_location(stage, config.version.lang()))
        // .add_params(energy(stage))
        // .add_params(base_hp(stage))
        .add_params(enemies_list(stage, true))
        // .add_params(treasure(stage))
        .add_params(restrictions_info(stage))
        // .add_params(score_rewards(stage))
        // .add_params(xp(stage))
        .add_params(width(stage))
        .add_params(max_enemies(stage))
    // .add_const(&[("jpname", "?"), ("script", "?"), ("romaji", "?")])
    // .add_params(star(stage))
    // .add_params(chapter(stage, stage_wiki_data))
    // .add_params(max_clears(stage))
    // .add_params(difficulty(stage))
    // .add_params(stage_nav(stage, stage_wiki_data))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Container {
    enemies_appearing: String,
    info: String,
    rules: String,
    restrictions: String,
    battlegrounds: String,
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

fn get_containers(stages: &[Stage]) -> Option<Vec<(Vec<u32>, Container)>> {
    let mut containers: Vec<(Vec<u32>, Container)> = vec![];

    for stage in stages {
        let container = Container {
            enemies_appearing: enemies_appearing(&stage),
            info: template_check(&stage).to_string(),
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

fn get_ranges(ids: &[u32]) -> Vec<(u32, u32)> {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TabberType {
    Tabber,
    SubTabber,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TabberTab {
    title: String,
    content: String,
}
impl Display for TabberTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\n{}", self.title, self.content)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tabber {
    ttype: TabberType,
    content: Vec<TabberTab>,
}
impl Display for Tabber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (open, mid, close) = match self.ttype {
            TabberType::Tabber => ("<tabber>", "|-|", "</tabber>"),
            TabberType::SubTabber => ("{{#tag:tabber", "{{!}}-{{!}}", "}}"),
        };

        write!(f, "{open}\n")?;

        let mut iter = self.content.iter().peekable();
        while let Some(tab) = iter.next() {
            write!(f, "{tab}\n")?;
            if iter.peek().is_some() {
                write!(f, "\n{mid}\n")?;
            }
        }

        write!(f, "{close}")
    }
}

fn template_final(stage: &Stage) -> Template {
    Template::named("Stage Info")
        // .add_params(stage_name(stage, config.version.lang()))
        // .add_params(stage_location(stage, config.version.lang()))
        // .add_params(energy(stage))
        // .add_params(base_hp(stage))
        .add_params(enemies_list(stage, true))
        // .add_params(treasure(stage))
        .add_params(restrictions_info(stage))
        // .add_params(score_rewards(stage))
        // .add_params(xp(stage))
        .add_params(width(stage))
        .add_params(max_enemies(stage))
    // .add_const(&[("jpname", "?"), ("script", "?"), ("romaji", "?")])
    // .add_params(star(stage))
    // .add_params(chapter(stage, stage_wiki_data))
    // .add_params(max_clears(stage))
    // .add_params(difficulty(stage))
    // .add_params(stage_nav(stage, stage_wiki_data))
}

fn get_range_repr(range: (u32, u32)) -> String {
    if range.0 == range.1 {
        (range.0 + 1).to_string()
    } else {
        format!("{}~{}", range.0 + 1, range.1 + 1)
    }
}

fn do_thing_single(map_id: &MapID, config: &Config) -> Tabber {
    let stages = get_stages(map_id, config);
    let containers = get_containers(&stages);
    let stage1 = &stages[0];
    let data = get_stage_wiki_data(&stage1.id);

    let param1 = stage_name(stage1, config.version.lang());
    let param2 = stage_location(stage1, config.version.lang());
    let param3 = chapter(stage1, &data);

    let mut tabber = Tabber {
        ttype: TabberType::Tabber,
        content: vec![],
    };

    for container in containers.iter() {
        for tab in container {
            let ranges = get_ranges(&tab.0);
            let stage_first = &stages[ranges[0].0 as usize];
            let template = Template::named("Stage Info")
                .add_params(param1.clone())
                .add_params(param2.clone())
                .add_params(enemies_list(stage_first, true))
                .add_params(restrictions_info(stage_first))
                .add_params(width(stage_first))
                .add_params(max_enemies(stage_first))
                .add_const(&[("jpname", "?"), ("script", "?"), ("romaji", "?")])
                .add_params(star(stage_first))
                .add_params(param3.clone())
                .add_params(max_clears(stage_first));

            let range_str = match ranges.len() {
                1 => get_range_repr(ranges[0]),
                _ => ranges
                    .iter()
                    .map(|range: &(u32, u32)| get_range_repr(*range))
                    .collect::<Vec<_>>()
                    .join(", "),
            };

            let tab = TabberTab {
                title: format!("Level {range_str}"),
                content: template.to_string(),
            };

            tabber.content.push(tab)
        }
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
    let stages = map_ids
        .iter()
        .map(|map_id| do_thing_single(map_id, config))
        .collect::<Vec<_>>();

    for stage in stages {
        println!("{stage}")
    }

    panic!();
    // panic!("{stages2:?}");
}
