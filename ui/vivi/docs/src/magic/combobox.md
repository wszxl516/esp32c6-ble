<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `ComboBox`

The `ComboBox` is a button that, when clicked, opens a popup to select a value.

### Properties

- **`popup_height`** (_out_ _length_): Gets the height of the popup.
- **`enabled`** (_in_ _bool_): If set to `false` the combo box is disabled.
- **`model`** (_in_ _[`ComboBoxItem`]_): The list of possible values.
- **`current_index`** (_in_out_ _int_): The index of the selected value. (default: -1)
- **`current_value`** (_in_ _[ComboBoxItem](../foundation/combobox_base.md)_): The currently selected item.
- **`placeholder_text`** (_in_ _string_): A placeholder text being shown when there is no text in the combo box.
- **`popup_position`** (_in_ _`PopupPosition`_): Defines the position of the popup. (default: bottom)
- **`style`** (_in_ _ComboBoxStyle_): Defines the style of the combo box.

### Callbacks

- **`selected(/* index */ int)`**: A value was selected from the combo box. The argument is the currently selected value.

### Example

```slint
import { ComboBox } from "@vivi/magic.slint";
export component Example inherits Window {
    width: 200px;
    height: 130px;
    ComboBox {
        y: 0px;
        width: self.preferred-width;
        height: self.preferred-height;
        model: [
            { text: "First" },
            { text: "Second" },
            { text: "Third" }
        ];
    }
}
```
