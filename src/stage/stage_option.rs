use charagroups::CHARAGROUP;

pub mod charagroups {
    use crate::file_handler::{get_file_location, FileLocation};
    use std::sync::LazyLock;

    #[derive(Debug, serde::Deserialize)]
    /// Fixed csv data in Charagroup.csv.
    struct CharaGroupFixedCSV {
        /// ID of charagroup.
        group_id: u32,
        /// Basically just `stage_restriction_charagroup_{group_id}`.
        _text_id: String,
        /// 0 = Can only use, 2 = can't use
        group_type: u32,
    }

    #[derive(Debug)]
    /// Type of the Charagroup.
    pub enum CharaGroupType {
        /// Can only use select cats.
        OnlyUse,
        /// Cannot use select cats.
        CannotUse,
    }

    impl From<u32> for CharaGroupType {
        fn from(value: u32) -> Self {
            match value {
                0 => Self::OnlyUse,
                2 => Self::CannotUse,
                _ => panic!("Value {value} is not recognised as a valid charagroup!"),
            }
        }
    }

    #[derive(Debug)]
    /// Data about a CharaGroup.
    pub struct CharaGroup {
        /// Type of charagroup.
        group_type: CharaGroupType,
        /// Units in charagroup.
        units: Vec<u32>,
    }

    /// Container for static data.
    pub struct CharaGroups {
        parsed_file: LazyLock<Vec<CharaGroup>>,
    }
    impl CharaGroups {
        const fn new() -> Self {
            CharaGroups {
                parsed_file: LazyLock::new(|| read_charagroup_file()),
            }
        }

        /// Get charagroup with id `id`.
        pub fn get_charagroup(&self, id: u32) -> &CharaGroup {
            &self.parsed_file[usize::try_from(id - 1).unwrap()]
        }
    }

    /// If you want group 1 then do `CHARAGROUP.get_charagroup(1)`.
    pub static CHARAGROUP: CharaGroups = CharaGroups::new();

    /// Reads the charagroup file and passes it into a vec of
    /// [CharaGroups][CharaGroup].
    fn read_charagroup_file() -> Vec<CharaGroup> {
        let path = get_file_location(FileLocation::GameData).join("DataLocal/Charagroup.csv");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)
            .unwrap();

        let mut records = rdr.byte_records();
        records.next();

        let mut count = 0;
        records
            .into_iter()
            .map(|record| {
                let result = record.unwrap();
                let fixed_data: CharaGroupFixedCSV = result.deserialize(None).unwrap();

                count += 1;
                debug_assert_eq!(count, fixed_data.group_id);

                let max_ind = if result[result.len() - 1].is_empty() {
                    result.len() - 1
                } else {
                    result.len()
                };

                let mut units: Vec<u32> = vec![];
                for i in 3..max_ind {
                    let n = std::str::from_utf8(&result[i])
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    units.push(n)
                }

                CharaGroup {
                    group_type: fixed_data.group_type.into(),
                    units,
                }
            })
            .collect()
    }
}
/*
class Restrictions:
    # ht15=Group
    # ht16=Level

    # DataLocal/Charagroup.csv
    # DataLocal/Stage_option.csv

    group:      Tuple[int, List[str]]
    '''Specific charagroups e.g. Finale's restriction only allowing you to spawn
    Cat. Arg 1 is mode (0 = only use, 2 = can't use), arg 2 is allowed units'''
        # //mapID, compatible★, stageID, rarity limit, cat limit, slot formation limit, production cost limit, upper limit, groupID
        # groupID requires using Charagroup.csv
        self.group = self.get_group(line[8])

    def get_group(self, lim) -> Tuple[int, List[str]]:
        if lim == "0":
            return (0, [])
        self.get_charagroup()

        for char in self.charagroups:
            if char[0] == lim:
                break

        mode = int(char[2])
        cats = []
        for cat in char[3:]:
            if not cat:
                continue

            cats.append(CatName.get_cat_link(cat))
        return mode, cats

    charagroups:    List[List[str]] = None
    def get_charagroup(cls):
        try:
            if not cls.charagroups:
                with open(f'{Options.data_mines}/DataLocal/Charagroup.csv', encoding='utf-8') as f:
                    cls.charagroups = list(csv.reader(f))
        except FileNotFoundError:
            pass
*/

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
/// Data stored in the stage option CSV. Details the restrictions on individual
/// stages within the map.
///
/// If map has multiple restrictions it will have multiple entries in the file.
/// If any restriction field is 0 then that restriction does not apply.
pub struct StageOptionCSV {
    /// Same as [`map_option`'s][crate::map::map_option::MapOptionCSV::map_id].
    pub map_id: u32,
    /// Crown difficulties that restriction applies to. -1 = all crowns,
    /// otherwise it's just 0-based.
    pub stars: u32,
    /// If is -1 then applies to all stages in map. Otherwise only applies to
    /// the stage in the map with that id.
    pub stage_id: u32,
    /// Rarities allowed. Binary value.
    pub rarity: u32,
    /// Cat deploy limit.
    pub deploy_limit: u32,
    /// Rows that you can deploy from.
    pub rows: u32,
    /// Minimum unit cost.
    pub min_cost: u32,
    /// Maximum unit cost.
    pub max_cost: u32,
    // TODO need to use charagroup to document this.
    pub charagroup: u32,
}

// Okay how to do this

pub fn also_do_stuff() {
    println!("{:?}", CHARAGROUP.get_charagroup(1));
}
