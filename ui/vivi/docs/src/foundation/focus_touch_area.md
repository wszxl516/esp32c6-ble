<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `FocusTouchArea`

Use `FocusTouchArea` to control what happens when the region it covers is touched or interacted with
using the mouse. The `clicked` callback is also emitted when the element has focus and enter is pressed.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the touch area is disabled.
- **`has_hover`** (_out_ _bool_): `FocusTouchArea` sets this to `true` when the mouse is over it.
- **`has_focus`** (_out_ _bool_): `FocusTouchArea` sets this to `true` when the area has keyboard focus.
- **`pressed`** (_out_ _bool_): Set to `true` by the `FocusTouchArea` when the mouse is pressed over it.
- **`enter_pressed`** (_out_ _bool_): Set to `true` by the `FocusTouchArea` when the area has focus and enter key is pressed.
- **`mouse_cursor`** (_out_ _`MouseCursor`_): The mouse cursor type when the mouse is hovering the FocusTouchArea.

### Callbacks

- **`clicked()`**: Invoked when clicked: A finger or the left mouse button is pressed, then released on this element.
- **`pointer_event(/* event */ PointerEvent)`**: Invoked when a button was pressed or released, a finger touched, or the pointer moved.
- **`key_pressed(/`_ event_/` KeyEvent)`**: Invoked when element has focus and a key is pressed. Ignores `enter` key.

### Example

```slint
import { FocusTouchArea } from "@vivi/foundation.slint";

export component Example inherits Window {
    width: 200px;
    height: 100px;

    touch-area := FocusTouchArea {
        width: parent.width;
        height: parent.height;

        clicked => {
            rect2.background = #ff0;
        }
    }

    Rectangle {
        x:0;
        width: parent.width / 2;
        height: parent.height;
        background: area.pressed ? blue: red;
    }

    rect2 := Rectangle {
        x: parent.width / 2;
        width: parent.width / 2;
        height: parent.height;
    }
}
```
