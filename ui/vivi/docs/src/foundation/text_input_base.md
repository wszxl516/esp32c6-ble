<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `TextInputStyle`

Defines the visual settings of a text input.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`selection_background`** (_brush_): Defines the background of the text selection: (default value: black)
- **`selection_foreground`** (_brush_): Defines the foreground of the text selection: (default value: black)
- **`icon_style`** (_[IconStyle](./icon_base.md)_): Defines the style of the prefix_icon.
- **`text_style`** (_[TextStyle](./text_base.md)_): Defines the style of the text input.
- **`placeholder_style`** (_[TextStyle](./text_base.md)_): Defines the style of the placeholder text.
- **`padding_left`** (_length_): Defines the padding on the left side of the text input layout. (default value: 0)
- **`padding_right`** (_length_): Defines the padding on the right side of the text input layout. (default value: 0)
- **`padding_top`** (_length_): Defines the padding on the top side of the text input layout. (default value: 0)
- **`padding_bottom`** (_length_): Defines the padding on the bottom side of the text input layout. (default value: 0)
- **`spacing`** (_length_): Defines the distance between elements in the text input layout. (default value: 0)
  text.
- **`layout_style`** (_[LayoutStyle](./vertical_layout_base.md)_): Defines the style of the layout.

## `TextInputBaste`

The `TextInputBaste` represents the base for text input components.

### Properties

- **`enabled`** (_in_ _bool_): If set to `false` the text input is disabled.
- **`has_error`** (_in_ _bool_): If set to `true` the text input is displayed in the error state.
- **`text`** (_in_out_ _string_): The text being edited.
- **`placeholder_text`** (_in_ _string_): A placeholder text being shown when there is no text in the text input.
- **`style`** (_in_ _TextInputStyle_): Defines the style of the text input.

### Callbacks

- **`accepted(/* text */ string)`**: Enter was pressed.
- **`edited(/* text */ string)`**: Emitted when the text has changed because the user modified it.
