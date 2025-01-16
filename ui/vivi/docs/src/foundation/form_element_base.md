<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `FormElementStyle`

Defines the visual settings of a form element.

### Fields

- **`title_horizontal_alignment`** (_length_): Defines the horizontal text alignment of the title. (default value: left)
- **`title_style`** (_[`TextStyle`](./text_base.md)_): Defines the style of the title.

# `FormElementBase`

The `FormElementBase` represents the base for for elements that can be styled by using `FormElementStyle`.

### Properties

- **`title`** (_in_ _string_): A text written as the title of the form element.
- **`style`** (_in_ _[FormElementStyle](../foundation/form_element_base.md)_): Defines the style of the form element.

### Example

```slint
import { FormElementBase, TextBase } from "@vivi/foundation.slint";

export component MyFormElement inherits FormElementBase {
    style: {
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

    MyFormElement {
        title: "Element";

        Rectangle {
            width: 60px;
            height: 60px;
            background: green;
        }
    }
}
```
