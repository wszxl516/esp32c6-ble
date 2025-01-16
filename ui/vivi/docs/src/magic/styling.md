<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

# `MagicPalette` struct

This struct can be used to customize the application brush palette.

- **`background`**: fines the default background brush. Use this if none of the more specialized background brushes apply.
- **`foreground`**: Defines the foreground brush that is used for content that is displayed on `background` brush.
- **`placeholder_foreground`**: Defines the foreground brush that is used for placeholder content.
- **`alternate_background`**: Defines an alternate background brush that is used for example for text input controls or panels like a side bar.
- **`control_background`**: Defines the default background brush for controls, such as push buttons, combo boxes, etc.
- **`control_foreground`**: Defines the foreground brush that is used for content that is displayed on `control_background` brush.
- **`accent_background`**: Defines the background brush for highlighted controls such as primary buttons.
- **`accent_foreground`**: Defines the foreground brush that is used for content that is displayed on `accent-background` brush.
- **`alternate_accent_background`**: Defines an alternate accent background brush.
- **`destructive_background`**: Defines the background brush for destructive controls such as destructive buttons.
- **`destructive_foreground`**: Defines the foreground brush that is used for content that is displayed on accent-background brush.
- **`border`**: Defines the brush that is used for borders such as separators and component borders.
- **`focus_border`**: Defines brush of focus borders.
- **`state_pressed`**: Defines the brush of the state overlay if the component is pressed.
- **`state_hovered`**: Defines the brush of the state overlay if the component is hovered by the mouse pointer.
- **`selection_background`**: Defines the background brush of a text selection.

# `Flavor` enum

