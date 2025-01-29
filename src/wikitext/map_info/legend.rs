use crate::{config::Config, data::map::parsed::map::{MapData, ResetType}};

/// Ensure that expected stage invariants are met.
fn test_invariants(map: &MapData) {
    // assert_eq!(map.crown_data, None);
    assert_eq!(map.reset_type, ResetType::None);
    assert_eq!(map.max_clears, None);
    assert_eq!(map.cooldown, None);
    // assert_eq!(map.star_mask, None);
    assert_eq!(map.hidden_upon_clear, false);
    // assert_eq!(map.restrictions, None);
    assert_eq!(map.ex_option_map, None);
    assert_eq!(map.special_rule, None);
}

pub fn get_legend_map(map: &MapData, _config: &Config) -> String {
    test_invariants(map);
    println!("{map:#?}");

    todo!()
}
