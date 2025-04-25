# okey

An easy to use and powerful key remapper for Linux written in Rust, inspired by [QMK](https://qmk.fm/).

> [!NOTE]
> This project is currently in its early stages of development. A more comprehensive documentation will be available soon.

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

## Installation

> [!IMPORTANT]
> The [Rust toolchain](https://rustup.rs/) is required to build the project.

There are currently no released binaries, but you can build the project from source using the following commands:

```bash
git clone --depth 1 https://github.com/luckasRanarison/okey/
cd okey && cargo install --path .
```

## Usage

`okey` is meant to be used as a [systemd](https://github.com/systemd/systemd) service and expects a configuration file at `~/.config/okey/config.yaml` (see the [schema](#configuration-schema)).

For simple testing, you can use the `start` command to activate keymaps.

```bash
okey start # using ~/.config/okey/config.yaml
okey start --config ./path/to/config/okey.yaml
okey start --config ./path/to/config/okey.yaml --daemon
```

To run `okey` as a service, you can use the following commands:

```bash
okey service install # creates the okey.service file
okey service start # shorthand for systemctl --user enable okey && systemctl --user start okey
```

Use `okey --help` to see all the available commands.

## Configuration Schema

> [!NOTE]
> To represent keys, `okey` uses the [keycode](https://docs.rs/evdev/latest/evdev/struct.KeyCode.html) strings from the [evdev](https://docs.rs/evdev/latest/evdev/) crate, use it for reference.

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

      KEY_M:
        [
          { press: KEY_LEFTSHIFT },
          { hold: KEY_LEFTSHIFT },
          { press: KEY_O },
          { release: KEY_O },
          { delay: 500 },
          KEY_K,
        ]

    combos:
      - keys: [KEY_D, KEY_F]
        action: KEY_LEFTCTRL

    tap_dances:
      KEY_S:
        tap: KEY_S
        hold: KEY_LEFTSHIFT
        timeout: 250 # (default: 200)

    layers:
      my_layer:
        modifier:
          key: KEY_C
          type: toggle # | oneshoot | momentary (default: momentary
        keys:
          KEY_A: KEY_D
```

</details>


Your configuration file defines how `okey` remaps keys and sets up advanced behaviors. Here's a breakdown of the schema using [TypeScript](https://www.typescriptlang.org/) type definitons:

### `defaults` (optional)

Shared global settings.

<details>

<summary>Expand</summary>

```typescript
type Defaults = {
  general?: {
    /** Delay for keys following non-acknowledged combo or tap dance keys (default: 80ms) */
    deferred_key_delay?: number;
  };
  tap_dance?: {
    /** Fallback tap dance timeout (default: 200ms) */
    default_timeout?: number;
  };
  combo?: {
    /** Window for acknowledging combos (default: 50ms) */
    default_threshold?: number;
  };
};
```

</details>

### `keyboards` (array)

Per keyboard configuration.

<details>

<summary>Expand</summary>

```typescript
type Keyboard = {
  /** Name of the keyboard as an input device, use 'cat /proc/bus/input/devices' */
  name: string;

  /** Main layer mappings, remap keys here */
  keys?: {
    [keycode: string]: KeyAction;
  };

  /** Dual function keys on tap/hold */
  tap_dances?: {
    [keycode: string]: {
      /** Action on tap, on release below timeout */
      tap: KeyAction;

      /** Action on hold, exceeded timeout */
      hold: KeyAction;

      /** When to consider as a hold (default: 200ms) */
      timeout?: 250;
    };
  };

  /** List of combo mappigns */
  combos?: {
    /** Set of keys to activate the combo */
    keys: string[];

    /** Action when keys are pressed/hold at the same time */
    action: KeyAction;
  }[];

  /** Shift like virtual layers */
  layers?: {
    [name: string]: {
      /** Layer activation key and behavior */
      modifier:
        | string
        | {
            /** Layer modfier keycode */
            key: string;

            /**
             * Layer switch behavior:
             * - momentary: Active on hold (default)
             * - toggle: Always active or inactive until toggled
             * - oneshoot: Active for one key press
             */
            type?: "momentary" | "toggle" | "oneshoot";
          };

      /** Key mappings for the layer */
      keys: {
        [keycode: string]: KeyAction;
      };
    };
  };
};
```

</details>

#### `KeyAction`

A single keycode or a sequence of key events (macro).

<details>

<summary>Expand</summary>

```typescript
/** A single keycode or a sequence of key events (macro) */
type KeyAction = string | KeyEvent[];

type KeyEvent =
  | { press: string } // key press event
  | { hold: string } // key hold event
  | { release: string } // key release event
  | { delay: number } // in milliseconds
  | string; // press + release
```

</details>

## License

MIT
