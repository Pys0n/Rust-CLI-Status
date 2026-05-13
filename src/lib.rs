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

    pub fn set_progress(&mut self, progress: u32) {
        self.progress = progress;
    }

    pub fn set_maximum(&mut self, maximum: u32) {
        self.maximum = maximum;
    }

    pub fn set_length(&mut self, length: u8) {
        self.length = length;
    }

    pub fn set_filled_char(&mut self, filled_char: char, set_current_char: bool) {
        self.filled_char = filled_char;
        if set_current_char {
            self.current_char = filled_char;
        };
    }

    pub fn set_current_char(&mut self, current_char: char) {
        self.current_char = current_char;
    }

    pub fn set_empty_char(&mut self, empty_char: char) {
        self.empty_char = empty_char;
    }

    pub fn set_border_chars(&mut self, border_chars: [char; 2]) {
        self.border_chars = border_chars;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

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

    pub fn set_show_progress(&mut self, show_progress: bool) {
        self.show_progress = show_progress;
    }

    pub fn reset_progress(&mut self) {
        self.progress = 0;
    }

    pub fn increase_progress(&mut self, increasement: u32) {
        self.progress += increasement;
    }

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

    pub fn print(&self) {
        print!("\r{}", self.get_string());
        std::io::stdout().flush().unwrap();
    }
}