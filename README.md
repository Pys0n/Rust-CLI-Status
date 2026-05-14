# CLI Progress Bar

A customizable Progress Bar for the Rust CLI.

## Documentation

To use the Progress Bar use:
```rust
use cli_status::{ProgressBar, ProgressBarStyle}
```
or always type `cli_status::` infront of `ProgressBar` and `ProgressBarStyle`

To use the Spinner use:
```rust
use cli_status::{Spinner, SpinnerStyle}
```
or always type `cli_status::` infront of `Spinner` and `SpinnerStyle`

### ProgressBar

You can create a new ProgressBar by using this code:
```rust
let mut pg_bar = cli_status::ProgressBar::new();
```

Or this code:
```rust
use cli_status::ProgressBar

let mut pg_bar = ProgressBar::new();
```


#### ProgressBar.get_border_chars() -> [char; 2];
Returns the border characters of the Progress Bar.


#### ProgressBar.get_current_char() -> char;
Returns the last character of the filled part of the Progress Bar.


#### ProgressBar.get_empty_char() -> char;
Returns the character of the empty part of the Progress Bar.


#### ProgressBar.get_filled_char() -> char;
Returns the character of the filled part of the Progress Bar.


#### ProgressBar.get_length() -> u8;
Returns the length of the Progress Bar (without title and border).


#### ProgressBar.get_maximum() -> u32;
Returns the maximum Progress of the Progress Bar.


#### ProgressBar.get_progress() -> u32;
Returns the Progress of the Progress Bar.


#### ProgressBar.get_show_progress() -> bool;
Returns the current state of `show_progress`.


#### ProgressBar.get_string() -> String;
Returns the Progress Bar as a String.


#### ProgressBar.get_title() -> &String;
Returns the title of the Progress Bar.


#### ProgressBar.increase_progress(increasement: u32);
Increases the progress by `increasement`.


#### ProgressBar::new() -> Self;
Creates a new Progress Bar with the following settings:

```
length: 10,
progress: 0,
maximum: 100,
title: String::from(""),
filled_char: '#',
current_char: '#',
empty_char: '-',
border_chars: ['|', '|'],
show_progress: false,
```


#### ProgressBar.print();
Prints the Progress Bar to the CLI.


#### ProgressBar.reset_progress();
Resets the Progress to 0.


#### ProgressBar.set_border_chars(border_chars: [char; 2]);
Sets the Border of the Progress Bar to the items in `border_chars`.

The first character in the array represents the first border and the second character the second border.


#### ProgressBar.set_current_char(current_char: char);
Sets the last character of the filled part of the Progress Bar to `current_char`.

The last character of the filled part is by default the same as the `filled_char`.


#### ProgressBar.set_empty_char(empty_char: char);
Sets the character of the not filled part of the Progress Bar to `empty_char`.


#### ProgressBar.set_filled_char(filled_char: char, set_current_char: bool);
Sets the character of the filled part of the ProgressBar to `filled_char`.

If `set_current_char` is set to `true`, the `current_char` will also be set to `filled_char`.


#### ProgressBar.set_length(length: u8);
Sets the length of the Progress Bar (without title and border).


#### ProgressBar.set_maximum(maximum: u32);
Sets the maximum progress of the Progress Bar.

If the Progress reaches this maximum, the Progress Bar is full.
The progress can be higher than `maximum`!


#### ProgressBar.set_progress(progress: u32);
Sets the Progress of the Progress Bar.


#### ProgressBar.set_show_progress(show_progress: bool);
Enables/Disables the Progress Text behind the Progress Bar.

If `show_progress` is set to `true`:
```
Title |#######---| 70/100
```
Otherwise:
```
Title |#######---|
```


#### ProgressBar.set_style(style: ProgressBarStyle);
Sets the style of the ProgressBar to one of the `ProgressBarStyle`s.


#### ProgressBar.set_title(title: String);
Sets the title of the Progress Bar.

If set to `""`, the title, and the automatic generated space after it, will not appear.


### ProgressBarStyle

There are currently three styles:

`ProgressBarStyle::Block`:
```
Title |███████   | 70/100
```


`ProgressBarStyle::DoubleArrow`:
```
Title [======>   ] 70/100
```


`ProgressBarStyle::Standart` *(default)*:
```
Title |#######---| 70/100
```

Don't forget to use this line of code:
```rust
use cli_status::ProgressBarStyle
```
Otherwise you would need to write `cli_status::ProgressBarStyle`.


### Spinner


You can create a new Spinner by using this code:
```rust
let mut spinner = cli_status::Spinner::new();
```

Or this code:
```rust
use cli_status::Spinner

let mut spinner = Spinner::new();
```


#### Spinner.finish();
Replaces the Spinner with the `finished_char` and prints it.


#### Spinner.get_finished_char() -> char;
Returns the character shown when `.finish()` gets called.


#### Spinner.get_style() -> &SpinnerStyle;
Returns the style of the Spinner.


#### Spinner.get_title() -> &String;
Returns the title of the Spinner.


#### Spinner.reset();
Resets the Spinner to it's starting state/character.


#### Spinner.set_finished_char(finished_char: char);
Sets the character shown when `.finish()` gets called.
`finished_char` is by default set to `'✔'`.


#### Spinner.set_style(style: SpinnerStyle);
Sets the style of the Spinner to `style`.


#### Spinner.set_title(title: String);
Sets the title of the Spinner.


#### Spinner.tick();
Updates the Spinner to it's next state and prints it.


### SpinnerStyle

There are currently seven styles:

`SpinnerStyle::ASCII`:
```
/
-
\
```

`SpinnerStyle::ASCIILongDash`:
```
/
—
\
```

`SpinnerStyle::ASCIILongDashExtended`:
```
/
—
\
|
```

`SpinnerStyle::BrailleSpinner`:
```
⠇
⠋
⠙
⠸
⠴
⠦
```

`SpinnerStyle::BrailleSpinnerMinimalBig`:
```
⠏
⠹
⠼
⠧
```

`SpinnerStyle::BrailleSpinnerMinimalSmall`:
```
⠋
⠙
⠚
⠓
```

`SpinnerStyle::BrailleSpinnerSmooth`:
```
⠋
⠙
⠹
⠸
⠼
⠴
⠦
⠧
⠇
⠏
```

Don't forget to use this line of code:
```rust
use cli_status::SpinnerStyle
```
Otherwise you would need to write `cli_status::SpinnerStyle`.
