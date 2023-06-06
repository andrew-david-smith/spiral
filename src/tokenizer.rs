use colored::Colorize;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct SpiralError<'a> {
    error_text: &'a str,
    help_text: &'a str,
    file: String,
    line_number: usize,
    begin: usize,
    end: usize,
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
            self.problematic_code(),
            self.error_display().red(),
            self.help_text.green()
        )
    }

    fn problematic_code(&self) -> String {
        self.file
            .lines()
            .nth(self.line_number - 1)
            .unwrap()
            .to_string()
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

pub struct Tokenizer<'a> {
    input: &'a str,
    current_index: usize,
    line_number: usize,
}

#[derive(Debug)]
pub enum TokenType {
    KeywordNamespace,
    KeywordExposing,
    KeywordImport,
    KeywordLet,
    KeywordIn,
    KeywordIf,
    KeywordElse,
    KeywordMatch,
    KeywordWhen,
    KeywordTrue,
    KeywordFalse,
    NamespaceId,
    FunctionId,
    TypeId,
    FieldId,
    VariableId,
    Integer,
    Float,
    Char,
    String,
    LeftSquareBracket,
    RightSquareBracket,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Underscore,
    Comma,
    Colon,
    Or,
    And,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Not,
    Equals,
    NotEquals,
    DoubleEquals,
    DoublePlus,
    Flow,
    Plus,
    Dash,
    ForwardSlash,
    Star,
    Caret,
    Period,
    Whitespace,
    Newline,
    LeftArrow,
}

pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

