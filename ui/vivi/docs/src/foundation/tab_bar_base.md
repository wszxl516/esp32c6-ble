<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `TabBarItem`

This struct describes an item of a combo box.

### Fields

- **`prefix_icon`** (_image_): Defines icon that is displayed in the front of the item.
- **`text`** (_string_): Defines the content of the item. (default: "")
- **`closeable`** (_bool_): If set to true a close button will be displayed. (default: false)

## `TabBarItemStyle`

Defines the visual settings of a combo box item.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`text_style`** (_[TextStyle](./text_base.md)_): Defines the style of the item's text.
- **`layout_style`** (_[LayoutStyle](./vertical_layout_base.md)_): Defines the style of the layout.
- **`icon_style`** (_[IconStyle](../foundation/icon_base.md)_): Defines the style of the item's icon.

## `TabBarStyle`

Defines the visual settings of a combo box.

### Fields

- **`item_style`** (_`TabBarItemStyle`_): Defines style of the combo box items.

## `TabBarBase`

The `TabBarBase` represents the base for a tab bar component.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the combo box is disabled.
- **`model`** (_in_ _[`TabBarItem`]_): The list of possible values.
- **`current_index`** (_in_out_ _int_): The index of the selected value. (default: -1)
- **`style`** (_in_ _TabBarStyle_): Defines the style of the combo box.

### Callbacks

- **`selected(/* index */ int)`**: A value was selected from the combo box. The argument is the currently selected value.
- **`close(/* index */ int)`**: If called it indicates the the item on the given index should be closed.
