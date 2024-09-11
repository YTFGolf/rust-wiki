mod consts {
    use lazy_static::lazy_static;
    use regex::Regex;

    #[derive(Debug)]
    pub struct StageCode<'a> {
        pub name: &'a str,
        pub number: i32,
        pub code: &'a str,
        pub has_r_prefix: bool,
    }

    const fn initialise_code<'a>(
        name: &'a str,
        number: i32,
        code: &'a str,
        has_r_prefix: bool,
    ) -> StageCode<'a> {
        StageCode {
            name,
            number,
            code,
            has_r_prefix,
        }
    }

    #[rustfmt::skip]
    pub const STAGE_CODES: [StageCode; 18] = [
        initialise_code("Stories of Legend",    000, "N",     true),
        initialise_code("Event Stages",         001, "S",     true),
        initialise_code("Collaboration Stages", 002, "C",     true),
        initialise_code("Main Chapters",        003, "main",  false),
        initialise_code("Extra Stages",         004, "RE|EX", false),
        initialise_code("Catclaw Dojo",         006, "T",     true),
        initialise_code("Towers",               007, "V",     true),
        initialise_code("Ranking Dojo",         011, "R",     true),
        initialise_code("Challenge Battle",     012, "M",     true),
        initialise_code("Uncanny Legends",      013, "NA",    true),
        initialise_code("Catamin Stages",       014, "B",     true),
        initialise_code("Gauntlets",            024, "A",     true),
        initialise_code("Enigma Stages",        025, "H",     true),
        initialise_code("Collab Gauntlets",     027, "CA",    true),
        initialise_code("Behemoth Culling",     031, "Q",     true),
        initialise_code("Labyrinth",            033, "L",     false),
        initialise_code("Zero Legends",         034, "ND",    true),
        initialise_code("Colosseum",            036, "SR",    true),
    ];

    #[derive(Debug)]
    pub struct StageTypeMap<'a> {
        pub matcher: Regex,
        pub stage_type: &'a str,
    }

    fn initialise_type_map<'a>(pattern: &'a str, stage_type: &'a str) -> StageTypeMap<'a> {
        let re = format!("(?:i)^({})$", pattern);
        let matcher = Regex::new(&re).unwrap();
        StageTypeMap {
            matcher,
            stage_type,
        }
    }

    lazy_static! {
    #[rustfmt::skip]
    pub static ref STAGE_TYPE_MAP: [StageTypeMap<'static>; 19] = [
        initialise_type_map("SoL|0|N|RN",                               "N"),
        initialise_type_map("Event|Special|1|S|RS",                     "S"),
        initialise_type_map("Collab|2|C|RC",                            "C"),
        initialise_type_map("Extra|4|RE|EX",                            "RE|EX"),
        initialise_type_map("Dojo|6|T|RT",                              "T"),
        initialise_type_map("Tower|7|V|RV",                             "V"),
        initialise_type_map("Rank|11|R|RR",                             "R"),
        initialise_type_map("Challenge|12|M|RM",                        "M"),
        initialise_type_map("UL|13|NA|RNA",                             "NA"),
        initialise_type_map("Catamin|14|B|RB",                          "B"),
        initialise_type_map("LQ|16|D",                                  "Why would you want to do Legend Quest"),
        initialise_type_map("Gauntlet|Baron|24|A|RA",                   "A"),
        initialise_type_map("Enigma|25|H|RH",                           "H"),
        initialise_type_map("27|CA|RCA",                                "CA"),
        initialise_type_map("Behemoth|31|Q|RQ",                         "Q"),
        initialise_type_map("Labyrinth|33|L",                           "L"),
        initialise_type_map("ZL|34|ND|RND",                             "ND"),
        initialise_type_map("Colosseum|36|SR|RSR",                      "SR"),
        initialise_type_map("EoC|ItF|W|CotC|Space|Aku|DM|Z|Filibuster", "main")
    ];
    }
}
use consts::{STAGE_TYPE_MAP, STAGE_CODES};

pub fn get_st_obj(selector: &str) -> &str {
    println!("{:?}", STAGE_CODES);
    println!("{:?}", STAGE_TYPE_MAP.iter().collect::<Vec<_>>());
    selector
}
