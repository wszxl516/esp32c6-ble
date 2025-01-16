<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `GroupBoxStyle`

Defines the visual settings of a group box.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`spacing`** (_length_): Defines the layout spacing. (default value: 0)
- **`padding`** (_length_): Defines the layout spacing. (default value: 0)
- **`title_horizontal_alignment`** (_length_): Defines the horizontal text alignment of the title. (default value: left)
- **`title_style`** (_[TextStyle](./text_base.md)_): Defines the style of the title.

## `GroupBoxBase`

The `GroupBoxBase` represents the base for group box elements that can be styled by using `GroupBoxStyle`.

### Properties

- **`title`** (_in_ _string_): Defines the text that is displayed on the top of the group box (default: "").
- **`style`** (_in_ _`GroupBoxStyle`_): Defines the style of the group box.

### Example

```slint
import { GroupBoxBase, TextBase } from "@vivi/foundation.slint";

export component MyGroupBox inherits GroupBoxBase {
    style: {
        background: black,
        border_radius: 8px,
        title_style: {
            foreground: black,
            font_size: 12px
        }
    };

    VerticalLayout {
        TextBase {
            style: root.title_style;
            text: root.title;
        }

        @children
    }
}

export component Example inherits Window {
    width: 200px;
    height: 100px;

    MyGroupBox {
        title: "Group";

        Rectangle {
            width: 60px;
            height: 60px;
            background: green;
        }
    }
}
```
