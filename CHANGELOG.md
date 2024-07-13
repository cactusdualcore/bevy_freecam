# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-07-13

## Changed

- Improved vertical FOV clamping.

## Fixed

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
