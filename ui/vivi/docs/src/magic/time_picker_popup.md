<!--
SPDX-FileCopyrightText: 2024 vivi developers <vivi-ui@tuta.io>
SPDX-License-Identifier: MIT
-->

### `Time`

Defines a time with hours, minutes, and seconds.

#### Fields

- **`hour`(int)**: The hour value (range from 0 to 23).
- **`minute`(int)**: The minute value (range from 1 to 59).
- **`second`(int)**: The second value (range form 1 to 59).

## `TimePickerStyle`

Defines the visual settings of a time picker.

### Fields

- **`border_style`** (_[BorderStyle](../foundation/border.md)_): Defines the style of the background border.
- **`layout_style`** (_[LayoutStyle](../foundation/vertical_layout_base.md)_): Defines the style of the layout.
- **`text_style`** (_[TextStyle](../foundation/text_base.md)_): Defines the style of the text.
- **`text_input_style`** (_[TextStyle](../foundation/text_base.md)_): Defines the style of text input elements.

## `TimePickerPopup`

Use the timer picker to select the time, in either 24-hour or 12-hour mode (AM/PM).

### Properties

- **`style`**: (_in_ _`TimePickerStyle`_): Defines the style of the timer picker.
- **`use_24-hour_format`**: (_in_ _bool_): If set to `true` 24 hours are displayed otherwise it is displayed in AM/PM mode. (default: system default, if cannot be determined then `true`)
- **`title`** (_in_ _string_): The text that is displayed at the top of the picker.
- **`initial_time`**: (_in_ struct _[`Time`](#struct-time)_): Set the initial displayed time.

### Callbacks

- **`canceled()`**: The cancel button was clicked.
- **`accepted(/* time */ Time)`** The ok button was clicked.

### Example

```slint
import { FilledButton, TimePickerPopup } from "@vivi/foundation.slint";
export component Example inherits Window {
    width: 600px;
    height: 600px;

    time_picker_button := FilledButton {
        text: @tr("Open TimePicker");

        clicked => {
            time_picker.show();
        }
    }

    time_picker := TimePickerPopup {
        canceled => {
            time-picker.close();
        }

        accepted(time) => {
            debug(time);
            time-picker.close();
        }
    }
}
```
