<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `Switch`

A `Switch` is a representation of a physical switch that allows users to turn things on or off. Consider using a `CheckBox` instead if you want the user to select or deselect values, for example in a list with multiple options.

### Properties

- **`checked_icon`** (_in_ _image_): The icon that is displayed if `checked` is `true`.
- **`unchecked_icon`** (_in_ _image_): The icon that is displayed if `checked` is `false`.
- **`text`** (_in_ _string_): If set to `false` the checkbox is disabled.
- **`enabled`** (_in_ _bool_): If set to `false` the checkbox is disabled.
- **`checked`** (_in-out_ _bool_): Whether the checkbox is checked or not (default: false).
- **`style`** (_in_ _[SwitchStyle](../foundation/check_box_base.md)_): Defines the style of the checkbox.

### Callbacks

- **`toggled(/* checked */ bool)`**: The `checked` value changed.

### Example

```slint
import { Switch } from "@vivi/magic.slint";
export component Example inherits Window {
    width: 200px;
    height: 25px;

    Switch {
        text: "Hello World";
    }
}
```
