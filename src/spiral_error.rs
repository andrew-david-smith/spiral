use colored::Colorize;
use std::error;
use std::fmt;

pub struct SpiralError<'a> {
    pub error_text: &'a str,
    pub help_text: &'a str,
    pub line_text: String,
    pub line_number: usize,
    pub begin: usize,
    pub end: usize,
}

impl<'a> fmt::Display for SpiralError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt_for_display())
    }
}

impl<'a> fmt::Debug for SpiralError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt_for_display())
    }
}

impl<'a> SpiralError<'a> {
    fn fmt_for_display(&self) -> String {
        format!(
            "{}\n\nL{}: {}\n{}\n{}",
            self.error_text.yellow(),
            self.line_number,
            self.line_text,
            self.error_display().red(),
            self.help_text.green()
        )
    }

    fn error_display(&self) -> String {
        format!(
            "{}{}",
            " ".repeat(self.begin + 3 + self.length_of_line_number()),
            "^".repeat(self.end - self.begin + 1)
        )
    }

    fn length_of_line_number(&self) -> usize {
        format!("{}", self.line_number).len()
    }
}

impl<'a> error::Error for SpiralError<'a> {}
