<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `MagicListViewItem`

A `MagicListViewItem` defines an item of the `MagicListView`.

## `MagicListViewItemStyle`

A `MagicListViewItemStyle` defines an style of a magic list view item.

### Fields

- **`border_style`** (_[BorderStyle](../foundation/border.md)_): Defines the style of the background border.
- **`text_style`** (_[TextStyle](../foundation/text_base.md)_): Defines the style of the item's text.
- **`supporting_text_style`** (_[TextStyle](../foundation/text_base.md)_): Defines the style of the item's supporting text.
- **`icon_style`** (_[IconStyle](../foundation/icon_base.md)_): Defines the style of the item's icon.
- **`layout_style`** (_[LayoutStyle](../foundation/vertical_layout_base.md)_): Defines the style of the layout.

### Fields

- **`prefix_icon`** (_image_): Defines icon that is displayed in the front of the item.
- **`text`** (_string_): Defines the content of the item.
- **`supporting_text`** (_string_): Defines the supporting text of the item.

## `MagicListView`

A `MagicListView` provides a model and generates the visual presentation of its items.

### Properties

- **`style`** (_in_ _[ScrollBarStyle](../foundation/scroll_view_base.md)_): Defines the style of the scroll view.
- **`enabled`** (_in_ _bool_): Used to render the frame as disabled or enabled, but doesn't change behavior of the widget.
- **`has_focus`** (_in-out_ _bool_): Used to render the frame as focused or unfocused, but doesn't change the behavior of the widget.
- **`viewport_width`** and **`viewport_height`** (_in-out_ _length_): The `width` and `length` properties of the viewport
- **`viewport_x`** and **`viewport_y`** (_in-out_ _length_): The `x` and `y` properties of the viewport. Usually these are negative
- **`visible_width`** and **`visible_height`** (_out_ _length_): The size of the visible area of the ScrollView (not including the scrollbar)
- **`model`** (_in_ _[`MagicListViewItem`]_): Defines the list of items.
- **`current_index`** (_in_out_ _int_): The index of the selected value. (default: -1)

### Callbacks

- **`row_pointer_event(/* index */ int, /* event */ PointerEvent, /* position */ Point)`**: Emitted on any mouse pointer event similar to TouchArea. Arguments are item index associated with the event, the `PointerEvent` itself and the mouse position within the list view.

### Example

```slint
import { MagicListView, Icons } from "@vivi/magic.slint";
export component Example inherits Window {
    width: 150px;
    height: 150px;

    MagicListView {
        width: 150px;
        height: 150px;
        model: [
            { prefix_icon: Icons.add, text: "Item 1" },
            { prefix_icon: Icons.add, text: "Item 2" },
            { prefix_icon: Icons.add, text: "Item 3" },
            { prefix_icon: Icons.add, text: "Item 4" },
            { prefix_icon: Icons.add, text: "Item 5" },
            { prefix_icon: Icons.add, text: "Item 6" },
        ];
    }
}
```
