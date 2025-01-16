<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `TextButton`

A button with no background and border.

### Properties

- **`text`** (_in_ _string_): The text written in the button.
- **`prefix_icon`** (_in_ _image_): The image to show before the text.
- **`suffix_icon`** (_in_ _image_): The image to show after the text.
- **`action`** (_in_ _[ButtonAction](./magic_button_base.md)_): Defines if the button is displayed `default`, `primary` or `destructive`.
- **`enabled`** (_in_ _bool_): If set to `false` the button is disabled.
- **`has_hover`** (_out_ _bool_): Button sets this to `true` when the mouse is over it.
- **`has_focus`** (_out_ _bool_): Button sets this to `true` when the area has keyboard focus.
- **`pressed`** (_out_ _bool_): Set to `true` by the button when the mouse is pressed over it.
- **`enter_pressed`** (_out_ _bool_): Set to `true` by the button when the area has focus and enter key is pressed.
- **`style`** (_in_ _ButtonStyle_): Defines the style of the button.

### Callbacks

- **`clicked()`**: Invoked when clicked: A finger or the left mouse button is pressed, then released on this element.
- **`pointer_event(/* event */ PointerEvent)`**: Invoked when a button was pressed or released, a finger touched, or the pointer moved.

### Example

```slint
import { TextButton, MagicVerticalBox } from "@vivi/magic.slint";
export component Example inherits Window {
    MagicVerticalBox {
        TextButton {
            text: "Click Me";
            clicked => { self.text = "Clicked"; }
        }
    }
}
```
