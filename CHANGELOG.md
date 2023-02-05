# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## Changed
- Make ruleset `Builder` fail on rules or functions with duplicate names
- `add_function` on `UserFunctions` and ruleset `Builder` now takes non-boxed `UserFunction`s

## Removed
- Removed `add_functions` method from `Userfunctions` and ruleset `Builder` because it does not make much sense
- Removed `get` method from `UserFunctions` public api
- Removed `BoxedFunction` type from prelude and public api

## [0.5.2] - 2022-01-20

### Added
- Utility function for creating `Expr::Reference`
- `From<usize>` implementation for `Value`
- Allow simple `String` or `usize` index parameter for `idx` besides expressions

## [0.5.1] - 2022-01-18

### Added
- Documentation for rules writers
- Into implementation from `Value` to `Expr`

### Changed
- Make add, sub, mul, div, and and or in the json rules language take more than 2 operands  Left associatively combine them into their respective expressions

## [0.5.0] - 2022-01-04

### Added
- if expression.

### Changed
- `FunctionError` type is replaced with `anyhow::Error`

### Removed
- Methods for adding rules and functions to a `RuleSet` directly.


## [0.5.0] - 2022-01-04

Was yanked and replaced by [0.5.0]


## [0.4.4] - 2022-12-08

### Added
- Add `BoxedFunction` to the prelude
- `Rule::name` method

### Fixed
- Equality comparison was not working

### Removed
- `Value::new` constructor


## [0.4.3] - 2022-12-05

### Added
- Constuctor expressions for Map and Vec


## [0.4.2] - 2022-12-01

### Added
- Add conversions for a bunch of numerical types to an from `Value`


## [0.4.1] - 2022-11-30

### Fixed
- Fix adding multiple user-functions


## [0.4.0] - 2022-11-30

### Added
- Methods for adding multiple rules and user-functions to a ruleset at once
- Add conversions to/from Value::Map and Value::Vec as convenience

### Changed
- Add name method to `UserFunction` trait instead of passing the name into the builder
- Rule and RuleSet now return `Outcome` instances after evaluation that contain some metadata about the evaluation
- `RuleSet::evaluate` does not halt rule evaluation when one rule returns `Err`
- Bumped minimum supported Rust version to 1.62.1


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


[unreleased]: https://github.com/mendelt/reval/compare/v0.5.2...HEAD
[0.5.2]: https://github.com/mendelt/reval/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/mendelt/reval/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/mendelt/reval/compare/v0.4.4...v0.5.0
[0.4.5]: https://github.com/mendelt/reval/compare/v0.4.4...v0.4.5
[0.4.4]: https://github.com/mendelt/reval/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/mendelt/reval/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/mendelt/reval/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/mendelt/reval/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/mendelt/reval/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/mendelt/reval/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/mendelt/reval/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/mendelt/reval/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/mendelt/reval/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/mendelt/reval/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mendelt/reval/releases/tag/v0.1.0
