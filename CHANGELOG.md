# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2024-01-20
### Changed
- Updated `enum-primitive-derive` to version `0.3`
- Updated `libflate` to version `0.2`

## [0.4.0] - 2023-04-22
### Changed
- Updated `nom` to version `7`. This is a breaking change since it changes the definition of the `matfile::Error` type

## [0.3.1] - 2021-06-26
### Changed
- Made the `ndarray` module public
- Adjusted docs.rs configuration to include all features

## [0.3.0] - 2021-06-26
### Changed
- Turned `matfile-ndarray` from a separate crate into a feature of `matfile`. Activate it with the Cargo `ndarray` feature flag
- The `ndarray` conversions now use the standard library's `TryInto` trait
- Updated `enum-primitive-derive` to version `0.2`
- Updated `nom` to version `6`. This is a breaking change since it changes the definition of the `matfile::Error` type
- Updated `ndarray` to version `0.15`
- Updated `num-complex` to version `0.4`

## [0.2.1] - 2020-05-17
### Changed
- Updated libflate to version 1.0

## [0.2.0] - 2019-04-05
### Changed
- Array size changed from `Vec<i32>` to `Vec<usize>`

## [0.1.0] - 2019-04-04
### Added
- Loading of numeric arrays

[0.2.1]: https://github.com/dthul/matfile/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/dthul/matfile/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/dthul/matfile/releases/tag/0.1.0