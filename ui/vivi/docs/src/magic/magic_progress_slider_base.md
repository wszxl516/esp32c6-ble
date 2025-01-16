<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

# `MagicProgressSliderBase`

The `MagicProgressSliderBase` represents the base for widgets that can be used as progressbar and slider.

### Properties

- **`value`** (_in_out_ _float_): The value. Defaults to the minimum.
- **`minimum`** (_in_out_ _float_): The minimum value (default: 0).
- **`maximum`** (_in_out_ _float_): The maximum value (default 100).
- **`progress`** (_out_ _float_): Returns the value mapped between range of 0 to 1 (default 0).
- **`range`** (_out_ _float_): Returns the range (maximum - minimum).
- **`style`** (_in_ _[RangeStyle](../foundation/range_base.md)_): Defines the style of the widget.
- **`value`** (_in_out_ _float_): The value. Defaults to the minimum.
- **`indeterminate`** (_in_ _bool_): Set to `true` if the progress of the operation cannot be determined by value (default value: `false`).
- **`interactive`** (_in_ _bool_): Set to `true` to display the widget as slider (default value: `false`).
- **`enabled`** (_in_ _bool_): If set to `false` the widget is disabled.
- **`has_focus`** (_out_ _bool_): Is true when the element has keyboard focus.
- **`step_size`** (_in_ _float_): The increment / decrement value on keyboard operations (default 1).

### Callbacks

- **`edited(/* value */ float)`**: Invoked when value is changed by user interaction.
