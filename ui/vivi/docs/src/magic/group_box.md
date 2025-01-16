<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `GroupBox`

A `GroupBox` is a container that groups its children together under a common title.

### Properties

- **`title`** (_in_ _string_): A text written as the title of the group box.
- **`style`** (_in_ _[GroupBoxStyle](../foundation/group_box_base.md)_): Defines the style of the group box.

### Example

```slint
import { GroupBox , MagicVerticalBox, CheckBox } from "@vivi/magic.slint";
export component Example inherits Window {
    width: 200px;
    height: 100px;
    GroupBox {
        title: "Groceries";
        VerticalLayout {
            CheckBox { text: "Bread"; checked: true ;}
            CheckBox { text: "Fruits"; }
        }
    }
}
```
