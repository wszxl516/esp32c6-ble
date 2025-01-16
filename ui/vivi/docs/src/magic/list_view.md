<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `ListView`

A `ListView` is like a [ScrollView](./scroll_view.md) but it should have a `for` element, and the content are
automatically laid out in a list.
Elements are only instantiated if they are visible.

### Properties

- **`style`** (_in_ _[ScrollBarStyle](../foundation/scroll_view_base.md)_): Defines the style of the scroll view.
- **`enabled`** (_in_ _bool_): Used to render the frame as disabled or enabled, but doesn't change behavior of the widget.
- **`has_focus`** (_in-out_ _bool_): Used to render the frame as focused or unfocused, but doesn't change the behavior of the widget.
- **`viewport_width`** and **`viewport_height`** (_in-out_ _length_): The `width` and `length` properties of the viewport
- **`viewport_x`** and **`viewport_y`** (_in-out_ _length_): The `x` and `y` properties of the viewport. Usually these are negative
- **`visible_width`** and **`visible_height`** (_out_ _length_): The size of the visible area of the ScrollView (not including the scrollbar)

### Example

```slint
import { ListView } from "@vivi/magic.slint";
export component Example inherits Window {
    width: 150px;
    height: 150px;
    ListView {
        width: 150px;
        height: 150px;
        for data in [
            { text: "Blue", color: #0000ff, bg: #eeeeee},
            { text: "Red", color: #ff0000, bg: #eeeeee},
            { text: "Green", color: #00ff00, bg: #eeeeee},
            { text: "Yellow", color: #ffff00, bg: #222222 },
            { text: "Black", color: #000000, bg: #eeeeee },
            { text: "White", color: #ffffff, bg: #222222 },
            { text: "Magenta", color: #ff00ff, bg: #eeeeee },
            { text: "Cyan", color: #00ffff, bg: #222222 },
        ] : Rectangle {
            height: 30px;
            background: data.bg;
            width: parent.width;
            Text {
                x: 0;
                text: data.text;
                color: data.color;
            }
        }
    }
}
```
