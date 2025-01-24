pub fn manual_zoutbreak_replace(buf: &mut String, abs_enemy_id: u32) {
    match abs_enemy_id {
        4 => {
            const FROM: &str = include_str!("outbreaks/those_guys_gen.txt");
            const TO: &str = include_str!("outbreaks/those_guys_fix.txt");
            *buf = buf.replace(FROM, TO);
        }
        _ => (),
    }
}
// Old attempt at analysing outbreaks but then just decided it's easier to
// manage manually.
/*
fn analyse_zoutbreak_mags<'a>(
    section_map: (SectionRef, Vec<&'a StageData<'a>>),
    abs_enemy_id: u32,
) -> Option<[u32; 3]> {
    let should_be_truncated = section_map.0 == SectionRef::EoCOutbreak && section_map.1.len() > 90;
    if !should_be_truncated {
        return None;
    }

    // This is not the best way to do this
    // This doesn't do any sort of filtering, doesn't group stages, etc. etc.
    let mut mags = [0, 0, 0];
    let f = &section_map.1;
    for stage in f {
        let map_id = stage.meta.map_num as usize;
        for enemy in stage
            .stage_csv_data
            .enemies
            .iter()
            .filter(|e| e.num == abs_enemy_id)
        {
            if mags[map_id] == 0 {
                mags[map_id] = enemy.magnification.unwrap();
            } else if mags[map_id] != enemy.magnification.unwrap() {
                return None;
            }
        }
    }

    if mags.contains(&0) {
        return None;
    }

    Some(mags)
}
*/

/*
How these were generated:
- Generate the actual encounters, paste eoc outbreaks into _gen.txt.
- Swap the chapter num and stage num, 0 pad ("Stage -\d-") ("Stage 1-4" ->
  "Stage -04-1").
- "^(\*Stage -\d+)-1(.*)$\n\1-2\2\n\1-3\2$"
- Remove chapter number and fix all of the "Stage 0"s.
*/
