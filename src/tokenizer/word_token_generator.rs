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
        tokenizer.current_index += 1;

        let mut char = tokenizer.current_char();
        if char.is_none() {
            return Ok(super::Token {
                value,
                token_type: super::TokenType::VariableId,
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
            })
        } else if value == "exposing" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordExposing,
            })
        } else if value == "import" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordImport,
            })
        } else if value == "let" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordLet,
            })
        } else if value == "in" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordIn,
            })
        } else if value == "if" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordIf,
            })
        } else if value == "else" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordElse,
            })
        } else if value == "match" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordMatch,
            })
        } else if value == "when" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordWhen,
            })
        } else if value == "true" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordTrue,
            })
        } else if value == "false" {
            Ok(super::Token {
                value,
                token_type: super::TokenType::KeywordFalse,
            })
        } else {
            Ok(super::Token {
                value,
                token_type: super::TokenType::VariableId,
            })
        }
    }
}
