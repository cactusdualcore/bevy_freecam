# Bevy Debug Camera

This repository contains a custom debug camera system for applications built
using the Bevy game engine. This system provides flexible and configurable
camera controls, including movement, rotation, and zooming, which are essential
for debugging and developing 3D applications.

## Features

- [x] **Customizable Keymaps:** Move the camera using customizable
        keybindings.
- [x] **Elegant Camera Mouse Controls:** Look around with the mouse with
        vertical rotation clamping and a consistent up.
- [ ] **Mouse Wheel Enabled Zoom:** Zoom in and out using the mouse scroll wheel.
- [ ] **Full Game Pad Support:** Controll all features with a controller
        instead of mouse and keyboard.
- [ ] **Cursor Capture:** Allow cursor capture in windows with an active debug
        camera for increased comfort.
- [ ] **Camera Local Option Overrides:** Replace the global options resource
        with local per-camera overrides.

## Usage

This crate revolves around the `DebugCamera` component, which exposes the
implemented functionality to any camera it is added to. Most functionally
requires any user to add the `DebugCameraPlugin` to the app as well though.

For gettting started check out the examples.

### Using Custom Configuration

Most features of this crate are thoroughly customizable through the
`debug_camera_options` field on `DebugCameraPlugin`. All available options are
available in the documentation of `DebugCameraOptions`.

```rust
let debug_camera_options = DebugCameraOptions {
    // Update any configuration here!
    ..Default::default()
};

App::new()
    .add_plugins((DefaultPlugins, DebugCameraPlugin {
        debug_camera_options,
        ..Default::default()
    }))
    .run();
```

### Keybindings

To avoid conflicts, `DebugCameraOptions` and `InputOptions` don't come with
preconfigured keybindings. You can either provide custom `KeyBindings` or use
one of the preconfigured instances. `KeyBindings` constructed through
`Default::default` _are_ preconfigured. You can see the keymap used by them in
the table below.

| Action        | Key                   | Field           |
| ------------- | --------------------- | --------------- |
| Move Forward  | <kbd>W</kbd>          | `forward`       |
| Move Backward | <kbd>S</kbd>          | `back`          |
| Move Left     | <kbd>A</kbd>          | `left`          |
| Move Right    | <kbd>D</kbd>          | `right`         |
| Move Up       | <kbd>Q</kbd>          | `up`            |
| Move Down     | <kbd>E</kbd>          | `down`          |
| Elevate       | <kbd>R</kbd>          | `global_up`     |
| Descend       | <kbd>F</kbd>          | `global_down`   |
| Move Fast     | <kbd>Shift</kbd>      | `fast_movement` |

## Bevy Version Compatibility

| `bevy` version | `bevy_awesome_debug_camera` version  |
| -------------- | ------------------------------------ |
| 0.14           | 0.1                                  |

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.
