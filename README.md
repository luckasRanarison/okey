# okey

An easy to use and powerful key remapper for Linux and Mac, written in Rust.

## Features

- Key remapping
- Macros
- Combos
- Tap dance (multi-function keys on tap/hold)
- Virtual layers (TODO)

> [!NOTE]
> This project is currently in its early stages of development. The following instructions are intended for basic testing purposes, a more comprehensive documentation will be available soon.

## Installation

> [!IMPORTANT]
> The [Rust toolchain](https://rustup.rs/) is required to build the project.

There are currently no released binaries, but you can build the project from source using the following commands:

```bash
git clone --depth 1 https://github.com/luckasRanarison/okey/
cd okey && cargo install --path .
```

## Usage

The configuration for okey is written in [YAML](https://yaml.org/), here is a sample:

<details>

<summary>Expand</summary>

```yaml
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

okey is meant to be used as a service but you can use it on the fly using the CLI.

```bash
okey start --config ./path/to/config/okey.yaml
```

## TODOs

- [ ] Virtual layeres
- [ ] String literal macros
- [ ] System services
- [ ] Binary releases
- [ ] Documentation

## License

MIT
