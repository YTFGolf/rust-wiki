//! Write out the encounters of an enemy.

/*
# Flow
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
