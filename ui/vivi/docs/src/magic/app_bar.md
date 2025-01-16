<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `AppBar`

An `AppBar` can be placed at the top of an app and contains a leading button, title and extra content.

### Properties

- **`title`** (_in_ _string_): Defines the title of the app bar.
- **`leading_button_icon`** (_in_ _string_): Defines the icon of the leading button.
- **`style`** (_in_ _`AppBarStyle`_): Defines the style of the button.

### Callbacks

- **`leading_button_clicked()`**: Invoked when leading button clicked.

### Example

```slint
import { AppBar, IconButton, Icons, MagicWindow } from "@vivi/magic.slint";

export component MainWindow inherits MagicWindow {
  min_width: 400px;
  min_height: 600px;

  VerticalLayout {
    AppBar {
      leading_button_icon: Icons.add;
      title: "My app";

      IconButton {
        icon: Icons.add;
      }
    }

    Rectangle {}
  }
}
```
