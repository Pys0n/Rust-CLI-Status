use std::io::Write;

pub enum PgBarStyle {
    Blocks,
    DoubleArrow,
    Standart,
}

pub struct ProgressBar {
    length: u8,
    progress: u32,
    maximum: u32,
    title: String,
    filled_char: char,
    current_char: char,
    empty_char: char,
    border_chars: [char; 2],
    show_progress: bool,
}

impl ProgressBar {
    /// Creates a new Progress Bar.
    pub fn new() -> Self {
        ProgressBar {
            length: 10,
            progress: 0,
            maximum: 100,
            title: String::from(""),
            filled_char: '#',
            current_char: '#',
            empty_char: '-',
            border_chars: ['|', '|'],
            show_progress: false,
        }
    }

    /// Sets the Progress of the Progress Bar. 
    pub fn set_progress(&mut self, progress: u32) {
        self.progress = progress;
    }

    /// Sets the maximum progress of the Progress Bar.
    pub fn set_maximum(&mut self, maximum: u32) {
        self.maximum = maximum;
    }

    /// Sets the length of the Progress Bar (without title and border).
    pub fn set_length(&mut self, length: u8) {
        self.length = length;
    }

    /// Sets the character of the filled part of the ProgressBar to `filled_char`.
    pub fn set_filled_char(&mut self, filled_char: char, set_current_char: bool) {
        self.filled_char = filled_char;
        if set_current_char {
            self.current_char = filled_char;
        };
    }

    /// Sets the last character of the filled part of the Progress Bar to `current_char`.
    pub fn set_current_char(&mut self, current_char: char) {
        self.current_char = current_char;
    }

    /// Sets the character of the not filled part of the Progress Bar to `empty_char`.
    pub fn set_empty_char(&mut self, empty_char: char) {
        self.empty_char = empty_char;
    }

    /// Sets the Border of the Progress Bar to the items in `border_chars`.
    pub fn set_border_chars(&mut self, border_chars: [char; 2]) {
        self.border_chars = border_chars;
    }

    /// Sets the title of the Progress Bar.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Sets the style of the ProgressBar to one of the `PgBarStyle`s.
    pub fn set_style(&mut self, style: PgBarStyle) {
        match style {
            PgBarStyle::Blocks => {
                self.set_border_chars(['|', '|']);
                self.set_empty_char(' ');
                self.set_filled_char('█', true);
            },
            PgBarStyle::DoubleArrow => {
                self.set_border_chars(['[', ']']);
                self.set_current_char('>');
                self.set_empty_char(' ');
                self.set_filled_char('=', false);
            },
            PgBarStyle::Standart => {
                self.set_border_chars(['|', '|']);
                self.set_empty_char('-');
                self.set_filled_char('#', true);
            },
        }
    }

    /// Enables/Disables the Progress Text behind the Progress Bar. 
    pub fn set_show_progress(&mut self, show_progress: bool) {
        self.show_progress = show_progress;
    }

    /// Resets the Progress to 0.
    pub fn reset_progress(&mut self) {
        self.progress = 0;
    }

    /// Increases the progress by `increasement`.
    pub fn increase_progress(&mut self, increasement: u32) {
        self.progress += increasement;
    }

    /// Returns the length of the Progress Bar (without title and border).
    pub fn get_length(&self) -> u8 {
        self.length
    }

    /// Returns the Progress of the Progress Bar.
    pub fn get_progress(&self) -> u32 {
        self.progress
    }

    /// Returns the maximum Progress of the Progress Bar.
    pub fn get_maximum(&self) -> u32 {
        self.maximum
    }

    /// Returns the title of the Progress Bar.
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Returns the character of the filled part of the Progress Bar.
    pub fn get_filled_char(&self) -> char {
        self.filled_char
    }

    /// Returns the character of the empty part of the Progress Bar.
    pub fn get_empty_char(&self) -> char {
        self.empty_char
    }

    /// Returns the last character of the filled part of the Progress Bar.
    pub fn get_current_char(&self) -> char {
        self.current_char
    }

    /// Returns the border characters of the Progress Bar.
    pub fn get_border_chars(&self) -> [char; 2] {
        self.border_chars
    }

    /// Returns the current state of `show_progress`.
    pub fn get_show_progress(&self) -> bool {
        self.show_progress
    }

    /// Returns the Progress Bar as a String.
    pub fn get_string(&self) -> String {
        let mut pg_bar_str = self.title.clone();
        if self.title.len() > 0 {
            pg_bar_str += &String::from(" ")
        }
        pg_bar_str += &String::from(self.border_chars[0]);
        for i in 1..self.length+1 {
            if f64::from(i) <= (f64::from(self.progress) / f64::from(self.maximum) * f64::from(self.length)) {
                if f64::from(i+1) <= (f64::from(self.progress) / f64::from(self.maximum) * f64::from(self.length)) {
                    pg_bar_str += &String::from(self.filled_char);
                } else {
                    pg_bar_str += &String::from(self.current_char);
                }
            } else {
                pg_bar_str += &String::from(self.empty_char);
            };
        };
        pg_bar_str += &String::from(self.border_chars[1]);
        if self.show_progress {
            pg_bar_str += &String::from(' ');
            pg_bar_str += &String::from(self.progress.to_string());
            pg_bar_str += &String::from('/');
            pg_bar_str += &String::from(self.maximum.to_string());
        }
        return pg_bar_str; 
    }

    /// Prints the Progress Bar to the CLI.
    pub fn print(&self) {
        print!("\r{}", self.get_string());
        std::io::stdout().flush().unwrap();
    }
}