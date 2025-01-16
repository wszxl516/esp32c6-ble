<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

# `IconStyle`

Defines the `color` and `icon_size` of an icon.

### Fields

- **`foreground`** (_brush_): Defines the color of the icon. (default value: black)
- **`icon_size`** (_relative_font_size_): Defines the height of the icon. (default value: 0)

# `IconBase`

The `IconBase` represents the base for icon elements that can be styled by using `IconStyle`.

### Properties

- **`style`** (_in_ _IconStyle_): Used to set `color` and `height` of the icon.
- **`icon`** (_in_ _image_): The image source of the icon.

### Example

```slint
import { IconBase } from "@vivi/foundation.slint";

export component MyIcon inherits IconBase {
    style: {
        foreground: black,
        icon_size: 12px,
    };
}

export component Example inherits Window {
    width: 200px;
    height: 100px;

    MyIcon {
        icon: @image-url("my-icon.svg");
    }
}
```
