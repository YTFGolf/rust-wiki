# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Fixed

### Changed

### Removed

## [0.4.0] - 2025-01-16

## [0.3.0] - 2024-12-25

## [0.2.1] - 2024-12-10

This update was way too big for a minor version. A significant amount of the
entire repository was completely changed or overhauled. The final update was
adding a prototype encounters. This summary is just a basic skim over the actual
update since there's just too much to actually make sense of.

### Added

- User config
- CLI
- Docs
- Prototype encounters
- Support up to 14.0

### Fixed

- Home directory expansion errors (475b136b46bc18544a3b627494718d40b392c335).

### Changed

- Move stage info default input to command line rather than stdin.
- Make wiki reader a command-line option instead of being disabled in the binary
  itself.
- Use real numbers for all Main Chapters stages rather than BCU-assigned ones.

## [0.2.0] - 2024-10-15

### Added

- Stage info
- Wiki reader
