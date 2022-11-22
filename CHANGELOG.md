# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

..

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

[unreleased]: https://github.com/mendelt/reval/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/mendelt/reval/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mendelt/reval/releases/tag/v0.1.0
