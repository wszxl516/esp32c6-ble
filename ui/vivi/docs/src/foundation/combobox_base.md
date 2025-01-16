<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `PopupPosition`

The enum describes the position of a popup.

### Values

- **`top`**: the popup is placed on the top.
- **`bottom`**: the popup is placed on the bottom.

## `ComboBoxItem`

This struct describes an item of a combo box.

### Fields

- **`text`** (_text_): The content of the item. (default: "")

## `ComboBoxItemStyle`

Defines the visual settings of a combo box item.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`text_style`** (_[TextStyle](./text_base.md)_): Defines the style of the item's text.
- **`layout_style`** (_[LayoutStyle](./vertical_layout_base.md)_): Defines the style of the layout.

## `ComboBoxStyle`

Defines the visual settings of a combo box.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`icon_style`** (_[IconStyle](./icon_base.md)_): Defines the style of the dropdown icon.
- **`text_style`** (_[TextStyle](./text_base.md)_): Defines the style of the current value text.
- **`placeholder_style`** (_[TextStyle](./text_base.md)_): Defines the style of the placeholder text.
- **`padding_left`** (_length_): Defines the padding on the left side of the combo box layout. (default value: 0)
- **`padding_right`** (_length_): Defines the padding on the right side of the combo box layout. (default value: 0)
- **`padding_top`** (_length_): Defines the padding on the top side of the combo box layout. (default value: 0)
- **`padding_bottom`** (_length_): Defines the padding on the bottom side of the combo box layout. (default value: 0)
- **`spacing`** (_length_): Defines the distance between elements in the combo box layout. (default value: 0)
- **`item_style`** (_`ComboBoxItemStyle`_): Defines style of the combo box items.
- **`popup_border_style`** (_[BorderStyle](./border.md)_): Defines the background of the popup border.

## `ComboBoxBase`

The `ComboBoxBase` represents the base for a combobox component.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the combo box is disabled.
- **`model`** (_in_ _[`ComboBoxItem`]_): The list of possible values.
- **`current_index`** (_in_out_ _int_): The index of the selected value. (default: -1)
- **`current_value`** (_in_ _`ComboBoxItem`_): The currently selected item.
- **`placeholder_text`** (_in_ _string_): A placeholder text being shown when there is no text in the combo box.
- **`popup_position`** (_in_ _`PopupPosition`_): Defines the position of the popup. (default: bottom)
- **`style`** (_in_ _ComboBoxStyle_): Defines the style of the combo box.

### Callbacks

- **`selected(/* index */ int)`**: A value was selected from the combo box. The argument is the currently selected value.
