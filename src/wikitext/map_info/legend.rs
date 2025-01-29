use crate::{config::Config, data::map::parsed::map::MapData};

pub fn get_legend_map(map: &MapData, _config: &Config) -> String {
    println!("{map:#?}");
    todo!()
}
