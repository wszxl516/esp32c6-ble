<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

# `TextStyle`

Defines the `color`, `font_weight` and `font_size` of a text.

### Fields

- **`foreground`** (_brush_): Defines the color of the text. (default value: black)
- **`font_size`** (_length_): Defines the font size of the text. (default value: 0)
- **`font_weight`** (_int_): Defines the font weight of the text. (default value: 0)

# `TextBase`

The `TextBase` represents the base for text elements that can be styled by using `TextStyle`. It inherits the [`Text`](https://releases.slint.dev/1.6.0/docs/slint/src/language/builtins/elements.html#text) element.

### Properties

- **`style`** (_in_ _TextStyle_): Used to set `color`, `font_weight` and `font_size` of the text.
- **`color`** (_in_ _brush_): The color of the text. (default value: depends on the style)
- **`font_family`** (_in_ _string_): The name of the font family selected for rendering the text.
- **`font_size`** (_in_ _length_): The font size of the text.
- **`font_weight`** (_in_ _int_): The weight of the font. The values range from 100 (lightest) to 900 (thickest). 400 is the normal weight.
- **`font_italic`** (_in_ _bool_): Whether or not the font face should be drawn italicized or not. (default value: false)
- **`horizontal_alignment`** (_in_ _enum [`TextHorizontalAlignment`](https://releases.slint.dev/1.6.0/docs/slint/src/language/builtins/enums#texthorizontalalignment)_): The horizontal alignment of the text.
- **`letter_spacing`** (_in_ _length_): The letter spacing allows changing the spacing between the glyphs. A positive value increases the spacing and a negative value decreases the distance. (default value: 0)
- **`overflow`** (_in_ _enum [`TextOverflow`](https://releases.slint.dev/1.6.0/docs/slint/src/language/builtins/enums#textoverflow)_): What happens when the text overflows (default value: clip).
- **`text`** (_in_ _string_): The text rendered.
- **`vertical_alignment`** (_in_ _enum [`TextVerticalAlignment`](https://releases.slint.dev/1.6.0/docs/slint/src/language/builtins/enums#textverticalalignment)_): The vertical alignment of the text.
- **`wrap`** (_in_ _enum [`TextWrap`](https://releases.slint.dev/1.6.0/docs/slint/src/language/builtins/enums#textwrap)_): The way the text wraps (default value: `no_wrap`).
- **`stroke`** (_in_ _brush_): The brush used for the text outline (default value: `transparent`).
- **`stroke_width`** (_in_ _length_): The width of the text outline. If the width is zero, then a hairline stroke (1 physical pixel) will be rendered.
- **`stroke_style`** (_in_ _enum [`TextStrokeStyle`](https://releases.slint.dev/1.6.0/docs/slint/src/language/builtins/enums#textstrokestyle)_): The style/alignment of the text outline (default value: `outside`).

### Example

```slint
import { TextBase } from "@vivi/foundation.slint";

export component MyText inherits TextBase {
    style: {
        foreground: black,
        font_size: 12px,
        font_weight: 300
    };
}

export component Example inherits Window {
    width: 200px;
    height: 100px;

    MyText {
        text: "hello world";
    }
}
```
