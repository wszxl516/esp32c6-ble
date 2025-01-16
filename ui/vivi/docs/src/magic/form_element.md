<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `FormElement`

A `FormElement` is a container that groups its children together under a common title without background.

### Properties

- **`title`** (_in_ _string_): A text written as the title of the form element.
- **`style`** (_in_ _[FormElementStyle](../foundation/form_element_base.md)_): Defines the style of the form element.

### Example

```slint
import { FormElement, FilledButton } from "@vivi/magic.slint";
export component Example inherits Window {
    width: 200px;
    height: 100px;
    FormElement {
        title: "Button";
        FilledButton {
          text: "Click me";
        }
    }
}
```
