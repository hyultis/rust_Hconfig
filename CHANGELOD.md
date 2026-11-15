# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - 2026-11-15

### Added
- `HConfig::name_get()` returns the config name.
- The README is the crate documentation, its example is compiled by `cargo test`.
- Documentation for every public item.

### Fixed
- `value_get_mut` (and every write through a path) no longer panics on an out of bounds array index (e.g. `array/5/cmd`).
- `value_set`, `value_get_or_set` and `value_get_mut` now work on an existing array index (they previously did nothing).
- `HConfigManager::create::<T>()` now really uses the wrapper `T` (it always used `WrapperJson`).
- `Utils::jsonIntoHashMap` no longer panics on non string values : they are converted to their json text.
- An unreadable config file (e.g. invalid UTF-8) now makes `create` / `file_load` return `Errors::ConfigCannotLoadFile` : it was silently loaded as `{}`, then overwritten on the next save.
- `file_save` writes the temporary file in one go, syncs it to disk before the rename, and removes it on error.

### Changed (breaking)
- Path semantics :
  - reading (`value_get`, `value_get_mut`, `value_remove`) never creates anything; `value_get_mut` returns `None` and `value_remove` returns `false` if the path does not exist (they previously created the missing object keys);
  - writing (`value_set`, `value_get_or_set`) forces the path : missing keys are created, an out of bounds array index grows the array (filled with `null`), a scalar on the path is replaced by an object (the write was previously ignored);
  - empty segments are ignored (`a//b`, `a/`), an empty path does nothing (it previously created a `""` key).
- The library name is now `hconfig` (`use hconfig::...`, `hconfig::tinyjson`); the crates.io package name stays `Hconfig`.
- `HConfigManager::get(&str)` now returns `Option<Arc<RwLock<HConfig>>>` (was a `dashmap` `RefMut`) : use `read()` / `write()` on it. No external type is exposed by the API anymore.
- `HConfigManager::create` on an already loaded config reloads it in place : handles already returned by `get` see the new content.
- `HConfig::root_get_mut` now returns `&mut JsonValue`.
- Paths are `&str` everywhere : `IOwrapper` methods, `HConfig::value_get_mut`, `HConfig::new(&str, &str)`, `IOwrapper::init(&str, &str)`.
- `value_get_mut` no longer has explicit lifetimes; `file_ext` returns `&'static str`; `file_path` returns `&str`.
- `Display` of a config now outputs json (it was a Rust debug output).
- Removed the never produced `Errors::ConfigNotSet` variant; `Errors::AnyError` is documented for custom `IOwrapper` implementations.
- Removed the unused private field `HConfig::constructed_fullpath` (use `file_path()`) and the crate-wide `allow(dead_code)`.

### Dependencies
- Removed `dashmap` and `arc-swap` (replaced by `std::sync::RwLock`).
- Updated all locked dependencies (`cargo update`).

### Migration from 2.x
- The library is now imported as `hconfig` (the crates.io package keeps the name `Hconfig`) : replace `use Hconfig::...` by `use hconfig::...`.
- `get()` returns a shared handle : lock it before use.
  ```rust,ignore
  // 2.x
  let mut config = HConfigManager::singleton().get("example").unwrap();
  // 3.0
  let config = HConfigManager::singleton().get("example").unwrap();
  let mut config = config.write().unwrap();
  ```
- Paths are `&str` : `value_get_mut("a/b")` instead of `value_get_mut(String)` ; custom `IOwrapper` implementations take `&str` in `init` and `value_*`, and return `&str` from `file_path` and `&'static str` from `file_ext`.
- `value_get_mut` / `value_remove` no longer create missing keys : use `value_set` or `value_get_or_set` to create a value.
- An unreadable config file is now an error at `create` / `file_load` instead of an empty config.
- `Errors::ConfigNotSet` no longer exists.

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