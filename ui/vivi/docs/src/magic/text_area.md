<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `TextArea`

The `TextArea` used to enter a multi line text.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the text field is disabled.
- **`read_only`** (_in_ _bool_): When set to `true`, text editing via keyboard and mouse is disabled but selecting text is still enabled as well as editing text programmatically (default value: `false`)
- **`wrap`** (_in_ _`TextWrap`_): The way the text wraps (default: word-wrap).
- **`horizontal_alignment`** (_in_ _`TextHorizontalAlignment`_): The horizontal alignment of the text.
- **`has_error`** (_in_ _bool_): If set to `true` the text field is displayed in the error state.
- **`has_focus`**: (_out_ _bool_): Set to true when the text field currently has the focus.
- **`text`** (_in_out_ _string_): The text being edited.
- **`placeholder_text`** (_in_ _string_): A placeholder text being shown when there is no text in the text field.
- **`style`** (_in_ _TextInputStyle_): Defines the style of the text field.

### Callbacks

_ **`accepted(/* text */ string)`**: Enter was pressed.
_ **`edited(/* text */ string)`**: Emitted when the text has changed because the user modified it.

### Functions

- **`focus()`** Call this function to focus the text field and make it receive future keyboard events.
- **`clear_focus()`** Call this function to remove keyboard focus from this `LineEdit` if it currently has the focus.
- **`set_selection_offsets(int, int)`** Selects the text between two UTF_8 offsets.
- **`select_all()`** Selects all text.
- **`clear_selection()`** Clears the selection.
- **`copy()`** Copies the selected text to the clipboard.
- **`cut()`** Copies the selected text to the clipboard and removes it from the editable area.
- **`paste()`** Pastes the text content of the clipboard at the cursor position.

### Example

```slint
import { TextArea } from "@vivi/magic.slint";
export component Example inherits Window {
    width: 200px;
    height: 64px;
    TextArea {
        width: parent.width;
        height: parent.height;
        placeholder_text: "Enter text here";
    }
}
```
