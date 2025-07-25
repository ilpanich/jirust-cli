# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [Unreleased]

## [0.16.1] 2025-07-16
### Fixed
- Uniform project_key short param name management (`-k`) across different commands

### Changed
- Updated dependencies

## [0.16.0] 2025-07-15
### Added
- Use tokio multi-threaded runtime for better performance (manage version-related tickets operations concurrently on version creation) [Only Linux & Windows, nto WASM]

### Changed
- Updated dependencies

## [0.15.2] 2025-06-27
### Refactored
- Applied the suggested cargo clippy fixes

### Changed
- Updated dependencies

## [0.15.1] 2025-06-22
### Changed
- Improved code documentation
- Fixed typo in README.md

## [0.15.0] 2025-06-21
### Added
- Retrieve version-related workitems
- Search for issues using JQL
- General code, documentation and tests improvements

### Changed
- Updated dependencies

## [0.14.6] 2025-05-24
### Refactor
- Removed some unnecessary clone replacing it with references

### Changed
- Updated dependencies

## [0.14.5] 2025-04-28
### Fixed
- Issue with the `webpki-roots` license type change

### Changed
- Updated dependencies

## [0.14.4] 2025-04-28
### Changed
- Updated dependencies

## [0.14.3] 2025-04-18
### Changed
- Updated dependencies

## [0.14.2] 2025-04-09
### Fixed
- Removed a debug print forgotten in the previous release

## [0.14.1] 2025-04-09
### Added
- Added support for WASM target
- Added create Jira Project command

### Fixed
- Fixed a bug in the github actions that causes builds to fail

## [0.14.0] 2025-04-09
### Changed
- Updated dependencies
- Refactor of the whole library code

## [0.13.4] 2025-03-31
### Changed
- Made the config_file_path optional in the process_command function
- Further refactoring of the functions included in the library
- Updated dependencies

## [0.13.3] 2025-03-25
### Fixed
- Fixed a bug in the version creation command that causes the automatic resolution to fail

### Changed
- Updated dependencies (includes a potential security issue fix)

## [0.13.2] 2025-03-14
### Changed
- Updated dependencies (includes a potential security issue fix)

## [0.13.1] 2025-03-02
### Fixed
- Fixed tests failing after having print logic moved to the bin

## [0.13.0] 2025-03-02
### Changed
- Improved overall error handling
- Moved printing logic from the lib to the bin

## [0.12.0] 2025-02-26
### Fixed
- Issue with the missing TLS feature in jira_v3_openapi _reqwest_ dependency
### Changed
- Updated dependencies
- Moved to Rust 2024 edition


## [0.11.3] 2025-01-10
### Fixed
- README.md not correctly formatted

## [0.11.2] 2025-01-10
### Changed
- Updated dependencies
- Updated Jira v3 OpenAPI autogenerated code
- Removed uselss feature flags from dependencies
- Updated README.md

### Added
- Added feature flags to jira_v3_openapi library to optimize the build and reduce the binary size

## [0.11.1] 2025-01-05
### Changed
- Dependencies updates
- Dependencies feature flags optimizations

## [0.11.0] 2025-01-03
### Fixed
- Fixed the format used in jira_doc_std_field macro to allow to use it properly
- Fixed some wrong documentation

### Added
- Manage multiple possible resolution (or final) transitions on automatic resolution in Jira version creation
- Added a new command to link Jira issues

### Changed
- Updated dependencies
- Updated Security Policy

## [0.10.0] 2024-11-21
### Added
- Macro jira_doc_std_field to generate the standard jira doc type field structure

### Changed
- Updated dependencies

## [0.9.8] 2024-11-12
### Changed
- Updated dependencies
- Added some optimizations flags to the build

## [0.9.7] 2024-10-19
### Fixed
- Jira issue create command now is working properly

## [0.9.6] 2024-10-06
### Changed
- Changed single version table display format to best fit small screens
- Minor fixes and improvements
- Updated dependencies

## [0.9.5] 2024-10-05
### Fixed
- Refactoring, cleanup and enhancements as suggested by rust-clippy

### Changed
- Updated dependencies

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
