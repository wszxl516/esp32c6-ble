<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `TabBar`

The `TabBar` displays a list of tab items and is used for tab based navigation.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the combo box is disabled.
- **`model`** (_in_ _[`TabBarItem`]_): The list of possible values.
- **`current_index`** (_in_out_ _int_): The index of the selected value. (default: -1)
- **`style`** (_in_ _TabBarStyle_): Defines the style of the combo box.

### Callbacks

- **`selected(/* index */ int)`**: A value was selected from the combo box. The argument is the currently selected value.
- **`close(/* index */ int)`**: If called it indicates the the item on the given index should be closed.

### Example

```slint
import { TabBar } from "@vivi/magic.slint";
export component Example inherits Window {
    MagicVerticalBox {
        TabBar {
          model: [
              { text: "Tab 1" },
              { text: "Tab 2" }
          ];
        }
    }
}
```
