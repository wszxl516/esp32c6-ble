<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `ButtonAction`

The `ButtonAction` enum defines different variants of a button `default`, `primary` and `destructive`.

- **`default`**: The button requires less attention from the user.
- **`primary`**: Suggested action - highest level of attention.
- **`destructive`**: Destructive action - highest level of attention.

## `MagicButtonBase`

The `MagicButtonBase` represents the base for button elements that can be styled by using `ButtonStyle` and matches the magic style.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the button is disabled.
- **`has_hover`** (_out_ _bool_): Button sets this to `true` when the mouse is over it.
- **`has_focus`** (_out_ _bool_): Button sets this to `true` when the area has keyboard focus.
- **`pressed`** (_out_ _bool_): Set to `true` by the button when the mouse is pressed over it.
- **`enter_pressed`** (_out_ _bool_): Set to `true` by the button when the area has focus and enter key is pressed.
- **`style`** (_in_ _ButtonStyle_): Defines the style of the button.
- **`mouse_cursor`** (_out_ _`MouseCursor`_): The mouse cursor type when the mouse is hovering the button.
- **`preferred_min_width`** (_in_ _`length`_): Defines the preferred min width.
- **`preferred_min_height`** (_in_ _`length`_): Defines the preferred min height.

### Callbacks

- **`clicked()`**: Invoked when clicked: A finger or the left mouse button is pressed, then released on this element.
- **`pointer_event(/* event */ PointerEvent)`**: Invoked when a button was pressed or released, a finger touched, or the pointer moved.

### Example

```slint
import { MagicButtonBase, MagicText } from "@vivi/magic.slint";

export component MyButton inherits MagicButtonBase {
    in property <string> text <=> text_label.text;

    width: 60px;
    height: 20px;

    style: {
        background: black,
        border_radius: 8px,
        text_style: {
            foreground: white,
            font_size: 12px,
            font_weight: 400,
        }
    };

    text_label := MagicText {
        style: root.text_style;
        vertical_alignment: center;
        horizontal_alignment: center;
    }
}

export component Example inherits Window {
    width: 200px;
    height: 100px;

    MyButton {
        text: "Click me";

        clicked => {
            debug("Button clicked");
        }
    }
}
```
