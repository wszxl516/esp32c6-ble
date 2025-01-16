<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `AppBarStyle`

Defines the visual settings of a button.

### Fields

- **`border_style`** (_[BorderStyle](./border.md)_): Defines the style of the background border.
- **`text_style`** (_[TextStyle](./text_base.md)_): Defines the style of the text.
- **`leading_button_style`** (_[ButtonStyle](./button_base.md)_): Defines the style of leading button.
- **`layout_style`** (_[LayoutStyle](./vertical_layout_base.md)_): Defines the style of the layout.

## `AppBarBase`

The `AppBarBase` represents the base for app bar elements that can be styled by using `AppBarStyle`.

### Properties

- **`title`** (_in_ _string_): Defines the title of the app bar.
- **`leading_button_icon`** (_in_ _string_): Defines the icon of the leading button.
- **`style`** (_in_ _`AppBarStyle`_): Defines the style of the button.

### Callbacks

- **`leading_button_clicked()`**: Invoked when leading button clicked.