impl<'a> Tokenizer<'a> {
    pub fn build(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input,
            current_index: 0,
            line_number: 1,
        }
    }

    pub fn execute(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while self.input.len() > self.current_index {
            let char = self.current_char();
            if char.is_none() {
                break;
            }
            let unwrapped_char = char.unwrap();

            if unwrapped_char == '#' {
                tokens.push(self.create_type_id()?);
            } else if unwrapped_char == '@' {
                tokens.push(self.create_namespace_id()?);
            } else if unwrapped_char == '\'' {
                tokens.push(self.create_char()?);
            } else if unwrapped_char == '"' {
                tokens.push(self.create_string()?);
            } else if unwrapped_char == '[' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::LeftSquareBracket,
                });
            } else if unwrapped_char == ']' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::RightSquareBracket,
                });
            } else if unwrapped_char == '(' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::LeftBracket,
                });
            } else if unwrapped_char == ')' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::RightBracket,
                });
            } else if unwrapped_char == '{' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::LeftCurlyBracket,
                });
            } else if unwrapped_char == '}' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::RightCurlyBracket,
                });
            } else if unwrapped_char == '<' {
                tokens.push(self.create_less_than()?);
            } else if unwrapped_char == '>' {
                tokens.push(self.create_greater_than()?);
            } else if unwrapped_char == '_' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::Underscore,
                });
            } else if unwrapped_char == ',' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::Comma,
                });
            } else if unwrapped_char == ':' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::Colon,
                });
            } else if unwrapped_char == '|' {
                tokens.push(self.create_from_or()?);
            } else if unwrapped_char == '&' {
                tokens.push(self.create_from_and()?);
            } else if unwrapped_char == '=' {
                tokens.push(self.create_from_equals()?);
            } else if unwrapped_char == '!' {
                tokens.push(self.create_from_not()?);
            } else if unwrapped_char == '+' {
                tokens.push(self.create_from_plus()?);
            } else if unwrapped_char == '-' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::Dash,
                });
            } else if unwrapped_char == '/' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::ForwardSlash,
                });
            } else if unwrapped_char == '*' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::Star,
                });
            } else if unwrapped_char == '^' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::Caret,
                });
            } else if unwrapped_char == '.' {
                tokens.push(Token {
                    value: unwrapped_char.to_string(),
                    token_type: TokenType::Period,
                });
            } else if unwrapped_char == ' ' {
                tokens.push(self.create_whitespace()?);
            } else if matches!(unwrapped_char, '\n' | '\r') {
                tokens.push(self.create_newlines()?);
            } else if unwrapped_char.is_ascii_uppercase() {
                tokens.push(self.create_function_id()?);
            } else if unwrapped_char.is_ascii_lowercase() {
                tokens.push(self.create_word()?);
            } else if unwrapped_char.is_ascii_digit() {
                tokens.push(self.create_number()?);
            } else {
                return Err(Box::new(SpiralError {
                    error_text: "Unable to parse character",
                    help_text: "",
                    file: self.input.to_string(),
                    begin: self.current_index,
                    end: self.current_index,
                    line_number: self.line_number,
                }));
            }

            self.current_index += 1;
        }
        return Ok(tokens);
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_index)
    }

    fn create_type_id(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        let begin_index = self.current_index;
        self.current_index += 1;

        let mut char = self.current_char().ok_or(SpiralError {
            error_text: "'#' must be followed by a capital letter",
            help_text: "",
            file: self.input.to_string(),
            begin: begin_index,
            end: begin_index,
            line_number: self.line_number,
        })?;

        if !char.is_ascii_uppercase() {
            return Err(Box::new(SpiralError {
                error_text: "Type must begin with capital letter",
                help_text: "",
                file: self.input.to_string(),
                begin: begin_index,
                end: self.current_index,
                line_number: self.line_number,
            }));
        }

        while char.is_ascii_uppercase() || char.is_ascii_lowercase() {
            value += &char.to_string();
            self.current_index += 1;
            let char_result = self.current_char();
            if char_result.is_none() {
                break;
            }
            char = char_result.unwrap();
        }

        self.current_index -= 1;
        Ok(Token {
            value,
            token_type: TokenType::TypeId,
        })
    }

    fn create_namespace_id(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        let begin_index = self.current_index;
        self.current_index += 1;

        let mut char = self.current_char().ok_or(SpiralError {
            error_text: "'@' must be followed by a capital letter",
            help_text: "",
            file: self.input.to_string(),
            begin: begin_index,
            end: begin_index,
            line_number: self.line_number,
        })?;

        if !char.is_ascii_uppercase() {
            return Err(Box::new(SpiralError {
                error_text: "Namespace must begin with capital letter",
                help_text: "",
                file: self.input.to_string(),
                begin: begin_index,
                end: self.current_index,
                line_number: self.line_number,
            }));
        }

        while char.is_ascii_uppercase() || char.is_ascii_lowercase() {
            value += &char.to_string();
            self.current_index += 1;
            let char_result = self.current_char();
            if char_result.is_none() {
                break;
            }
            char = char_result.unwrap();
        }

        self.current_index -= 1;
        Ok(Token {
            value,
            token_type: TokenType::NamespaceId,
        })
    }

    fn create_char(&mut self) -> Result<Token> {
        let mut value = String::from("");
        let begin_index = self.current_index;
        self.current_index += 1;

        let mut char = self.current_char().ok_or(SpiralError {
            error_text: "Char must be closed",
            help_text: "",
            file: self.input.to_string(),
            begin: begin_index,
            end: begin_index,
            line_number: self.line_number,
        })?;

        while !(char == '\'' && self.input.chars().nth(self.current_index - 1).unwrap() != '\\') {
            value += &char.to_string();
            self.current_index += 1;
            let char_result = self.current_char();
            if char_result.is_none() {
                break;
            }
            char = char_result.unwrap();
        }

        if char != '\'' {
            Err(Box::new(SpiralError {
                error_text: "Char must be closed",
                help_text: "",
                file: self.input.to_string(),
                begin: begin_index,
                end: self.current_index,
                line_number: self.line_number,
            }))
        } else {
            Ok(Token {
                value,
                token_type: TokenType::Char,
            })
        }
    }

    fn create_string(&mut self) -> Result<Token> {
        let mut value = String::from("");
        let begin_index = self.current_index;
        self.current_index += 1;

        let mut char = self.current_char().ok_or(SpiralError {
            error_text: "String must be closed",
            help_text: "",
            file: self.input.to_string(),
            begin: begin_index,
            end: begin_index,
            line_number: self.line_number,
        })?;

        while !(char == '"' && self.input.chars().nth(self.current_index - 1).unwrap() != '\\') {
            value += &char.to_string();
            self.current_index += 1;
            let char_result = self.current_char();
            if char_result.is_none() {
                break;
            }
            char = char_result.unwrap();
        }

        if char != '"' {
            Err(Box::new(SpiralError {
                error_text: "String must be closed",
                help_text: "",
                file: self.input.to_string(),
                begin: begin_index,
                end: self.current_index,
                line_number: self.line_number,
            }))
        } else {
            Ok(Token {
                value,
                token_type: TokenType::String,
            })
        }
    }

    fn create_less_than(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;

        let char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::LessThan,
            });
        }
        let unwrapped_char = char.unwrap();

        if unwrapped_char == '=' {
            value += &unwrapped_char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::LessThanEquals,
            })
        } else if unwrapped_char == '-' {
            value += &unwrapped_char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::LeftArrow,
            })
        } else {
            self.current_index -= 1;
            Ok(Token {
                value,
                token_type: TokenType::LessThan,
            })
        }
    }

    fn create_greater_than(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;

        let char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::GreaterThan,
            });
        }
        let unwrapped_char = char.unwrap();

        if unwrapped_char == '=' {
            value += &unwrapped_char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::GreaterThanEquals,
            })
        } else {
            self.current_index -= 1;
            Ok(Token {
                value,
                token_type: TokenType::GreaterThan,
            })
        }
    }

    fn create_from_or(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        let begin_index = self.current_index;
        self.current_index += 1;

        let char = self.current_char().ok_or(SpiralError {
            error_text: "Unknown Character: '|'",
            help_text: "",
            file: self.input.to_string(),
            begin: begin_index,
            end: begin_index,
            line_number: self.line_number,
        })?;

        if char == '|' {
            value += &char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::Or,
            })
        } else if char == '>' {
            value += &char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::Flow,
            })
        } else {
            Err(Box::new(SpiralError {
                error_text: "Unknown Character: '|'",
                help_text: "",
                file: self.input.to_string(),
                begin: begin_index,
                end: begin_index,
                line_number: self.line_number,
            }))
        }
    }

    fn create_from_and(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        let begin_index = self.current_index;
        self.current_index += 1;

        let char = self.current_char().ok_or(SpiralError {
            error_text: "Unknown Character: '&'",
            help_text: "",
            file: self.input.to_string(),
            begin: begin_index,
            end: begin_index,
            line_number: self.line_number,
        })?;

        if char == '&' {
            value += &char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::And,
            })
        } else {
            Err(Box::new(SpiralError {
                error_text: "Unknown Character: '&'",
                help_text: "",
                file: self.input.to_string(),
                begin: begin_index,
                end: begin_index,
                line_number: self.line_number,
            }))
        }
    }

    fn create_from_equals(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;

        let char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::Equals,
            });
        }
        let unwrapped_char = char.unwrap();

        if unwrapped_char == '=' {
            value += &unwrapped_char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::DoubleEquals,
            })
        } else {
            self.current_index -= 1;
            Ok(Token {
                value,
                token_type: TokenType::Equals,
            })
        }
    }

    fn create_from_not(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;

        let char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::Not,
            });
        }
        let unwrapped_char = char.unwrap();

        if unwrapped_char == '=' {
            value += &unwrapped_char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::NotEquals,
            })
        } else {
            self.current_index -= 1;
            Ok(Token {
                value,
                token_type: TokenType::Not,
            })
        }
    }

    fn create_from_plus(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;

        let char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::Plus,
            });
        }
        let unwrapped_char = char.unwrap();

        if unwrapped_char == '+' {
            value += &unwrapped_char.to_string();
            Ok(Token {
                value,
                token_type: TokenType::DoublePlus,
            })
        } else {
            self.current_index -= 1;
            Ok(Token {
                value,
                token_type: TokenType::Plus,
            })
        }
    }

    fn create_whitespace(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;

        let mut char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::Whitespace,
            });
        }
        let mut unwrapped_char = char.unwrap();

        while unwrapped_char == ' ' {
            value += &unwrapped_char.to_string();
            self.current_index += 1;
            char = self.current_char();
            if char.is_none() {
                break;
            }
            unwrapped_char = char.unwrap();
        }

        self.current_index -= 1;
        Ok(Token {
            value,
            token_type: TokenType::Whitespace,
        })
    }

    fn create_newlines(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;
        self.line_number += 1;

        let mut char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::Newline,
            });
        }
        let mut unwrapped_char = char.unwrap();

        while matches!(unwrapped_char, '\n' | '\r') {
            value += &unwrapped_char.to_string();
            self.current_index += 1;
            char = self.current_char();
            if char.is_none() {
                break;
            }
            unwrapped_char = char.unwrap();
        }

        self.current_index -= 1;
        Ok(Token {
            value,
            token_type: TokenType::Newline,
        })
    }

    fn create_function_id(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;

        let mut char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::FunctionId,
            });
        }
        let mut unwrapped_char = char.unwrap();

        while unwrapped_char.is_ascii_uppercase() || unwrapped_char.is_ascii_lowercase() {
            value += &unwrapped_char.to_string();
            self.current_index += 1;
            char = self.current_char();
            if char.is_none() {
                break;
            }
            unwrapped_char = char.unwrap();
        }

        self.current_index -= 1;
        Ok(Token {
            value,
            token_type: TokenType::FunctionId,
        })
    }

    fn create_word(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        self.current_index += 1;

        let mut char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::VariableId,
            });
        }
        let mut unwrapped_char = char.unwrap();

        while unwrapped_char.is_ascii_uppercase()
            || unwrapped_char.is_ascii_lowercase()
            || unwrapped_char == ':'
        {
            value += &unwrapped_char.to_string();
            if unwrapped_char == ':' {
                return Ok(Token {
                    value,
                    token_type: TokenType::FieldId,
                });
            }

            self.current_index += 1;
            char = self.current_char();
            if char.is_none() {
                break;
            }
            unwrapped_char = char.unwrap();
        }

        self.current_index -= 1;
        if value == "namespace" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordNamespace,
            })
        } else if value == "exposing" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordExposing,
            })
        } else if value == "import" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordImport,
            })
        } else if value == "let" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordLet,
            })
        } else if value == "in" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordIn,
            })
        } else if value == "if" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordIf,
            })
        } else if value == "else" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordElse,
            })
        } else if value == "match" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordMatch,
            })
        } else if value == "when" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordWhen,
            })
        } else if value == "true" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordTrue,
            })
        } else if value == "false" {
            Ok(Token {
                value,
                token_type: TokenType::KeywordFalse,
            })
        } else {
            Ok(Token {
                value,
                token_type: TokenType::VariableId,
            })
        }
    }

    fn create_number(&mut self) -> Result<Token> {
        let mut value = self.current_char().unwrap().to_string();
        let begin_index = self.current_index;
        self.current_index += 1;
        let mut period_used = false;

        let mut char = self.current_char();
        if char.is_none() {
            return Ok(Token {
                value,
                token_type: TokenType::Integer,
            });
        }
        let mut unwrapped_char = char.unwrap();

        while unwrapped_char.is_ascii_digit() || unwrapped_char == '.' {
            if unwrapped_char == '.' {
                if !period_used {
                    period_used = true;
                } else {
                    return Err(Box::new(SpiralError {
                        error_text: "Number contains multiple periods",
                        help_text: "Ensure the number has a maximum of one period",
                        file: self.input.to_string(),
                        begin: begin_index,
                        end: begin_index,
                        line_number: self.line_number,
                    }));
                }
            }

            value += &unwrapped_char.to_string();
            self.current_index += 1;
            char = self.current_char();
            if char.is_none() {
                break;
            }
            unwrapped_char = char.unwrap();
        }

        self.current_index -= 1;
        if period_used {
            Ok(Token {
                value,
                token_type: TokenType::Float,
            })
        } else {
            Ok(Token {
                value,
                token_type: TokenType::Integer,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespaces_should_parse_with_specific_exposes() {
        let result = parse("namespace @Maths exposing [Function variable #Type]").unwrap();
        let expected = vec![
            "KeywordNamespace",
            "Whitespace",
            "NamespaceId",
            "Whitespace",
            "KeywordExposing",
            "Whitespace",
            "LeftSquareBracket",
            "FunctionId",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "TypeId",
            "RightSquareBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn namespaces_should_parse_with_wildcard_exposes() {
        let result = parse("namespace @Maths exposing _").unwrap();
        let expected = vec![
            "KeywordNamespace",
            "Whitespace",
            "NamespaceId",
            "Whitespace",
            "KeywordExposing",
            "Whitespace",
            "Underscore",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn imports_should_parse_with_no_specific_imports() {
        let result = parse("import @Maths").unwrap();
        let expected = vec!["KeywordImport", "Whitespace", "NamespaceId"];
        assert_eq!(result, expected);
    }

    #[test]
    fn imports_should_parse_with_specific_imports() {
        let result = parse("import @Maths exposing [Function variable #Type]").unwrap();
        let expected = vec![
            "KeywordImport",
            "Whitespace",
            "NamespaceId",
            "Whitespace",
            "KeywordExposing",
            "Whitespace",
            "LeftSquareBracket",
            "FunctionId",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "TypeId",
            "RightSquareBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn function_definitions_should_parse_function_type_definition() {
        let result = parse("AddTwo : #Fn<#Int,#Int>").unwrap();
        let expected = vec![
            "FunctionId",
            "Whitespace",
            "Colon",
            "Whitespace",
            "TypeId",
            "LessThan",
            "TypeId",
            "Comma",
            "TypeId",
            "GreaterThan",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn function_definitions_should_parse_function_definition() {
        let result = parse("Main = @IO.Print(AddTwo 3)").unwrap();
        let expected = vec![
            "FunctionId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "NamespaceId",
            "Period",
            "FunctionId",
            "LeftBracket",
            "FunctionId",
            "Whitespace",
            "Integer",
            "RightBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn type_definitions_should_parse_type_definitions() {
        let result = parse("#Number = #Int || #Float").unwrap();
        let expected = vec![
            "TypeId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "TypeId",
            "Whitespace",
            "Or",
            "Whitespace",
            "TypeId",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_booleans() {
        let result = parse("true || false").unwrap();
        let expected = vec![
            "KeywordTrue",
            "Whitespace",
            "Or",
            "Whitespace",
            "KeywordFalse",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_integers() {
        let result = parse("2 + 7").unwrap();
        let expected = vec!["Integer", "Whitespace", "Plus", "Whitespace", "Integer"];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_floats() {
        let result = parse("3.14 + 4.9").unwrap();
        let expected = vec!["Float", "Whitespace", "Plus", "Whitespace", "Float"];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_characters_and_strings() {
        let result = parse("'a' ++ \"hello\"").unwrap();
        let expected = vec!["Char", "Whitespace", "DoublePlus", "Whitespace", "String"];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_lists() {
        let result = parse("[1 2 3]").unwrap();
        let expected = vec![
            "LeftSquareBracket",
            "Integer",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Integer",
            "RightSquareBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_a_let_statement() {
        let result =
            parse("let\n  twentyFour = 3 * 8\n  sixteen = 4 ^ 2\nin\n  twentyFour + sixteen")
                .unwrap();
        let expected = vec![
            "KeywordLet",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Star",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Caret",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordIn",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "VariableId",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_an_if_statement_without_else_if_clause() {
        let result = parse("if key == 40\n  n + 1\nelse\n  n").unwrap();
        let expected = vec![
            "KeywordIf",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "DoubleEquals",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordElse",
            "Newline",
            "Whitespace",
            "VariableId",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_an_if_statement_with_an_else_if_clause() {
        let result = parse("if key == 40\n  n + 1\nelse if key == 38\n  n - 1\nelse\n  n").unwrap();
        let expected = vec![
            "KeywordIf",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "DoubleEquals",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordElse",
            "Whitespace",
            "KeywordIf",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "DoubleEquals",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Dash",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordElse",
            "Newline",
            "Whitespace",
            "VariableId",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_a_case_statement_without_a_when_clause() {
        let result = parse("match n\nelse\n  1").unwrap();
        let expected = vec![
            "KeywordMatch",
            "Whitespace",
            "VariableId",
            "Newline",
            "KeywordElse",
            "Newline",
            "Whitespace",
            "Integer",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_a_case_statement_with_a_when_clause() {
        let result =
            parse("match n\nwhen 0\n  1\nwhen 1\n  1\nelse\n  Fib(n-1) + Fib(n-2)").unwrap();
        let expected = vec![
            "KeywordMatch",
            "Whitespace",
            "VariableId",
            "Newline",
            "KeywordWhen",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordWhen",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordElse",
            "Newline",
            "Whitespace",
            "FunctionId",
            "LeftBracket",
            "VariableId",
            "Dash",
            "Integer",
            "RightBracket",
            "Whitespace",
            "Plus",
            "Whitespace",
            "FunctionId",
            "LeftBracket",
            "VariableId",
            "Dash",
            "Integer",
            "RightBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn bin_ops_should_parse_math_operators() {
        let result = parse("1 + 2 - 3 * 4 / 5 ^ 6").unwrap();
        let expected = vec![
            "Integer",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Dash",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Star",
            "Whitespace",
            "Integer",
            "Whitespace",
            "ForwardSlash",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Caret",
            "Whitespace",
            "Integer",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn bin_ops_should_parse_boolean_operators() {
        let result = parse("!true || false && true").unwrap();
        let expected = vec![
            "Not",
            "KeywordTrue",
            "Whitespace",
            "Or",
            "Whitespace",
            "KeywordFalse",
            "Whitespace",
            "And",
            "Whitespace",
            "KeywordTrue",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn bin_ops_should_parse_flow_operator() {
        let result = parse("\"Hello\" |> Output").unwrap();
        let expected = vec!["String", "Whitespace", "Flow", "Whitespace", "FunctionId"];
        assert_eq!(result, expected);
    }

    #[test]
    fn bin_ops_should_parse_concatinate_operator() {
        let result = parse("' ' ++ \"Hello\"").unwrap();
        let expected = vec!["Char", "Whitespace", "DoublePlus", "Whitespace", "String"];
        assert_eq!(result, expected);
    }

    #[test]
    fn structs_should_parse_struct_type_definition() {
        let result = parse("#MyStruct = #Struct<x: #Int, y: #Int>").unwrap();
        let expected = vec![
            "TypeId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "TypeId",
            "LessThan",
            "FieldId",
            "Whitespace",
            "TypeId",
            "Comma",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "TypeId",
            "GreaterThan",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn structs_should_parse_struct_creation() {
        let result = parse("point = { x: 3, y: 4 }").unwrap();
        let expected = vec![
            "VariableId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "LeftCurlyBracket",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "Integer",
            "Comma",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "Integer",
            "Whitespace",
            "RightCurlyBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn structs_should_parse_field_access() {
        let result = parse("point.x == 3").unwrap();
        let expected = vec![
            "VariableId",
            "Period",
            "VariableId",
            "Whitespace",
            "DoubleEquals",
            "Whitespace",
            "Integer",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn structs_should_parse_struct_editing() {
        let result = parse("{ point <- x: point.x + 1, y: point.y + 1 }").unwrap();
        let expected = vec![
            "LeftCurlyBracket",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "LeftArrow",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "VariableId",
            "Period",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Comma",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "VariableId",
            "Period",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Whitespace",
            "RightCurlyBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_new_lines() {
        let result = parse("Main = @IO.Print\n  \"Hello World!\"").unwrap();
        let expected = vec![
            "FunctionId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "NamespaceId",
            "Period",
            "FunctionId",
            "Newline",
            "Whitespace",
            "String",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_raise_error_if_namespace_does_not_begin_with_a_capital_letter() {
        let result = parse("@io.Print");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_if_type_does_not_begin_with_a_capital_letter() {
        let result = parse("#type");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unknown_or_token() {
        let result = parse("true | false");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unknown_and_token() {
        let result = parse("true & false");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unknown_other_token() {
        let result = parse("true $ false");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unclosed_string() {
        let result = parse("\"unclosed string");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unclosed_char() {
        let result = parse("'c");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_invalid_floats() {
        let result = parse("3.14.6");
        assert_eq!(result.is_err(), true);
    }

    fn parse(input: &str) -> super::Result<Vec<String>> {
        let mut t = super::Tokenizer::build(input);
        let tokens = t.execute()?;
        let mut output = Vec::new();
        for token in tokens {
            output.push(format!("{:?}", token.token_type));
        }
        Ok(output)
    }
}
