<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `CheckBox`

Use a `CheckBox` to let the user select or deselect values, for example in a list with multiple options. Consider using a `Switch` element instead if the action resembles more something that’s turned on or off.

### Properties

- **`text`** (_in_ _string_): If set to `false` the checkbox is disabled.
- **`enabled`** (_in_ _bool_): If set to `false` the checkbox is disabled.
- **`checked`** (_in-out_ _bool_): Whether the checkbox is checked or not (default: false).
- **`style`** (_in_ _[CheckBoxStyle](../foundation/check_box_base.md)_): Defines the style of the checkbox.

### Callbacks

- **`toggled(/* checked */ bool)`**: The `checked` value changed.

### Example

```slint
import { CheckBox } from "@vivi/magic.slint";
export component Example inherits Window {
    width: 200px;
    height: 25px;

    CheckBox {
        text: "Hello World";
    }
}
```
