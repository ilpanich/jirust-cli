# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.
## [Unreleased]

## [0.9.4] 2024-09-27
### Fixed
- Using the transition name instead of the transition id in the version creation command when resolve issues flag is set

### Changed
- Updated dependencies

## [0.9.3] 2024-09-21
### Fixed
- Fixed a wrong code merge that causes the build to fail

### Changed
- Centered the logo in the README.md

## [0.9.2] 2024-09-21
### Changed
- Minor changes to the README.md
- Changed the logo
- Added dependabot configuration

## [0.9.1] 2024-09-21
### Improved
- Improved jira issue automatic resolution on version creation

## [0.9.0] 2024-09-20
### Added
- Added Jira issue management commands

## [0.8.4] 2024-08-21
### Added
- Added automatic version description extraction from a "keepachangelog" markdown formatted changelog file

## [0.8.3] 2024-08-19
### Removed
- Removed macOS from github actions (I can find no way to make it work, and I don't want to spend more time on it)

### Changed
- Updated dependencies
- Due to the macOS build issue related tests, you might have lost some changes since version "0.8.0"; Please look at the complete changelog at [CHANGELOG.md](https://github.com/ilpanich/jirust-cli/blob/main/CHANGELOG.md)

## [0.8.2] 2024-08-19
### Changed
- Improved github actions (build also for macOS)
- Updated dependencies

## [0.8.1] 2024-08-19
### Changed
- Refactored CLI args management (better help, moved some common args, better syntax)
- Improved docs and tests according to the new CLI args management
- Updated dependencies

## [0.8.0] 2024-08-18
### Added
- Added Jira projects-related commands

### Changed
- Improved docs and tests
- Updated dependencies
- Updated README.md
- Updated github actions

## [0.7.4] 2024-08-14
### Changed
- github actions improvements
- Added some useful badges to README.md
- Updated dependencies

## [0.7.3] 2024-08-14
### Fixed
- github actions for CI/CD issues

## [0.7.2] 2024-08-14
### Changed
- Updated README.md
- Improved docs and tests
- Added github actions for CI/CD

## [0.7.1] 2024-08-13
### Changed
- Updated README.md
- Improved docs and tests

## [0.7.0] 2024-08-13
### Changed
- Refactored to split the logic between lib and bin


## [0.5.0] 2024-08-13
First release
