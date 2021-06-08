# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!--
## [UNRELEASED]

### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security
### Docs
-->
## [UNRELEASED]

### Changed
- Updated `Token` with `PartialEq` trait implementation.
- Updated `TraceableToken` with `PartialEq` trait implementation.




## v0.2.0

### Added
- Added `LexicalAnalysis` struct.

### Changed
- [BC] Updated `Token` to contain a `String`. `Token` no longer has an `a` lifetime.
- [BC] `TraceableToken` no longer has an `a` lifetime.
- Updated `Token` with `Clone` trait implementation.
- Updated `TraceableToken` with `Clone` trait implementation.

### Removed
- [BC] Removed `LexicalAnalyzer` trait. `LexicalAnalysis` must be used instead.
- [BC] Removed `LexicalAnalyzer` struct. `LexicalAnalysis` must be used instead.




## v0.1.1

### Docs
 - Added `CHANGELOG`.

### Fixed
 - Cargo.lock file is ignored as best practices suggests.




## v0.1.0

### Added
- Added `LexicalAnalyzer` trait.
- Added `LexicalAnalyzer` struct.
- Added `LexSpec` trait.
- Added `TextLocation` struct.
- Added `Token` struct.
- Added `TraceableToken` struct.
