pub fn manual_zoutbreak_replace(buf: &mut String, abs_enemy_id: u32) {
    match abs_enemy_id {
        4 => {
            const FROM: &str = include_str!("outbreaks/those_guys_gen.txt");
            const TO: &str = include_str!("outbreaks/those_guys_fix.txt");
            *buf = buf.replace(FROM, TO);
        }
        286 => {
            const FROM: &str = include_str!("outbreaks/zoge_gen.txt");
            const TO: &str = include_str!("outbreaks/zoge_fix.txt");
            *buf = buf.replace(FROM, TO);
        }
        287 => {
            const FROM: &str = include_str!("outbreaks/znache_gen.txt");
            const TO: &str = include_str!("outbreaks/znache_fix.txt");
            *buf = buf.replace(FROM, TO);
        }
        288 => {
            const FROM: &str = include_str!("outbreaks/zomboe_gen.txt");
            const TO: &str = include_str!("outbreaks/zomboe_fix.txt");
            *buf = buf.replace(FROM, TO);
        }
        294 => {
            const FROM: &str = include_str!("outbreaks/zroco_gen.txt");
            const TO: &str = include_str!("outbreaks/zroco_fix.txt");
            *buf = buf.replace(FROM, TO);
        }
        _ => (),
    }
}

/*
How these are generated:
- Generate the actual encounters, paste eoc outbreaks into _gen.txt.
- Swap the chapter num and stage num, 0 pad ("Stage -\d-") ("Stage 1-4" ->
  "Stage -04-1").
- "^(\*Stage -\d+)-1(.*)$\n\1-2\2\n\1-3\2$"
- Remove chapter number and fix all of the "Stage 0"s.
*/
