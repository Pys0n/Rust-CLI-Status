use std::io::Write;

pub enum SpinnerStyle {
    ASCII,
    ASCIILongDash,
    ASCIILongDashExtended,
    BrailleSpinner,
    BrailleSpinnerMinimalBig,
    BrailleSpinnerMinimalSmall,
    BrailleSpinnerSmooth,
}

pub struct Spinner {
    title: String,
    style: SpinnerStyle,
    finished_char: char,
    state: usize,
}

impl Spinner {
    /// Creates a new Spinner.
    pub fn new() -> Self {
        Self {
            title: String::from("Loading..."),
            style: SpinnerStyle::ASCII,
            finished_char: '✔',
            state: 0,
        }
    }

    /// Sets the title of the Spinner.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Sets the style of the Spinner to `style`.
    pub fn set_style(&mut self, style: SpinnerStyle) {
        self.style = style;
    }

    /// Sets the character shown when `.finish()` gets called.
    pub fn set_finished_char(&mut self, finished_char: char) {
        self.finished_char = finished_char;
    }

    /// Returns the title of the Spinner.
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Returns the style of the Spinner.
    pub fn get_style(&self) -> &SpinnerStyle {
        &self.style
    }

    /// Returns the character shown when `.finish()` gets called.
    pub fn get_finished_char(&self) -> char {
        self.finished_char
    }

    /// Resets the Spinner to it's starting state/character.
    pub fn reset(&mut self) {
        self.state = 0;
    }

    /// Updates the Spinner to it's next state and prints it.
    pub fn tick(&mut self) {
        let style = match self.style {
            SpinnerStyle::ASCII => "/-\\",
            SpinnerStyle::ASCIILongDash => "/—\\",
            SpinnerStyle::ASCIILongDashExtended => "/—\\|",
            SpinnerStyle::BrailleSpinner => "⠇⠋⠙⠸⠴⠦",
            SpinnerStyle::BrailleSpinnerMinimalBig => "⠏⠛⠹⠼⠶⠧",
            SpinnerStyle::BrailleSpinnerMinimalSmall => "⠋⠙⠚⠓",
            SpinnerStyle::BrailleSpinnerSmooth => "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
        };

        self.state += 1;

        print!("\r{} {}", style.chars().nth(self.state % (style.chars().count())).unwrap(), self.title);
        std::io::stdout().flush().unwrap();
    }

    /// Replaces the Spinner with the `finished_char` and prints it.
    pub fn finish(&mut self) {
        self.reset();

        print!("\r{} {}", self.finished_char, self.title);
        std::io::stdout().flush().unwrap();
    }
}