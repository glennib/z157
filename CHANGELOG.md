# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.2](https://github.com/glennib/z157/compare/v1.0.1...v1.0.2) - 2025-04-25

### Other

- Let expected output be doctest

## [1.0.1](https://github.com/glennib/z157/compare/v1.0.0...v1.0.1) - 2025-02-22

### Other

- grammar
- Upgrade winnow to 0.7.3
- cargo update
- Upgrade to 2024 edition

## [1.0.0](https://github.com/glennib/z157/compare/v0.6.0...v1.0.0) - 2025-01-06

### Other

- No changes.

## [0.6.0](https://github.com/glennib/z157/compare/v0.5.0...v0.6.0) - 2025-01-06

### Added

- [**breaking**] Remove Copy from Field to increase future compatibility
- [**breaking**] make Field methods take self by ref to increase future compatibility

## [0.5.0](https://github.com/glennib/z157/compare/v0.4.0...v0.5.0) - 2025-01-06

### Fixed

- [**breaking**] Parse ! as negation instead of -, as per the spec

### Other

- *(fuzz)* del logs does not need target
- *(fuzz)* make just commands generic over fuzz target
- *(fuzz)* Rename fuzz_target_1 to parse_walk

## [0.4.0](https://github.com/glennib/z157/compare/v0.3.2...v0.4.0) - 2024-12-28

### Added

- Add leaves iterator method to Tree
- [**breaking**] Use impl iterator instead of concrete types

### Fixed

- [**breaking**] Fix problem where Tree::walk returns empty root node

### Other

- Add fuzz logs to gitignore
- Add example program
- Add tests for field walk and children
- Add root node test
- Add fuzzing
- Move inputs to separate directory

## [0.3.2](https://github.com/glennib/z157/compare/v0.3.1...v0.3.2) - 2024-12-27

### Added

- Add has_children

### Other

- Add bench workflow
- configure bench
- Reordering of parser fields improves performance of small inputs.
- Increase sample size and sample time for benchmarking
- Add benchmarks
- Format, test, lint

## [0.3.1](https://github.com/glennib/z157/compare/v0.3.0...v0.3.1) - 2024-12-21

### Other

- Fix docs

## [0.3.0](https://github.com/glennib/z157/compare/v0.2.1...v0.3.0) - 2024-12-21

### Added

- [**breaking**] Return unparsable string

### Other

- Complete example
- Improve docs and README
- [**breaking**] Change the way to construct a Tree
- Reflect struct name changes in docs
- [**breaking**] Rename Params to Tree and Param to Field
- Rename params module to tree
- Rename maybe_slice module to str_range

## [0.2.1](https://github.com/glennib/z157/compare/v0.2.0...v0.2.1) - 2024-12-21

### Other

- Replace MaybeSlice with StrRange

## [0.2.0](https://github.com/glennib/z157/compare/v0.1.2...v0.2.0) - 2024-12-20

### Added

- [**breaking**] Change Params to be no-copy (no realloc)
- Add the MaybeSlice type

### Other

- Configure release-plz

## [0.1.2](https://github.com/glennib/z157/compare/v0.1.1...v0.1.2) - 2024-12-20

### Other

- typo
- Add badges

## [0.1.1](https://github.com/glennib/z157/compare/v0.1.0...v0.1.1) - 2024-12-19

### Other

- Add repository link
- More info in README
- Add example to README

## [0.1.0](https://github.com/glennib/z157/compare/v0.0.2...v0.1.0) - 2024-12-19

### Other

- Add docs
- Add top
- Param methods take self by val
- Add children iterator
- Implement Params
- Add release-plz action
