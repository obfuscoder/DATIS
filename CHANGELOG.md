# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [1.0.0-alpha.2] - 2019-10-31

### Added
- Support for Windows' TTS as an additional TTS provider #24
- Carrier ATIS (reports altimeter, BRC and Case variant to be used)

### Fixed
- Derive a visibility value from cloud and fog weather settings (instead of using the visibility mission setting that never changes)

## [1.0.0-alpha.1] - 2019-10-25

Moving to `1.0.0` as encouraged by semantic versioning.

### Added
- Support for AWS Polly as an additional TTS provider (implemented by @16AGR-Durham)

### Changed
- Upgrade to the SRS 1.7.0.0 network changes. DATIS now acts as a 1.7.0.0 SRS client.
- The internals of DATIS are no based on async Rust. With that, DATIS does not create two threads for each ATIS station anymore (max. number of threads is now the number of cores available to the system).

## [0.10.0-beta.1] - 2019-10-18
### Changed
- Upgrade to the SRS 1.7.0.0 network changes. DATIS now acts as a 1.7.0.0 SRS client.

## [0.9.2] - 2019-10-28
### Fixed
- In some cases stations stopped transmitting every ~0.5 secs should be fixed
- Restore consistent ~3sec pause between reports

## [0.9.1] - 2019-10-14
### Fixed
- Do not SPAM logs when connection to SRS is lost #18

### Changed
- Automatically try to re-connect to SRS if connection to SRS got lost or if DATIS is started before SRS

## [0.9.0] - 2019-08-13
### Added
- Option to enable debug logging (useful when investigating issues; logs into `Saved Games\DCS\Logs\DATIS.log`)
- Option to change the SRS Server port

### Fixed
- Fix rounding issue in parsing of certain frequencies #15

## [0.8.0] - 2019-04-23
### Changed
- DATIS now acts as an SRS version 1.6.0.0 client (and thus doesn't work with <1.6 servers anymore)

## [0.7.1] - 2019-03-03
### Fixed
- Fix missing `Terrain` global when starting DCS server with `--norender`

### Changed
- Use different log levels for DCS Lua hook

## [0.7.0] - 2019-01-10
### Changed
- Added radio information to the initial SRS sync message #10
- All ATIS reports are now exported into the DCS saved games directory into `Logs\atis-reports.json`

### Added
- Log a warning if there are no ATIS stations found

### Fixed
- Fix parsing of airfield names that contain a space in their name
- Fix wind speed unit

## [0.6.0] - 2018-12-04
### Changed
- Reduced logging output (to started, stopped and error messages)

## [0.5.0] - 2018-11-28
### Added
- Added QFE in inHg and hPa to the ATIS report remarks

### Fixed
- Fixed reading runways with a _L_ or _R_ suffix

## [0.4.4] - 2018-11-21
### Fixed
- Properly rotate and normalize the DCS reported wind direction

## [0.4.3] - 2018-11-11
### Fixed
- Fixed active runway calculation

## [0.4.2] - 2018-11-07
### Fixed
- Pressure and temperature readings should now also work when the server does not hit the "Briefing" button
- SRS broadcasts should now always be stopped when the mission is stopped

## [0.4.1] - 2018-11-04
### Fixed
- Properly handle and report Gcloud TTS API errors (the previous error message was not useful at all, see #8)

## [0.4.0] - 2018-11-03
### Added
- Added option to setup ATIS stations by [adding static units](https://github.com/rkusa/DATIS#mission-setup) with a specific naming scheme to the mission
- Possibility to use a different voice per station

### Fixed
- TTS should now properly read ZERO and not "o"
- Pressure and temperature readings fixed for multiplayer servers (was only working correctly in SP)

### Changed
- Added "Cloud conditions" in front of the cloud report
- Added longer breaks between the different report parts
- Skip the "DECIMAL" when calling out the altimeter setting

## [0.3.0] - 2018-10-24
### Changed
- Extracted Google Cloud Access Key into "DCS ATIS" option specials menu

## [0.2.1] - 2018-10-12
### Fixed
- reverted most phonetic number replacements (TTL handles normal numbers fine)

## [0.2.0] - 2018-10-12
### Added
- visibility report
- clouds report

### Fixed
- QNH properly read at ground level
