# okey

An easy to use and powerful key remapper for Linux written in Rust, inspired by [QMK](https://qmk.fm/).

## Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Configuration Schema](#configuration-schema)
    - [Defaults](#defaults-optional)
    - [Keyboards](#keyboards-array)
5. [License](#license)

## Features

- Key remapping
- Macros
- Combos
- Tap dance (multi-function keys on tap/hold)
- Virtual layers (TODO)

> [!NOTE]
> This project is currently in its early stages of development. A more comprehensive documentation will be available soon.

## Installation

> [!IMPORTANT]
> The [Rust toolchain](https://rustup.rs/) is required to build the project.

There are currently no released binaries, but you can build the project from source using the following commands:

```bash
git clone --depth 1 https://github.com/luckasRanarison/okey/
cd okey && cargo install --path .
```

## Usage

`okey` is meant to be used as a [systemd](https://github.com/systemd/systemd) service and expects a configuration file is required at `~/.config/okey/config.yaml` (see the [schema](#configuration-schema)).

For simple testing, you can use the `start` command to activate keymaps.

```bash
okey start # using ~/.config/okey/config.yaml
okey start --config ./path/to/config/okey.yaml
okey start --config ./path/to/config/okey.yaml --daemon
```

To run `okey` as a service, you can use the following commands:

```
okey service install -- creates the okey.service file
okey service start -- shorthand for systemctl --user enable okey && systemd --user start okey
```

Use `okey --help` to see all the available commands.

## Configuration Schema

The configuration for okey is written in [YAML](https://yaml.org/), and here is a sample:

<details>

<summary>Expand</summary>

```yaml
defaults:
  general:
    deferred_key_delay: 80
  tap_dance:
    default_timeout: 200
  combo:
    default_threshold: 50

keyboards:
  - name: "AT Translated Set 2 keyboard"

    keys:
      KEY_Q: KEY_A
      KEY_CAPSLOCK: [KEY_H, KEY_E, KEY_L, KEY_L, KEY_O]

    combos:
      - keys: [KEY_D, KEY_F]
        action: KEY_LEFTCTRL

    tap_dances:
      KEY_S:
        tap: KEY_S
        hold: KEY_LEFTSHIFT

    layers:
      my_layer:
        modifier:
          key: KEY_C
          # type: momentary (default) | toggle | oneshoot
        keys:
          KEY_A: KEY_D
```

</details>


Your configuration file defines how `okey` remaps keys and sets up advanced behaviors. Here's a breakdown of all available fields:

### `defaults` (optional)

Shared global settings.

#### **`general`**

- **`deferred_key_delay`** (number): Delay for keys following non-acknowledged combos or tap dance keys.

#### **`tap_dances`**

- **`default_timeout`** (number): Fallback tap dance timeout.

#### **`combo`**

- **`default_threshold`** (number): Window for acknowledging key combos.

### `keyboards` (array)

Defines the keyboards to apply remappings to.

Each keyboard:

- **`name`** (`string`, required): The exact name of the keyboard as an input device (use `cat /proc/bus/input/devices`).

- **`keys`** (`map<string, string | string[]>`, optional): Simple key remaps or macros.  

  - Remap one key to another:  
    `KEY_Q: KEY_A`  
  - Define a macro (sequence of keys):  
    `KEY_CAPSLOCK: [KEY_H, KEY_E, KEY_L, KEY_L, KEY_O]`

- **`combos`** (`array<object>`, optional): Define combos — pressing multiple keys together to trigger another action.
  
  Each combo object contains:
  - **`keys`** (`string[]`, required): The key combination.
  - **`action`** (`string`, required): The triggered key.

- **`tap_dances`** (`map<string, object>`, optional): Define tap/hold behavior for a key.
  
  Each entry:
  - **`tap`** (`string`, required): The key to send on a tap.
  - **`hold`** (`string`, required): The key to send on a hold.

- **`layers`** (`map<string, object>`, optional): Define virtual layers (like shift layers on steroids).

  Each layer:
  - **`modifier`** (`object`, required):  
    The key that activates the layer.
    - **`key`** (`string`, required): The modifier key.
    - **`type`** (`string`, optional): Activation type.  
      One of:  
      - `momentary` (default): active while holding
      - `toggle`: toggles on/off
      - `oneshoot`: active for one keypress
  - **`keys`** (`map<string, string>`, required):  The remappings that apply within this layer.

## License

MIT
