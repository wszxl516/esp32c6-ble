<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `ButtonStyle`

Defines the visual settings of a button.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`icon_style`** (_[IconStyle](./icon_base.md)_): Defines the style of the icon.
- **`text_style`** (_[TextStyle](./text_base.md)_): Defines the style of the text.
- **`layout_style`** (_[LayoutStyle](./vertical_layout_base.md)_): Defines the style of the layout.

## `ButtonBase`

The `ButtonBase` represents the base for button elements that can be styled by using `ButtonStyle`.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the button is disabled.
- **`has_hover`** (_out_ _bool_): Button sets this to `true` when the mouse is over it.
- **`has_focus`** (_out_ _bool_): Button sets this to `true` when the area has keyboard focus.
- **`pressed`** (_out_ _bool_): Set to `true` by the button when the mouse is pressed over it.
- **`enter_pressed`** (_out_ _bool_): Set to `true` by the button when the area has focus and enter key is pressed.
- **`style`** (_in_ _ButtonStyle_): Defines the style of the button.
- **`mouse_cursor`** (_out_ _`MouseCursor`_): The mouse cursor type when the mouse is hovering the button.

### Callbacks

- **`clicked()`**: Invoked when clicked: A finger or the left mouse button is pressed, then released on this element.
- **`pointer_event(/* event */ PointerEvent)`**: Invoked when a button was pressed or released, a finger touched, or the pointer moved.

### Example

```slint
import { ButtonBase, TextBase } from "@vivi/foundation.slint";

export component MyButton inherits ButtonBase {
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

    text_label := TextBase {
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
