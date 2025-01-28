use crate::{config::Config, data::map::parsed::map::MapData};

pub fn get_map_info(map: &MapData, _config: &Config) -> String {
    println!("{map:#?}");
    todo!()
}
