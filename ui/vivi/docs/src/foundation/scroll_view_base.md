<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `ScrollBarStyle`

Defines the visual settings of a scroll bar.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`handle_style`** (_[BorderStyle](./border.md)_): Defines the style of the handle.

## `ScrollViewStyle`

Defines the visual settings of a scroll view.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`scroll_bar_style`** (_`ScrollBarStyle`_): Defines the style of the scroll bars of the scroll view.

## `ScrollViewBase`

`ScrollViewBase` represents the base for scroll view components.

### Properties

- **`style`** (_in_ _`ScrollBarStyle`_): Defines the style of the scroll view.
- **`enabled`** (_in_ _bool_): Used to render the frame as disabled or enabled, but doesn't change behavior of the widget.
- **`has_focus`** (_in_out_ _bool_): Used to render the frame as focused or unfocused, but doesn't change the behavior of the widget.
- **`viewport_width`** and **`viewport_height`** (_in-out_ _length_): The `width` and `length` properties of the viewport
- **`viewport_x`** and **`viewport_y`** (_in-out_ _length_): The `x` and `y` properties of the viewport. Usually these are negative
- **`visible_width`** and **`visible_height`** (_out_ _length_): The size of the visible area of the ScrollView (not including the scrollbar)

### Example

```slint
import { ScrollViewBase } from "@vivi/foundation.slint";

export component ScrollView inherits ScrollViewBase {
    flickable := Flickable {
        viewport_x <=> horizontal_scroll_bar.value;
        viewport_y <=> vertical_scroll_bar.value;

        @children
    }
}

export component Example inherits Window {
    width: 200px;
    height: 200px;
    ScrollView {
        width: 200px;
        height: 200px;
        viewport-width: 300px;
        viewport-height: 300px;
        Rectangle { width: 30px; height: 30px; x: 275px; y: 50px; background: blue; }
        Rectangle { width: 30px; height: 30px; x: 175px; y: 130px; background: red; }
        Rectangle { width: 30px; height: 30px; x: 25px; y: 210px; background: yellow; }
        Rectangle { width: 30px; height: 30px; x: 98px; y: 55px; background: orange; }
    }
}
```
