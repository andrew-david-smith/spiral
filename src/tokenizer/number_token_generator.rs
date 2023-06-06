pub struct NumberTokenGenerator {}

impl super::token_generator::TokenGenerator for NumberTokenGenerator {
    fn matches(&self, input: char) -> bool {
        input.is_ascii_digit()
    }

    fn generate(
        &self,
        _input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token> {
        let mut value = tokenizer.current_char().unwrap().to_string();
        let begin_index = tokenizer.current_index;
        tokenizer.current_index += 1;
        let mut period_used = false;

        let mut char = tokenizer.current_char();
        if char.is_none() {
            return Ok(super::Token {
                value,
                token_type: super::TokenType::Integer,
            });
        }
        let mut unwrapped_char = char.unwrap();

        while unwrapped_char.is_ascii_digit() || unwrapped_char == '.' {
            if unwrapped_char == '.' {
                if !period_used {
                    period_used = true;
                } else {
                    return Err(Box::new(super::SpiralError {
                        error_text: "Number contains multiple periods",
                        help_text: "Ensure the number has a maximum of one period",
                        file: tokenizer.input.to_string(),
                        begin: begin_index,
                        end: begin_index,
                        line_number: tokenizer.line_number,
                    }));
                }
            }

            value += &unwrapped_char.to_string();
            tokenizer.current_index += 1;
            char = tokenizer.current_char();
            if char.is_none() {
                break;
            }
            unwrapped_char = char.unwrap();
        }

        tokenizer.current_index -= 1;
        if period_used {
            Ok(super::Token {
                value,
                token_type: super::TokenType::Float,
            })
        } else {
            Ok(super::Token {
                value,
                token_type: super::TokenType::Integer,
            })
        }
    }
}