Defines one of the four flavors of the [Catppuccin](https://github.com/catppuccin/catppuccin) theme.

# `CatppuccinPalette` struct

Used to define the palette of one of the [Catppuccin's](https://github.com/catppuccin/catppuccin) flavors.

For details check https://catppuccin.com/palette.

# `Catppuccin` global

Give access to all four flavor palettes of [Catppuccin](https://github.com/catppuccin/catppuccin).

- **`latte`** (_out_ _`CatppuccinPalette`_): Defines the palette of the Latte flavor.
- **`frappe`** (_out_ _`CatppuccinPalette`_): Defines the palette of the Frappe flavor.
- **`macchiato`** (_out_ _`CatppuccinPalette`_): Defines the palette of the Macchiato flavor.
- **`mocha`** (_out_ _`CatppuccinPalette`_): Defines the palette of the Mocha flavor.
- **`current_flavor`** (_int_ _`Flavor`_): Defines the current selected flavor (default: depending on system settings, on dark it is macchiato, on light it is latte)
- **`current_palette`** (_out_ _`CatppuccinPalette`_): Give access to the current selected catppuccin palette.
- **`current_magic_palette`** (_out_ _`MagicPalette`_): Give access to the current selected catppuccin palette converted to a magic palette.

# `Palette` global

Use `Palette` to create custom components that are matches the design of the `magic` component set. The default platte is based on the [Catppuccin](https://github.com/catppuccin/catppuccin) theme.

### Properties

- **`current_palette`** (_in_ _`MagicPalette`_): defines the current application brush palette. (default: on system setting to dark default it is catppuccin macchiato otherwise latte)
- **`background`** (_out_ _brush_): fines the default background brush. Use this if none of the more specialized background brushes apply.
- **`foreground`** (_out_ _brush_): Defines the foreground brush that is used for content that is displayed on `background` brush.
- **`placeholder_foreground`** (_out_ _brush_): Defines the foreground brush that is used for placeholder content.
- **`alternate_background`** (_out_ _brush_): Defines an alternate background brush that is used for example for text input controls or panels like a side bar.
- **`control_background`** (_out_ _brush_): Defines the default background brush for controls, such as push buttons, combo boxes, etc.
- **`control_foreground`** (_out_ _brush_): Defines the foreground brush that is used for content that is displayed on `control_background` brush.
- **`accent_background`** (_in_out_ _brush_): Defines the background brush for highlighted controls such as primary buttons.
- **`accent_foreground`** (_out_ _brush_): Defines the foreground brush that is used for content that is displayed on `accent-background` brush.
- **`alternate_accent_background`** (_out_ _brush_): Defines an alternate accent background brush.
- **`destructive_background`** (_out_ _brush_): Defines the background brush for destructive controls such as destructive buttons.
- **`destructive_foreground`** (_out_ _brush_): Defines the foreground brush that is used for content that is displayed on accent-background brush.
- **`border`** (_out_ _brush_): Defines the brush that is used for borders such as separators and component borders.
- **`focus_border`** (_out_ _brush_): Defines brush of focus borders.
- **`state_pressed`** (_out_ _brush_): Defines the brush of the state overlay if the component is pressed.
- **`state_hovered`** (_out_ _brush_): Defines the brush of the state overlay if the component is hovered by the mouse pointer.
- **`selection_background`** (_out_ _brush_): Defines the background brush of a text selection.

# `FontSettings` global

Use `FontSettings` to create custom components that matches the font settings of the `magic` style.

### Properties

- **`light_font_weight`** (_out_ _int_): Defines the light font weight of the `magic` style.
- **`regular_font_weight`** (_out_ _int_): Defines the regular font weight of the `magic` style.
- **`semi_bold_font_weight`** (_out_ _int_): Defines the semi bold font weight of the `magic` style.
- **`light`** (_out_ _[`TextStyle`](./text_base.md)_): Defines a text style for a sub text.
- **`supporting`** (_out_ _[`TextStyle`](./text_base.md)_): Defines a text style for a supporting text.
- **`body`** (_out_ _[`TextStyle`](./text_base.md)_): Defines the body text style of `magic`.
- **`body_strong`** (_out_ _[`TextStyle`](./text_base.md)_): Defines body text style with more bold font.
- **`header_1`** (_out_ _[`TextStyle`](./text_base.md)_): Defines a header text style first order.
- **`header_2`** (_out_ _[`TextStyle`](./text_base.md)_): Defines a header text style second order .
- **`header_3`** (_out_ _[`TextStyle`](./text_base.md)_): Defines a header text style third order.

# `IconSettings` global

Use `IconSettings` to create custom components that matches the icon settings of the `magic` style.

### Properties

- **`body`** (_out_ _[`IconStyle`](./icon_base.md)_): Defines the body icon style of `magic`.

# `AnimationSettings` global

Use `AnimationSettings` to create animations for custom components that matches the animation settings of the `magic` style.

### Properties

- **`color_duration`** (_out_ _duration_): Defines the animation duration for color animations.
- **`resize_duration`** (_out_ _duration_): Defines the animation duration of resizing of an element.
- **`move_in_duration`** (_out_ _duration_): Defines the animation duration of elements entering the screen.
- **`move_out_duration`** (_out_ _duration_): Defines the animation duration of elements exit the screen.
- **`move_in_easing`** (_out_ _easing_): Defines the animation easing of elements entering the screen.
- **`move_out_easing`** (_out_ _easing_): Defines the animation easing of elements exit the screen.

# `SizeSettings` global

Use `SizeSettings` to define sizes of custom components that matches the settings of the `magic` style.

### Properties

- **`mobile_size`** (_in_out_ _bool_): If set to `true` the size settings are increased to make the components better touchable on mobile. (default value: false)
- **`box_height`** (_out_ _length_): Defines the height of box elements like checkbox.
- **`item_height`** (_out_ _length_): Defines the height of list item elements.
- **`control_height`** (_out_ _length_): Defines the default height of controls like buttons.
- **`app_bar_height`** (_out_ _length_): Defines the min height of the app bar.
- **`min_text_field_width`** (_out_ _length_): Defines the min width of text input elements.
- **`min_text_area_height`** (_out_ _length_): Defines the min height of a text area.

# `LayoutSettings` global

Use `LayoutSettings` to define padding and spacing of custom components that matches the settings of the `magic` style.

### Properties

- **`layout_spacing`** (_out_ _length_): Defines the default inner spacing of layouts.
- **`layout_padding`** (_out_ _length_): Defines the default padding of layouts.
- **`alternate_layout_padding`** (_out_ _length_): Defines the alternate padding of layouts.
- **`control_spacing`** (_out_ _length_): Defines the default inner spacing of elements inside of a component.
- **`control_padding`** (_out_ _length_): Defines the default padding of a component.
- **`text_input_padding`** (_out_ _length_): Defines the default padding of a text input components.
- **`control_style`** (_out_ _[LayoutStyle](../foundation/vertical_layout_base.md)_): Defines the default layout style of controls / components.
- **`control_style`** (_out_ _[LayoutStyle](../foundation/vertical_layout_base.md)_): Defines the default layout style of e.g. `AppBar`.
- **`text_input_style`** (_out_ _[LayoutStyle](../foundation/vertical_layout_base.md)_): Defines the default layout style of text input elements.

# `BorderSettings` global

Use `BorderSettings` to define border setting of custom components that matches the `magic` style.

### Properties

- **`bar_border_radius`** (_out_ _length_): Defines the border radius of bar elements like checkbox.
- **`box_border_radius`** (_out_ _length_): Defines the border radius of box elements like checkbox.
- **`control_border_radius`** (_out_ _length_): Defines the default border radius of elements inside of a component.
- **`control_border_width`** (_out_ _length_): Defines the default border width of elements inside of a component.
- **`alternate_border_style`** (_out_ _[BorderStyle](../foundation/border.slint)_): Defines the style of a background border with a alternate background color.
- **`popup_border_style`** (_out_ _[BorderStyle](../foundation/border.slint)_): Defines the style of a popup border.
