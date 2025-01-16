<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

# `MagicIcon`

The `MagicIcon` represents an icon element that matches the settings of the `magic` style.

### Properties

- **`style`** (_in_ _[`IconStyle`](../foundation/icon_base.slint)_): Used to set `color` and `height` of the icon.
- **`icon`** (_in_ _image_): The image source of the icon.

### Example

```slint
import { MagicIcon } from "@vivi/magic.slint";

export component Example inherits Window {
    width: 200px;
    height: 100px;

    MagicIcon {
        icon: @image-url("my-icon.svg");
    }
}
```
