# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

This project DOES NOT adhere to Semantic Versioning. Reason being that I pretty
much need to add an enum variant every single update, which is a breaking change
in Rust. Besides enum variants I'll try to do semantic versioning perhaps idk.

## [Unreleased]

The versions here were a little rushed because doing part of map info made me
realise how badly I need to fix `StageMeta`.

### Added

### Fixed

### Changed

- Split `get_stages` function into `get_stage_files` and the initialiser.

### Removed

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

[unreleased]: https://github.com/YTFGolf/rust-wiki/compare/v0.4.2...HEAD
[0.4.2]: https://github.com/YTFGolf/rust-wiki/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/YTFGolf/rust-wiki/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/YTFGolf/rust-wiki/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/YTFGolf/rust-wiki/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/YTFGolf/rust-wiki/compare/v0.2...v0.2.1
[0.2.0]: https://github.com/YTFGolf/rust-wiki/compare/11db433...v0.2
[0.1.0]: https://github.com/YTFGolf/rust-wiki/commit/11db4333ba632f3967d85350d66ceef4bdd7090b
