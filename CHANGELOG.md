# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

This project DOES NOT adhere to Semantic Versioning. Reason being that I pretty
much need to add an enum variant every single update, which is a breaking change
in Rust. Besides updates I'll try to do semantic versioning perhaps idk.

From [0.5.4] onwards, this will describe changes for game updates in a separate
subheading.

## [Unreleased]

### Added

### Fixed

### Changed

### Removed

## [0.8.1] - 2025-11-27

Mainly filling in tests, also making various copyedits.

### Fixed

- Jurassic Cat crit talent total cost is now 25 instead of 75.
  ([0b076c1](https://github.com/YTFGolf/rust-wiki/commit/0b076c1))

## [0.8.0] - 2025-11-09

### 15.0

- Establish new combos struct and make existing combo code generic&mdash;15.0
  adds in data at the start of the row which makes this
  non-backwards-compatible.
- Special Rules once again.
- New unknown charagroup.

### 14.7 EN

- Eva -> EVA

### Added

- Fully implemented Cat Talents.
- TalentNames.csv.
- New aliases for ranking dojo.
- Version number as u32 like in unitbuy.
- Stats template versions 1.0 and 1.1.
- Cat script can now use common name as well as ID.
- Cat spirit section.
- Cat AnimationViewer, gallery, reference, CatNav and footer (minus categories).
- Dojo to map script.

### Fixed

- Superfeline upgrade cost.

### Changed

- Moved scripts directory to this repo instead of storing on wiki.
- Combos now panic less annoying-ly, in a way that allows you to edit the code
  to prevent the panic.
- Tweaks to Gauntlets in map script.

## [0.7.4] - 2025-10-01

Due to the sheer amount of work required for talents I decided to call this one
early.

### 14.7

- Freeze Dojo rule.
- New Colosseum rules.
- Filibuster invasion outbreak stage type + a whole lot of custom logic to deal
  with it (main chapters suck).

### Added

- Cat info: intro, appearance, evolution, combos, cost, upgrade cost, catfruit
  evolution.
- Category and `Display` for cat rarity.
- `CacheableVersionData::init_data_with_version`, which allows `init_data` to be
  implemented with access to the version object.
- `Version::location()` as a public method.
- `CatForm::from_repr`
- Let cat form names return an option.
- Function for cat deploy icons.
- Dojo time limit.
- Other `SectionTitle` options.
- Stats template 0.2.
- Talents parser (still incomplete).

### Changed

- Moved evolution items out of private test file.
- `get_cat_descriptions` returns an option.

## [0.7.3] - 2025-09-27

### Added

- Full implementation of `StatsTemplateVersion` including support for `serde`
  and `clap`.
- `use_stats_validation` flag for `cat_info`.
- Cat description template in `cat_info`.
- Gauntlet support for `map_info`.
- `Page` wikitext object.

### Changed

- Stage and map configs now have `#[serde(default)]` on them
- `stage_table` from `map_info` is now in its own module.
- Unhid `cat_info` and `map_info` (warnings are still displayed).

## [0.7.2] - 2025-09-24

### Added

- Cat descriptions.
- Cat rarity string values.
- Dojo score bonuses + DojoRule template.
- Simplified charagroup restriction text for stage infobox.

### Changed

- `VersionLanguage::language()` is now public.

## [0.7.1] - 2025-09-08

### Added

- Ability icons to cat stats.
- `AttackHits::iter`, `UnitLevelRaw::iter`.
- New format for cat stats + template version switcher.
- Cat info config.
- `CatForm` enum to `cat_info`.

### Fixed

- Changed how omni and ld are displayed for cats (see issue #15).
- Cats with max level 1+30 will have stats shown at that level.
- Added gauntlet cooldown check to TreasureAdjustment condition.
- Kamikaze units and units that can't attack now work properly with `cat_info`.

### Changed

- Separated `get_ordinal` and `get_small_ordinal`.
- Logger format.
- Allow `EmptyAnimation` if unit has 1 form (Cheetah, Iron Wall).
- Publicised all modules.

### Removed

- Animations no longer attempt to use TW/KR data.
- Removed Kamikaze ability and replaced with a boolean flag.

## [0.7.0] - 2025-08-24

Probably a bit too big for a single update.

### 14.5/14.6

- Added support for
  [fallback](https://github.com/YTFGolf/bc-data-fallback/tree/main) data. Sets
  `Fallback` as a valid language to achieve this. See issue #16.

### 14.4

- Updated special rules.

### Added

- TW/KR support.
- `AnimationError` enum for cat animations.
- Reset type to parsed stage; now Merciless XP displays top reward as being
  unlimited.
- `MultiLangContainer` to abstract size of container.
- Colosseum map data.
- `stage_table` now properly customises the link part.

### Fixed

- Empty animations return an error.
- Animation tests now use fallback data instead of JP.
- Bases are no longer said to spawn after base hit (issue #11).
- Ms. Sign on Ta-Da! works properly now.

### Changed

- `cooldown` for parsed cat `Form`, replaces old `tba`.
- `SpecialRules::init_data` will initialise a `Default` if the file is not
  found.
- Use proper attack cycle calculation rather than cringe backswing calculation.
- Warn if all `gauntlet` stages are different rather than doing nothing.
- Use Lux Ori's better base barrier intro.

## [0.6.2] - 2025-06-07

### Added

- Cat stats table.

### Fixed

- `get_formatted_float`:
  - When `round(num, precision) = 1.0`, now returns `(num + 1).to_string()`.
  - When `round(num, precision) = 0.0`, now returns `num.to_string()`.

### Changed

- Anim now applies to all types of animation, although only attack is done.
- Stage ID `parse_general_stage_id` returns a result instead of option.

### Removed

- Config language; replaced with `VersionLanguage`.
- `write_formatted_float`: didn't know how to deal with the case where
  `num.prec$f` is `1.0` other than to remove this function entirely.

## [0.6.1] - 2025-05-31

### Added

- `thiserror` dependency.
- Error returns for `StageData`, minus `deserialise_single_enemy`.
  - By extension, a general CSV error enum.
- Error impl for `StageTypeParseError`.
- `unitlevel.csv` support in `Cat` object.
  - Includes calculations for the cat's stats at a certain level.
- `foreswing`, `attack_length` and `total_damage` methods on `AttackHits`.
- `MultiLangVersionContainer` trait for use in anims.
- Base of `cat-info` script.

### Changed

- Moved `get_stages` to `encounters` module as it contains an unwrap.
- Split `wiki_utils` into `text_utils` and `number_utils`.
  - Replace ad-hoc precision and formatting code with proper functions from
    `number_utils`.

## [0.6.0] - 2025-05-25

This update involved a major module reorganisation as per #9.

### Added

- Dev profiles to [Cargo.toml](Cargo.toml) to speed up compilation.

## [0.5.4] - 2025-05-23

### 14.4

- `CatGuideOrder` uses a `u32` instead of a `u16`.
- `AwesomeUnitSpeed` special rule.
- New special rule labels.
- Fix `read_stage_csv` when `line[7]` is a comment.

### Added

- Version can now call a specific language rather than being stuck on the one in
  preferences.
- Improved error handling on unit animations.
- Tests for animations and unitbuy.
- Special rule data and label placeholders to avoid day 0 breakage on new
  updates. Now uses tests to check validity rather than runtime panics.

### Changed

- Use newly un-deprecated `std::env::home_dir` instead of `home` crate.
- Version language no longer needs separate `cur_index` as well as `lang`.
- `static_regex` and `infallible_write` now use `#[track_caller]`.
- Large enemy base images now use 200px as size instead of 250px.

### Removed

- Missing and incomplete/ignored tests.

## [0.5.3] - 2025-05-03

### Added

- Gauntlet script.
- More cat raw and parsed data.
- Tabber and Section wikitext features.
- `stage_info` end-to-end tests.
- `static_regex` function and `regex_handler` module as per #8.
- `infallible_write` as per #8.
- (beta) Event preset for map info.

### Changed

- Split cat data module into raw and parsed.
- Made most internals of the stage info script public.
- Replaced old `format_parser` for stage info template.

## [0.5.2] - 2025-03-30

### Added

- Cat data.
- `Display` impl for map and stage ID.

### Changed

- Images now use the language's file name rather than en.
- Special rule names use an enum rather than having to do manual string
  matching.
- `TemplateParameter` now uses `Cow`s instead of hardcoded types.
- Use `strum` derive methods rather than manual number matching.
- Internal: CSV-reading in `GameMapData`.

## [0.5.1] - 2025-03-14

### Added

- Informal docs section.
- Specification docs section.

### Changed

- Updated dependencies.
- Updated Rust edition to 2024.
- Renamed Map data objects.
- Replaced existing `mapid` functions with `MapID`s.

### Replaced

- `StageData::from_selector` -> `from_file_name` and others replaced by id.
- `Stage::new_current` -> `from_id_current` and others replaced by id.

## [0.5.0] - 2025-03-08

Removed `LegacyStageMeta`.

## [0.4.3] - 2025-03-07

This update's main goal was to deprecate `StageMeta`.

After attempting to make a map data module in [0.4.2], I realised how badly
`StageMeta` needed to be fixed. It was a monolith that did way too many things,
which only really started causing problems when I tried implementing map data
and realised that it was so much easier to reuse `StageMeta` than it was to
properly implement an object for map metadata.

`StageMeta` has not been removed, nor has it been officially marked as
deprecated in the source code, although it has been renamed to
`LegacyStageMeta`.

### Added

- `StageVariantID` &ndash; replaces `StageTypeEnum`.
- `MapID` &ndash; details a map.
- `StageID` &ndash; details a stage; replaces `StageMeta`.
- Command-line option to show all selector values.
- Dedicated modules for parsing strings into map and stage IDs.
- Dedicated modules for parsing IDs into data such as file names.

### Changed

- Split `get_stages` function into `get_stage_files` and the initialiser.
- Slightly changed how selectors work to go along with the update.

## [0.4.2] - 2025-02-26

Updated supported game version to 14.2.

### Added

- Beta map data feature (supports Legend Stages).
- GameMap stores map file number (i.e. the background in Legend Stages).
- Drop items (Ototo materials).

### Changed

- Made `Config` clonable.
- Removed "Internal" stage info folder, reorganised stage info.

## [0.4.1] - 2025-01-25

Alongside these specific updates, updated supported game version to 14.1.

### Added

- Changelog + version tags.
- Parsed map object.
- Catamin cost for Catamin stages.
- Manual EoC zombie outbreak formatter.

### Fixed

- Mount Aku Invasion error in Encounters.
- Four-crown default restriction showing up in restrictions list.
- `UnclearMaybeRaw` drop amount being hardcoded as "1 time".

### Changed

- Modified `StageMeta::from_selector_main` to take 2 args instead of 1.

## [0.4.0] - 2025-01-16

This update overhauled the Config and CLI. This includes significant breaking
changes all over the place.

### Added

- Parser for `SpecialRulesMap.json` and adding rules to stage info.
- Logger.

### Changed

- Significant amount of fixes suggested by `clippy::all`.
- Changed the `Version` substantially: removed the version data's `RefCell` and
  `Pin`ned the contents of its vec.

## [0.3.0] - 2024-12-25

This update finished the Encounters module.

### Added

- Reverse enemy name map.
- Continue stages map.

### Changed

- `StageData::new` return type from `Option` to `Result`.
- Minor fixes.

## [0.2.1] - 2024-12-10

This update was way too big for a patch version. A significant amount of the
entire repository was completely changed or overhauled. The final update was
adding a prototype encounters. This summary is just a basic skim over the actual
update since there's just too much to actually make sense of.

### Added

- User config
- CLI
- Docs
- Prototype encounters
- Support up to 14.0

### Changed

- Move stage info default input to command line rather than stdin.
- Make wiki reader a command-line option instead of being disabled in the binary
  itself.
- Use real numbers for all Main Chapters stages rather than BCU-assigned ones.

## [0.2.0] - 2024-10-15

### Added

- Stage info
- Wiki reader

## [0.1.0] - 2024-09-11

Project started.

[unreleased]: https://github.com/YTFGolf/rust-wiki/compare/v0.8.1...dev
[0.8.1]: https://github.com/YTFGolf/rust-wiki/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/YTFGolf/rust-wiki/compare/v0.7.4...v0.8.0
[0.7.4]: https://github.com/YTFGolf/rust-wiki/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/YTFGolf/rust-wiki/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/YTFGolf/rust-wiki/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/YTFGolf/rust-wiki/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/YTFGolf/rust-wiki/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/YTFGolf/rust-wiki/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/YTFGolf/rust-wiki/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/YTFGolf/rust-wiki/compare/v0.5.4...v0.6.0
[0.5.4]: https://github.com/YTFGolf/rust-wiki/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/YTFGolf/rust-wiki/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/YTFGolf/rust-wiki/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/YTFGolf/rust-wiki/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/YTFGolf/rust-wiki/compare/v0.4.3...v0.5.0
[0.4.3]: https://github.com/YTFGolf/rust-wiki/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/YTFGolf/rust-wiki/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/YTFGolf/rust-wiki/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/YTFGolf/rust-wiki/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/YTFGolf/rust-wiki/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/YTFGolf/rust-wiki/compare/v0.2...v0.2.1
[0.2.0]: https://github.com/YTFGolf/rust-wiki/compare/11db433...v0.2
[0.1.0]: https://github.com/YTFGolf/rust-wiki/commit/11db4333ba632f3967d85350d66ceef4bdd7090b
