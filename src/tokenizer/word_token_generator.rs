pub struct WordTokenGenerator {}

impl super::token_generator::TokenGenerator for WordTokenGenerator {
    fn matches(&self, input: char) -> bool {
        input.is_ascii_lowercase()
    }

    fn generate(
        &self,
        _input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token> {
        let mut value = tokenizer.current_char().unwrap().to_string();
        let begin_index = tokenizer.current_index;
        tokenizer.current_index += 1;

        let mut char = tokenizer.current_char();
        if char.is_none() {
            return Ok(super::Token {
                value,
                token_type: super::TokenType::VariableId,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            });
        }
        let mut unwrapped_char = char.unwrap();

        while unwrapped_char.is_ascii_uppercase()
            || unwrapped_char.is_ascii_lowercase()
            || unwrapped_char == ':'
        {
            value += &unwrapped_char.to_string();
            if unwrapped_char == ':' {
                return Ok(super::Token {
                    value,
                    token_type: super::TokenType::FieldId,
                    begin: begin_index,
                    end: tokenizer.current_index,
                    line_number: tokenizer.line_number,
                    line: tokenizer.current_line(),
                });
            }

            tokenizer.current_index += 1;
            char = tokenizer.current_char();
            if char.is_none() {
                break;
            }
            unwrapped_char = char.unwrap();
        }

        tokenizer.current_index -= 1;
        if value == "namespace" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordNamespace,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "exposing" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordExposing,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "import" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordImport,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "let" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordLet,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "in" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordIn,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "if" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordIf,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "else" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordElse,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "match" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordMatch,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "when" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordWhen,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "true" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordTrue,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else if value == "false" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordFalse,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        } else {
            Ok(super::Token {
                value,
                token_type: super::TokenType::VariableId,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        }
    }
}
