<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `RangeStyle`

Defines the visual settings of a range based (maximum, minimum, value) widget.

### Fields

- **`bar_height`** (_length_): Defines the height of the background bar and the track bar. (default: 0)
- **`border_radius`** (_length_): The size of the radius for background bar and track bar. (default value: 0)
- **`background`** (_brush_): Defines the brush of the background. (default value: black)
- **`track_background`** (_brush_): Defines the brush of the track background. (default value: black)
- **`handle_background`** (_brush_): Defines the brush of a drag handle if available. (default value: black)
- **`handle_size`** (_length_): Defines the width and height of a drag handle if available. (default value: 0)

## `RangeBase`

The `RangeBase` represents the base for widgets based on a `maximum`, `minimum` and value like a `ProgressBar` or `Slider`.

### Properties

- **`value`** (_in_out_ _float_): The value. Defaults to the minimum.
- **`minimum`** (_in_out_ _float_): The minimum value (default: 0).
- **`maximum`** (_in_out_ _float_): The maximum value (default 100).
- **`progress`** (_out_ _float_): Returns the value mapped between range of 0 to 1 (default 0).
- **`range`** (_out_ _float_): Returns the range (maximum - minimum).
- **`style`** (_in_ _`RangeStyle`_): Defines the style of the widget.

### Example

```slint
import { RangeBase } from "@vivi/foundation.slint";

export component ProgressBar inherits RangeBase {
    in property <string> text <=> text_label.text;

    min-width: 100px;
    height: 8px;

    style: {
        background: black,
        border_radius: 4px,
        track_background: blue,
        track_border_radius: 4px,
    };

    background_layer := Rectangle {
        width: 100%;
        height: 100%;
        background: root.style.background;
        border_radius: root.style.border_radius;
    }

    track := Rectangle {
        x: 0;
        y: 0;
        width: root.width * (root.value - root.minimum) / (root.maximum - root.minimum);
        height: 100%;
        background: root.style.track_background;
        border_radius: root.style.track_border_radius;
    }
}

export component Example inherits Window {
    width: 200px;
    height: 100px;

    ProgressBar {
      value: 50;
    }
}
```
