<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `BorderStyle`

Defines the visual settings of a group box.

### Fields

- **`background`** (_brush_): Defines the brush of the background. (default value: black)
- **`border_brush`** (_brush_): Defines the border brush. (default value: black)
- **`border_radius`** (_length_): Defines the border radius size. (default value: 0)
- **`border_width`** (_length_): Defines the width of the border. (default value: 0)

## `Border`

`Border` is just a rectangle with a style.

### Properties

- **`background`** (_in_ _brush_): The background brush of this `Border`, typically a color. (default value: black)
- **`border_color`** (_in_ _brush_): The color of the border. (default value: `transparent`)
- **`border_radius`** (_in_ _length_): The size of the radius. (default value: 0)
- **`border_width`** (_in_ _length_): The width of the border. (default value: 0)
- **`clip`** (_in_ _bool_): By default, when an element is bigger or outside another element, it's still shown. When this property is set to `true`, the children of this `Rectangle` are clipped to the border of the rectangle. (default value: `false`)
- **`style`** (_in_ _`BorderStyle`_): Defines the style of the border.

### Example

```slint
import { Border } from "@vivi/foundation.slint";
export component Example inherits Window {
    width: 200px;
    height: 100px;
    Border {
      style: {
          background: #000000
      };
    }
}
```
