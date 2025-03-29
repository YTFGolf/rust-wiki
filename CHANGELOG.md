# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

This project DOES NOT adhere to Semantic Versioning. Reason being that I pretty
much need to add an enum variant every single update, which is a breaking change
in Rust. Besides enum variants I'll try to do semantic versioning perhaps idk.

## [Unreleased]

### Added

- Cat data.

### Fixed

### Changed

- Images now use the language's file name rather than en.
- Special rule names use an enum rather than having to do manual matching.

### Removed

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

### Removed

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

## [0.1.0] - (2024-09-11)[11db433]

Project started.

[unreleased]: https://github.com/YTFGolf/rust-wiki/compare/v0.5.1...dev
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
