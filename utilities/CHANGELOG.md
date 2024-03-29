# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Bug Fixes

- Gpx time displayed incorrectly (#124)

### Features

- First stab at the Fitshow utility
- Fitshow handles GPX too
- Add TCX support
- Pretty print
- Create linter for PR titles

### Miscellaneous Tasks

- Changelog update
- Main README update
- Rename action
- Folder rename
- README update
- Fitshow rename to fitview
- Clean up lints
- Fix spelling mistakes
- Dependencies version update
- Dependencies update

### Refactor

- More commonalities and simplification
- Remove TimeStamp (#113)
- Lint fix (#116)
- Switch to Clap v4 (#122)

### Build

- Update fitparser requirement in /fit2json
- Update env_logger requirement in /fit2json (#47)

## [0.4.0] - 2022-02-21

### Bug Fixes

- Day and Month were off by 1

### Features

- Start on fitrename
- Fitrename FIT and GPX done
- Fitrename TCX support
- Add utilities crate - not yet working

### Miscellaneous Tasks

- Changelog update
- Changelog update
- Changelog

### Refactor

- Move commonalities to shared package
- Commonalities to shared package Part 2
- Separate out commonalities Part 3
- Move commonalities Part 4

## [0.3.6] - 2022-01-03

### Chore

- Repo maintenance

### Miscellaneous Tasks

- [**breaking**] Updated to Clap v3

### T2C

- CSV export of Trackpoints
- Minor updates

## [0.1.1] - 2021-10-23

### F2C

- Crate version updates

### G2C

- Minor documentation update

## [0.3.3] - 2021-10-11

### F2C

- Refactor and clean up
- Documentation and Params
- Rework logging
- Additional logging
- Better logging

### F2J

- Documentation update
- Improved logging

### G2C

- Cleaned up code based on Clippy feedback
- Document and rework logging
- Error output
- Improved logging

## [0.3.2] - 2021-10-09

### F2C

- Big refactor

### F2c

- Cosmetics

### G2c

- All Activities summary export

## [0.3.1] - 2021-10-09

### G2c

- Add Activity duration

## [0.3.0] - 2021-10-09

### G2c

- FIrst track export working
- Parse multiple files and output session & track summary
- Export waypoint details

## [0.2.7] - 2021-10-08

### F2c

- Paths and debug info
- Set defaults to None instead of Some
- Minor cleanup of parsers use section
- Move HrZone parser to a From trait
- Refactor HrZone parsing a little
- Change how time is displayed
- Add session file destination on -o parameter

### G2c

- Parse header
- Export session information to JSON
- Set defaults to None instead of Some
- More types, more robust parsing
- More detail in debug
- Add additional types, add some traits
- (re)define all the types
- Added waypoints to tracks
- Refactor Types
- Move Fix unit tests into a submodule
- Some trait updates
- Move metdata info to TOML

<!-- generated by git-cliff -->
