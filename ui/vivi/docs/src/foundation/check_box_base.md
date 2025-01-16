<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `CheckBoxStyle`

Defines the visual settings of a checkbox.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`box_style`** (_[BorderStyle](./border.md)_): Defines the style of the check box.
- **`box_icon_style`** (_[IconStyle](./icon_base.md)_): Defines the style of the check box icon.
- **`text_style`** (_[TextStyle](./text_base.md)_): Defines the style of the text.
- **`spacing`** (_length_): Defines the distance between elements in the checkbox layout. (default value: 0)
- **`layout_style`** (_[LayoutStyle](./vertical_layout_base.md)_): Defines the style of the layout.

# `CheckBoxBase`

The `CheckBoxBase` represents the base for checkbox elements that can be styled by using `CheckBoxStyle`.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the checkbox is disabled.
- **`checked`** (_in-out_ _bool_): Whether the checkbox is checked or not (default: false).
- **`style`** (_in_ _`CheckBoxStyle`_): Defines the style of the checkbox.

### Callbacks

- **`toggled(/* checked */ bool)`**: The `checked` value changed.

### Example

```slint
import { CheckBoxBase } from "@vivi/foundation.slint";

export component MyCheckBox inherits CheckBoxBase {
    width: 20px;
    height: 20px;

    style: {
        background: black,
        border_radius: 8px,
    };
}

export component Example inherits Window {
    width: 200px;
    height: 100px;

    MyCheckBox {
        toggled(checked) => {
            debug(checked);
        }
    }
}
```
