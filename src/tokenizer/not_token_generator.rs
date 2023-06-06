pub struct NotTokenGenerator {}

impl super::token_generator::TokenGenerator for NotTokenGenerator {
    fn matches(&self, input: char) -> bool {
        '!' == input
    }

    fn generate(
        &self,
        _input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token> {
        let mut value = tokenizer.current_char().unwrap().to_string();
        tokenizer.current_index += 1;

        let char = tokenizer.current_char();
        if char.is_none() {
            return Ok(super::Token {
                value,
                token_type: super::TokenType::Not,
            });
        }
        let unwrapped_char = char.unwrap();

        if unwrapped_char == '=' {
            value += &unwrapped_char.to_string();
            Ok(super::Token {
                value,
                token_type: super::TokenType::NotEquals,
            })
        } else {
            tokenizer.current_index -= 1;
            Ok(super::Token {
                value,
                token_type: super::TokenType::Not,
            })
        }
    }
}