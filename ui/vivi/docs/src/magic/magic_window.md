<!--
SPDX-FileCopyrightText: 2024 Vivi <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

## `MagicWindow`

`MagicWindow` is the root of the tree of elements that are visible on the screen. It's almost the same a `Slint`'s default window except it defines background and font settings
on the window that matches `coop`'s `pure` design.

The `MagicWindow` geometry will be restricted by its layout constraints: Setting the `width` will result in a fixed width,
and the window manager will respect the `min_width` and `max_width` so the window can't be resized bigger
or smaller. The initial width can be controlled with the `preferred_width` property. The same applies to the `Window`s height.

### Properties

- **`accent_background`** (_in_out_ _brush_): Defines the background brush for highlighted controls such as primary buttons.
- **`always_on_top`** (_in_ _bool_): Whether the window should be placed above all other windows on window managers supporting it.
- **`background`** (_in_ _brush_): The background brush of the `Window`. (default value: depends on the style)
- **`default_font_family`** (_in_ _string_): The font family to use as default in text elements inside this window, that don't have their `font_family` property set.
- **`default_font_size`** (_in_out_ _length_): The font size to use as default in text elements inside this window, that don't have their `font_size` property set. The value of this property also forms the basis for relative font sizes.
- **`default_font_weight`** (_in_ _int_): The font weight to use as default in text elements inside this window, that don't have their `font_weight` property set. The values range from 100 (lightest) to 900 (thickest). 400 is the normal weight.
- **`icon`** (_in_ _image_): The window icon shown in the title bar or the task bar on window managers supporting it.
- **`no_frame`** (_in_ _bool_): Whether the window should be borderless/frameless or not.
- **`resize_border`** (_in_ _length_): Size of the resize border in borderless/frameless windows (winit only for now).
- **`title`** (_in_ _string_): The window title that is shown in the title bar.
