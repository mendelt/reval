# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Methods for adding multiple rules and user-functions to a ruleset at once
- Add conversions to/from Value::Map and Value::Vec as convenience

### Changed

- Add name method to `UserFunction` trait instead of passing the name into the builder

## [0.3.0] - 2022-11-29

### Added

- Allow parsing individual `Expr` instances from json
- Make `Expr::evaluate` public
- Add `cacheable` method to `UserFunction` trait

### Changed

- Make `parse::parse` a method on `Expr`
- Make `and` and `or` evaluation lazy


## [0.2.0] - 2022-11-27

### Added

- Add neg function to invert sign of numeric values
- Add cast functions for numeric values

### Changed

- Remove implicit casts in numeric operations


## [0.1.3] - 2022-11-26

### Added

- User-function error type
- TryFrom<> implementations to convert Value into primitive types
- User Function example and doctest
- Parse function call for json rules
- Some better documentation


## [0.1.2] - 2022-11-22

### Changed

- Added Ruleset and UserFunction to prelude
- Removed Error types from prelude
- Moved `parse_json` into the `Rule` type


## [0.1.1] - 2022-11-22

### Added 

- This changelog
- Ruleset builder
- Prelude for easy import of `reval` types

### Changed

- Make `evaluate` method take any parameter that implements `serde::Serialize`


## [0.1.0] - 2022-11-21

### Added

- Initial release
- Parse rules from json
- Experimental support for parsing rules with nom
- Implement `Value` type that represents input and output for rules
- Add serialize support for `Value` so any type that implements serde `Serialize` can be used as input
- Implement `Expr` type as the AST for rules
- Implement a `RuleSet` as an entry point for executing a set of rules on a piece of data 
- async `UserFunction` support

[unreleased]: https://github.com/mendelt/reval/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/mendelt/reval/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/mendelt/reval/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/mendelt/reval/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/mendelt/reval/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/mendelt/reval/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mendelt/reval/releases/tag/v0.1.0
