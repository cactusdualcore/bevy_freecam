# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-07-13

### Added

- Field `force_y_up_direction` on `DebugCameraOptions` to toggle Y direction as
  up direction enforcement.
- Zooming functionality to the mouse wheel.
- Fast movement functionality, with a new keybind (`fast_movement`) and an
  option (`InputOptions::sticky_fast_movement`) whether the key should toggle
  or enable this fast movement.
- CI to test and lint this crate.

### Changed

- Renamed `zoom_speed` and `zoom_distance_range` to `zoom_intensity` and `zoom_range`, respectively.
- Improved vertical FOV clamping.
- Force up direction to Y direction is now immediately enforced without time
  gap of inconsistet state.

### Fixed

- Wrong behavior when looking back.
- `DebugCameraPlugin` was disabled by default in basic example.

## [0.1.0] - 2024-07-06

### Added

- a README file with planned and implemented features and information for users
  and potential contributors.
- This CHANGELOG file as a record of all changes.
- DebugCamera component to mark cameras for this library.
- DebugCameraOptions resource as a global configuration for all DebugCameras.
- Custom keybinding support for DebugCameraOptions.
- A default keybinding with description in the README.
- Systems for processing input and driving camera movement.
- A plugin registering types and implemented functionality with bevy.
- An example documenting basic usage.

[0.1.0]: https://github.com/cactusdualcore/bevy_freecam/releases/tag/v0.1.0
