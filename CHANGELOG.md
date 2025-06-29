# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.2] - 2025-06-29

### Changed

- Fixed incorrect generics scoping on items with syn::Signature

## [0.3.1] - 2025-06-11

### Added

- Added missing DirectMut impl for FullDefault

### Removed

- Removed unused type FullGenericScope.

## [0.3.0] - 2025-06-11

### Added

- Mut visits

## [0.2.0] - 2025-06-11

### Added

- Generic scoping nodes (GenericsEnter, GenericsExit) to syn node set

### Changed

- syn::direct::Full now has the Visit parameter on the trait instead of each fn
- syn::direct fns renamed to begin with direct_* instead of visit_*
- syn::direct freestanding fns now take the Director by mut instead of by value

## [0.1.0] - 2025-06-10

### Added

- Initial release