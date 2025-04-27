# okey

An advanced, easy-to-use key remapper for Linux written in Rust, inspired by [QMK](https://qmk.fm/).

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
- Virtual layers

> [!TIP]
> The features are composable. For example, you can use a combo to trigger a tap dance.

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
          KEY_K, # press + release
        ]

      KEY_R: [{ string: "Hello " } , { env: USERNAME }]
      KEY_T: { unicode: 🙂👍 }
      KEY_Y: { shell: 'date "+%d %B %Y"', trim: true }

    combos:
      - keys: [KEY_D, KEY_F]
        action: KEY_LEFTCTRL

    tap_dances:
      KEY_S:
        tap: KEY_S
        hold: KEY_LEFTSHIFT
        timeout: 250

    layers:
      my_layer:
        modifier:
          key: KEY_C
          type: toggle # | oneshoot | momentary (default: momentary)
        keys:
          KEY_A: KEY_D
```

</details>


Your configuration file defines how `okey` remaps keys and sets up advanced behaviors. Here's a breakdown of the schema:

### `defaults` (optional)

Shared global settings, fields:

#### `general`

- `deferred_key_delay`: Delay for keys following non-acknowledged special keys.

  _Type_: `number`

  _Default_: `80` (ms)

- `unicode_input_delay`: Delay for inserting unicode codepoints with macro. (flushing)

  _Type_: `number`

  _Default_: `50` (ms)

- `event_poll_timeout`: Controls the main event loop interval.

  _Type_: `number`

  _Default_: `1` (ms)

#### `tap_dance`

- `default_timeout`: Fallback tap dance timeout.

  _Type_: `number`

  _Default_: `200` (ms)

#### `combo`

- `default_threshold`: Window for acknowledging combos.

  _Type_: `number`

  _Default_: `50` (ms)

### `keyboards` (array)

Per keyboard configuration.

Type aliases:

<details>

<summary><code>KeyCode</code> (expand)</summary>

#### `KeyCode`

A custom string or one of:

- `KEY_RESERVED`
- `KEY_ESC`
- `KEY_1`
- `KEY_2`
- `KEY_3`
- `KEY_4`
- `KEY_5`
- `KEY_6`
- `KEY_7`
- `KEY_8`
- `KEY_9`
- `KEY_0`
- `KEY_MINUS`
- `KEY_EQUAL`
- `KEY_BACKSPACE`
- `KEY_TAB`
- `KEY_Q`
- `KEY_W`
- `KEY_E`
- `KEY_R`
- `KEY_T`
- `KEY_Y`
- `KEY_U`
- `KEY_I`
- `KEY_O`
- `KEY_P`
- `KEY_LEFTBRACE`
- `KEY_RIGHTBRACE`
- `KEY_ENTER`
- `KEY_LEFTCTRL`
- `KEY_A`
- `KEY_S`
- `KEY_D`
- `KEY_F`
- `KEY_G`
- `KEY_H`
- `KEY_J`
- `KEY_K`
- `KEY_L`
- `KEY_SEMICOLON`
- `KEY_APOSTROPHE`
- `KEY_GRAVE`
- `KEY_LEFTSHIFT`
- `KEY_BACKSLASH`
- `KEY_Z`
- `KEY_X`
- `KEY_C`
- `KEY_V`
- `KEY_B`
- `KEY_N`
- `KEY_M`
- `KEY_COMMA`
- `KEY_DOT`
- `KEY_SLASH`
- `KEY_RIGHTSHIFT`
- `KEY_KPASTERISK`
- `KEY_LEFTALT`
- `KEY_SPACE`
- `KEY_CAPSLOCK`
- `KEY_F1`
- `KEY_F2`
- `KEY_F3`
- `KEY_F4`
- `KEY_F5`
- `KEY_F6`
- `KEY_F7`
- `KEY_F8`
- `KEY_F9`
- `KEY_F10`
- `KEY_NUMLOCK`
- `KEY_SCROLLLOCK`
- `KEY_KP7`
- `KEY_KP8`
- `KEY_KP9`
- `KEY_KPMINUS`
- `KEY_KP4`
- `KEY_KP5`
- `KEY_KP6`
- `KEY_KPPLUS`
- `KEY_KP1`
- `KEY_KP2`
- `KEY_KP3`
- `KEY_KP0`
- `KEY_KPDOT`
- `KEY_ZENKAKUHANKAKU`
- `KEY_102ND`
- `KEY_F11`
- `KEY_F12`
- `KEY_RO`
- `KEY_KATAKANA`
- `KEY_HIRAGANA`
- `KEY_HENKAN`
- `KEY_KATAKANAHIRAGANA`
- `KEY_MUHENKAN`
- `KEY_KPJPCOMMA`
- `KEY_KPENTER`
- `KEY_RIGHTCTRL`
- `KEY_KPSLASH`
- `KEY_SYSRQ`
- `KEY_RIGHTALT`
- `KEY_LINEFEED`
- `KEY_HOME`
- `KEY_UP`
- `KEY_PAGEUP`
- `KEY_LEFT`
- `KEY_RIGHT`
- `KEY_END`
- `KEY_DOWN`
- `KEY_PAGEDOWN`
- `KEY_INSERT`
- `KEY_DELETE`
- `KEY_MACRO`
- `KEY_MUTE`
- `KEY_VOLUMEDOWN`
- `KEY_VOLUMEUP`
- `KEY_POWER`
- `KEY_KPEQUAL`
- `KEY_KPPLUSMINUS`
- `KEY_PAUSE`
- `KEY_SCALE`
- `KEY_KPCOMMA`
- `KEY_HANGEUL`
- `KEY_HANJA`
- `KEY_YEN`
- `KEY_LEFTMETA`
- `KEY_RIGHTMETA`
- `KEY_COMPOSE`
- `KEY_STOP`
- `KEY_AGAIN`
- `KEY_PROPS`
- `KEY_UNDO`
- `KEY_FRONT`
- `KEY_COPY`
- `KEY_OPEN`
- `KEY_PASTE`
- `KEY_FIND`
- `KEY_CUT`
- `KEY_HELP`
- `KEY_MENU`
- `KEY_CALC`
- `KEY_SETUP`
- `KEY_SLEEP`
- `KEY_WAKEUP`
- `KEY_FILE`
- `KEY_SENDFILE`
- `KEY_DELETEFILE`
- `KEY_XFER`
- `KEY_PROG1`
- `KEY_PROG2`
- `KEY_WWW`
- `KEY_MSDOS`
- `KEY_COFFEE`
- `KEY_DIRECTION`
- `KEY_ROTATE_DISPLAY`
- `KEY_CYCLEWINDOWS`
- `KEY_MAIL`
- `KEY_BOOKMARKS`
- `KEY_COMPUTER`
- `KEY_BACK`
- `KEY_FORWARD`
- `KEY_CLOSECD`
- `KEY_EJECTCD`
- `KEY_EJECTCLOSECD`
- `KEY_NEXTSONG`
- `KEY_PLAYPAUSE`
- `KEY_PREVIOUSSONG`
- `KEY_STOPCD`
- `KEY_RECORD`
- `KEY_REWIND`
- `KEY_PHONE`
- `KEY_ISO`
- `KEY_CONFIG`
- `KEY_HOMEPAGE`
- `KEY_REFRESH`
- `KEY_EXIT`
- `KEY_MOVE`
- `KEY_EDIT`
- `KEY_SCROLLUP`
- `KEY_SCROLLDOWN`
- `KEY_KPLEFTPAREN`
- `KEY_KPRIGHTPAREN`
- `KEY_NEW`
- `KEY_REDO`
- `KEY_F13`
- `KEY_F14`
- `KEY_F15`
- `KEY_F16`
- `KEY_F17`
- `KEY_F18`
- `KEY_F19`
- `KEY_F20`
- `KEY_F21`
- `KEY_F22`
- `KEY_F23`
- `KEY_F24`
- `KEY_PLAYCD`
- `KEY_PAUSECD`
- `KEY_PROG3`
- `KEY_PROG4`
- `KEY_DASHBOARD`
- `KEY_SUSPEND`
- `KEY_CLOSE`
- `KEY_PLAY`
- `KEY_FASTFORWARD`
- `KEY_BASSBOOST`
- `KEY_PRINT`
- `KEY_HP`
- `KEY_CAMERA`
- `KEY_SOUND`
- `KEY_QUESTION`
- `KEY_EMAIL`
- `KEY_CHAT`
- `KEY_SEARCH`
- `KEY_CONNECT`
- `KEY_FINANCE`
- `KEY_SPORT`
- `KEY_SHOP`
- `KEY_ALTERASE`
- `KEY_CANCEL`
- `KEY_BRIGHTNESSDOWN`
- `KEY_BRIGHTNESSUP`
- `KEY_MEDIA`
- `KEY_SWITCHVIDEOMODE`
- `KEY_KBDILLUMTOGGLE`
- `KEY_KBDILLUMDOWN`
- `KEY_KBDILLUMUP`
- `KEY_SEND`
- `KEY_REPLY`
- `KEY_FORWARDMAIL`
- `KEY_SAVE`
- `KEY_DOCUMENTS`
- `KEY_BATTERY`
- `KEY_BLUETOOTH`
- `KEY_WLAN`
- `KEY_UWB`
- `KEY_UNKNOWN`
- `KEY_VIDEO_NEXT`
- `KEY_VIDEO_PREV`
- `KEY_BRIGHTNESS_CYCLE`
- `KEY_BRIGHTNESS_AUTO`
- `KEY_DISPLAY_OFF`
- `KEY_WWAN`
- `KEY_RFKILL`
- `KEY_MICMUTE`
- `BTN_0`
- `BTN_1`
- `BTN_2`
- `BTN_3`
- `BTN_4`
- `BTN_5`
- `BTN_6`
- `BTN_7`
- `BTN_8`
- `BTN_9`
- `BTN_LEFT`
- `BTN_RIGHT`
- `BTN_MIDDLE`
- `BTN_SIDE`
- `BTN_EXTRA`
- `BTN_FORWARD`
- `BTN_BACK`
- `BTN_TASK`
- `BTN_TRIGGER`
- `BTN_THUMB`
- `BTN_THUMB2`
- `BTN_TOP`
- `BTN_TOP2`
- `BTN_PINKIE`
- `BTN_BASE`
- `BTN_BASE2`
- `BTN_BASE3`
- `BTN_BASE4`
- `BTN_BASE5`
- `BTN_BASE6`
- `BTN_DEAD`
- `BTN_SOUTH`
- `BTN_EAST`
- `BTN_C`
- `BTN_NORTH`
- `BTN_WEST`
- `BTN_Z`
- `BTN_TL`
- `BTN_TR`
- `BTN_TL2`
- `BTN_TR2`
- `BTN_SELECT`
- `BTN_START`
- `BTN_MODE`
- `BTN_THUMBL`
- `BTN_THUMBR`
- `BTN_TOOL_PEN`
- `BTN_TOOL_RUBBER`
- `BTN_TOOL_BRUSH`
- `BTN_TOOL_PENCIL`
- `BTN_TOOL_AIRBRUSH`
- `BTN_TOOL_FINGER`
- `BTN_TOOL_MOUSE`
- `BTN_TOOL_LENS`
- `BTN_TOOL_QUINTTAP`
- `BTN_TOUCH`
- `BTN_STYLUS`
- `BTN_STYLUS2`
- `BTN_TOOL_DOUBLETAP`
- `BTN_TOOL_TRIPLETAP`
- `BTN_TOOL_QUADTAP`
- `BTN_GEAR_DOWN`
- `BTN_GEAR_UP`
- `KEY_OK`
- `KEY_SELECT`
- `KEY_GOTO`
- `KEY_CLEAR`
- `KEY_POWER2`
- `KEY_OPTION`
- `KEY_INFO`
- `KEY_TIME`
- `KEY_VENDOR`
- `KEY_ARCHIVE`
- `KEY_PROGRAM`
- `KEY_CHANNEL`
- `KEY_FAVORITES`
- `KEY_EPG`
- `KEY_PVR`
- `KEY_MHP`
- `KEY_LANGUAGE`
- `KEY_TITLE`
- `KEY_SUBTITLE`
- `KEY_ANGLE`
- `KEY_ZOOM`
- `KEY_FULL_SCREEN`
- `KEY_MODE`
- `KEY_KEYBOARD`
- `KEY_SCREEN`
- `KEY_PC`
- `KEY_TV`
- `KEY_TV2`
- `KEY_VCR`
- `KEY_VCR2`
- `KEY_SAT`
- `KEY_SAT2`
- `KEY_CD`
- `KEY_TAPE`
- `KEY_RADIO`
- `KEY_TUNER`
- `KEY_PLAYER`
- `KEY_TEXT`
- `KEY_DVD`
- `KEY_AUX`
- `KEY_MP3`
- `KEY_AUDIO`
- `KEY_VIDEO`
- `KEY_DIRECTORY`
- `KEY_LIST`
- `KEY_MEMO`
- `KEY_CALENDAR`
- `KEY_RED`
- `KEY_GREEN`
- `KEY_YELLOW`
- `KEY_BLUE`
- `KEY_CHANNELUP`
- `KEY_CHANNELDOWN`
- `KEY_FIRST`
- `KEY_LAST`
- `KEY_AB`
- `KEY_NEXT`
- `KEY_RESTART`
- `KEY_SLOW`
- `KEY_SHUFFLE`
- `KEY_BREAK`
- `KEY_PREVIOUS`
- `KEY_DIGITS`
- `KEY_TEEN`
- `KEY_TWEN`
- `KEY_VIDEOPHONE`
- `KEY_GAMES`
- `KEY_ZOOMIN`
- `KEY_ZOOMOUT`
- `KEY_ZOOMRESET`
- `KEY_WORDPROCESSOR`
- `KEY_EDITOR`
- `KEY_SPREADSHEET`
- `KEY_GRAPHICSEDITOR`
- `KEY_PRESENTATION`
- `KEY_DATABASE`
- `KEY_NEWS`
- `KEY_VOICEMAIL`
- `KEY_ADDRESSBOOK`
- `KEY_MESSENGER`
- `KEY_DISPLAYTOGGLE`
- `KEY_SPELLCHECK`
- `KEY_LOGOFF`
- `KEY_DOLLAR`
- `KEY_EURO`
- `KEY_FRAMEBACK`
- `KEY_FRAMEFORWARD`
- `KEY_CONTEXT_MENU`
- `KEY_MEDIA_REPEAT`
- `KEY_10CHANNELSUP`
- `KEY_10CHANNELSDOWN`
- `KEY_IMAGES`
- `KEY_PICKUP_PHONE`
- `KEY_HANGUP_PHONE`
- `KEY_DEL_EOL`
- `KEY_DEL_EOS`
- `KEY_INS_LINE`
- `KEY_DEL_LINE`
- `KEY_FN`
- `KEY_FN_ESC`
- `KEY_FN_F1`
- `KEY_FN_F2`
- `KEY_FN_F3`
- `KEY_FN_F4`
- `KEY_FN_F5`
- `KEY_FN_F6`
- `KEY_FN_F7`
- `KEY_FN_F8`
- `KEY_FN_F9`
- `KEY_FN_F10`
- `KEY_FN_F11`
- `KEY_FN_F12`
- `KEY_FN_1`
- `KEY_FN_2`
- `KEY_FN_D`
- `KEY_FN_E`
- `KEY_FN_F`
- `KEY_FN_S`
- `KEY_FN_B`
- `KEY_BRL_DOT1`
- `KEY_BRL_DOT2`
- `KEY_BRL_DOT3`
- `KEY_BRL_DOT4`
- `KEY_BRL_DOT5`
- `KEY_BRL_DOT6`
- `KEY_BRL_DOT7`
- `KEY_BRL_DOT8`
- `KEY_BRL_DOT9`
- `KEY_BRL_DOT10`
- `KEY_NUMERIC_0`
- `KEY_NUMERIC_1`
- `KEY_NUMERIC_2`
- `KEY_NUMERIC_3`
- `KEY_NUMERIC_4`
- `KEY_NUMERIC_5`
- `KEY_NUMERIC_6`
- `KEY_NUMERIC_7`
- `KEY_NUMERIC_8`
- `KEY_NUMERIC_9`
- `KEY_NUMERIC_STAR`
- `KEY_NUMERIC_POUND`
- `KEY_NUMERIC_A`
- `KEY_NUMERIC_B`
- `KEY_NUMERIC_C`
- `KEY_NUMERIC_D`
- `KEY_CAMERA_FOCUS`
- `KEY_WPS_BUTTON`
- `KEY_TOUCHPAD_TOGGLE`
- `KEY_TOUCHPAD_ON`
- `KEY_TOUCHPAD_OFF`
- `KEY_CAMERA_ZOOMIN`
- `KEY_CAMERA_ZOOMOUT`
- `KEY_CAMERA_UP`
- `KEY_CAMERA_DOWN`
- `KEY_CAMERA_LEFT`
- `KEY_CAMERA_RIGHT`
- `KEY_ATTENDANT_ON`
- `KEY_ATTENDANT_OFF`
- `KEY_ATTENDANT_TOGGLE`
- `KEY_LIGHTS_TOGGLE`
- `BTN_DPAD_UP`
- `BTN_DPAD_DOWN`
- `BTN_DPAD_LEFT`
- `BTN_DPAD_RIGHT`
- `KEY_ALS_TOGGLE`
- `KEY_BUTTONCONFIG`
- `KEY_TASKMANAGER`
- `KEY_JOURNAL`
- `KEY_CONTROLPANEL`
- `KEY_APPSELECT`
- `KEY_SCREENSAVER`
- `KEY_VOICECOMMAND`
- `KEY_ASSISTANT`
- `KEY_KBD_LAYOUT_NEXT`
- `KEY_BRIGHTNESS_MIN`
- `KEY_BRIGHTNESS_MAX`
- `KEY_KBDINPUTASSIST_PREV`
- `KEY_KBDINPUTASSIST_NEXT`
- `KEY_KBDINPUTASSIST_PREVGROUP`
- `KEY_KBDINPUTASSIST_NEXTGROUP`
- `KEY_KBDINPUTASSIST_ACCEPT`
- `KEY_KBDINPUTASSIST_CANCEL`
- `KEY_RIGHT_UP`
- `KEY_RIGHT_DOWN`
- `KEY_LEFT_UP`
- `KEY_LEFT_DOWN`
- `KEY_ROOT_MENU`
- `KEY_MEDIA_TOP_MENU`
- `KEY_NUMERIC_11`
- `KEY_NUMERIC_12`
- `KEY_AUDIO_DESC`
- `KEY_3D_MODE`
- `KEY_NEXT_FAVORITE`
- `KEY_STOP_RECORD`
- `KEY_PAUSE_RECORD`
- `KEY_VOD`
- `KEY_UNMUTE`
- `KEY_FASTREVERSE`
- `KEY_SLOWREVERSE`
- `KEY_DATA`
- `KEY_ONSCREEN_KEYBOARD`
- `KEY_PRIVACY_SCREEN_TOGGLE`
- `KEY_SELECTIVE_SCREENSHOT`
- `BTN_TRIGGER_HAPPY1`
- `BTN_TRIGGER_HAPPY2`
- `BTN_TRIGGER_HAPPY3`
- `BTN_TRIGGER_HAPPY4`
- `BTN_TRIGGER_HAPPY5`
- `BTN_TRIGGER_HAPPY6`
- `BTN_TRIGGER_HAPPY7`
- `BTN_TRIGGER_HAPPY8`
- `BTN_TRIGGER_HAPPY9`
- `BTN_TRIGGER_HAPPY10`
- `BTN_TRIGGER_HAPPY11`
- `BTN_TRIGGER_HAPPY12`
- `BTN_TRIGGER_HAPPY13`
- `BTN_TRIGGER_HAPPY14`
- `BTN_TRIGGER_HAPPY15`
- `BTN_TRIGGER_HAPPY16`
- `BTN_TRIGGER_HAPPY17`
- `BTN_TRIGGER_HAPPY18`
- `BTN_TRIGGER_HAPPY19`
- `BTN_TRIGGER_HAPPY20`
- `BTN_TRIGGER_HAPPY21`
- `BTN_TRIGGER_HAPPY22`
- `BTN_TRIGGER_HAPPY23`
- `BTN_TRIGGER_HAPPY24`
- `BTN_TRIGGER_HAPPY25`
- `BTN_TRIGGER_HAPPY26`
- `BTN_TRIGGER_HAPPY27`
- `BTN_TRIGGER_HAPPY28`
- `BTN_TRIGGER_HAPPY29`
- `BTN_TRIGGER_HAPPY30`
- `BTN_TRIGGER_HAPPY31`
- `BTN_TRIGGER_HAPPY32`
- `BTN_TRIGGER_HAPPY33`
- `BTN_TRIGGER_HAPPY34`
- `BTN_TRIGGER_HAPPY35`
- `BTN_TRIGGER_HAPPY36`
- `BTN_TRIGGER_HAPPY37`
- `BTN_TRIGGER_HAPPY38`
- `BTN_TRIGGER_HAPPY39`
- `BTN_TRIGGER_HAPPY40`

</details>

<details>

<summary><code>KeyAction</code> (expand)</summary>

#### `KeyAction`

A single keycode or a sequence of key events (macro).

_Type_: `KeyCode` | `KeyEvent[]`

_Example_: `KEY_C`, `[KEY_H, { press: KEY_I }, { release: KEY_I }]`

#### `KeyEvent`

> To **hold** a key, a press event musy be preceding hold.

_Type_:

- `{ press: KeyCode }`
- `{ hold: KeyCode }`
- `{ release: KeyCode }`
- `{ delay: number }`: Input delay in milliseconds.
- `{ string: string }`: ASCII string.
- `{ env: string }`: Environment variable key.
- `{ unicode: string }`: Unicode string.
- `{ shell: string }`: Bash shell command.
- `KeyCode`: Press + Release.

</details>

<details>

<summary><code>TapDance</code> (expand)</summary>

#### `TapDance`

Tap dance entry configuration.

- `tap`: Action on tap, on release below timeout.

  _Type_: `KeyAction`

- `hold`: Action on hold, exceeded timeout.

  _Type_: `KeyAction`

- `timeout`: When to consider as a hold.

  _Type_: `number`

  _Default_: `250` (ms)

</details>

<details>

<summary><code>Combo</code> (expand)</summary>

#### `Combo`

Combo entry cobfiguration.

- `keys`: Set of keys to activate the combo.

  _Type_: `KeyCode[]`

- `action`: Action when keys are pressed/held at the same time.

  _Type_: `KeyAction`

</details>

<details>

<summary><code>Layer</code> (expand)</summary>

#### `Layer`

Layer entry configuration.

- `modifier`: Layer activation key and behavior.

  _Type_: `KeyCode` | `{ key: KeyCode; type?: "momentary" | "toggle" | "oneshoot" }`

- `keys`: Key mappings for the layer.

  _Type_: `Record<KeyCode, KeyAction>`

</details>

Fields:

#### `name`

Name of the keyboard as an input device, use `cat /proc/bus/input/devices`.

  _Type_: `string`

#### `keys` (optional)

Key mappings for the main layer.

_Type_: `Record<KeyCode, KeyAction>`


#### `tap_dances` (optional)

Dual function keys on tap/hold.

_Type_: `Record<KeyCode, TapDance>`


#### `combos` (optional)

List of combo mappings.

_Type_: `Combo[]`


#### `layers` (optional)

Virtual layers (shift-like).

_Type_: `Record<string, Layer>`


## License

MIT
