# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - Unreleased

### Fixed
- `value_get_mut` (and every write through a path) no longer panics on an out of bounds array index (e.g. `array/5/cmd`).
- `value_set`, `value_get_or_set` and `value_get_mut` now work on an existing array index (they previously did nothing).

### Changed
- `value_set` and `value_get_or_set` on an out of bounds array index now grow the array up to that index, filling the gap with `null`.
- `value_get_mut` returns `None` on an out of bounds array index.

### Notes
- Development of this version is assisted by an agentic AI (Claude Code), working under a documented workflow (plans, decisions, reviews) with every change reviewed by the maintainer.

---

## [2.0.2] - 2025-12-13

### Added
- Added `value_remove` to delete a value.

### Changed
- Updated dependency.

---

## [2.0.1] - [2025-09-29]

### Changed
- fix `IOwrapper::init`.

---

## [2.0.0] - [2025-08-15]

### Changed
- revamped all classes, HConfigManager now have a create::<Wrapper>() function to initialize a configuration with a "Wrapper", added default WrapperJson.
- migrated from `serde_json` to `tinyjson`.

---

## [1.1.0] - [2024-08-17]

### Changed
- migrated from `rusty_json` to `serde_json`.
- tests refactoring.

---

## [1.0.0] - [2024-07-24]

### Added
- Initial release.