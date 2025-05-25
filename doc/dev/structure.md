General organisation of the project.

## Code

- `mod.rs`, and only `mod.rs`, is for organising.  The `mod` keyword should only appear in files named `mod.rs`.
  - This does not apply to any modules named `tests`.
  - `lib.rs` is an exception for obvious reasons.
  - Exceptions can be made for extremely small, private, inline modules, such as [this one](https://github.com/YTFGolf/rust-wiki/blob/6fe6a3e4ce59fb25f83fb4bf52933750851d235e/src/game_data/map/cached/special_rules.rs#L16) where the only reason it's not its own file is sheer laziness.
  - Use the regex `\bmod\b(?! tests)`, and filter it by only including Rust files not named `mod.rs` or `lib.rs`.
  - Use a pattern like `<module name>_util.rs` if you can't think of a real organisation method.

## Docs

- `dev` &ndash; detail/notes related to the development process
- `spec` &ndash; formal-ish detail about the inputs and outputs of this project (e.g. game data, wiki data, output formats)
- `informal` &ndash; ramblings and essays in place of a properly defined structure and semantics
