//! Deals with enemy encounters.

/*
# Flow
## Here
- Get enemy number.
- Find all stages with enemy. Get list of magnifications.

## Wikitext
- Order stages + sort out extra stages
  - Order is done by a Rust sort
  - Extra stages will be done with... something idk. Setting to 999 should work
    since if a stage is an earlier continuation then it would just appear before
    the later ones. Would also fix like proving ground continuations.
- Loop through sections:
  - Get stage names for each stage
  - Display stage names. Filter out if doesn't begin with '['.
    - Hashmap for map name display type
    - Vec for stage display type
- If Catamin or extra stages then should print dire warning
- Else copy to clipboard, message saying "copied to clipboard" in green

Classes:
- EncountersSection enum: contains ordering and initialisation as well.
- DisplayType enum: `Stage x` or map name

Other things:
- StageData::new; StageEnemy::get_magnification
- Some logging crate needed to log out which pages are skipped
- Testing can be done easily for small parts but the overall thing can only be
  measured empirically
*/

use regex::Regex;

use crate::{config::Config, data::stage::raw::stage_data::StageData};

/// Do thing (temp)
pub fn do_thing(config: Config) {
    let stage_file_re = Regex::new(r"^stage.*?\d{2}\.csv$").unwrap();

    let dir = &config.current_version.get_file_path("DataLocal");
    for f in std::fs::read_dir(dir).unwrap() {
        let file_name = f.unwrap().file_name().into_string().unwrap();
        if !stage_file_re.is_match(&file_name) {
            continue;
        };
        let stage = StageData::new(&file_name, &config.current_version).unwrap();
        println!("{:?}", stage.stage_csv_data.enemies)
        // TODO make this an iterator or something
    }
    todo!()
}
