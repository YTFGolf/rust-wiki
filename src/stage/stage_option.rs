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
    pub chara_group: u32,
}

// Okay how to do this
