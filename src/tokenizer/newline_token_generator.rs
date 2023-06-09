pub struct NewlineTokenGenerator {}

impl super::token_generator::TokenGenerator for NewlineTokenGenerator {
    fn matches(&self, input: char) -> bool {
        matches!(input, '\n' | '\r')
    }

    fn generate(
        &self,
        _input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token> {
        let mut value = tokenizer.current_char().unwrap().to_string();
        let begin_index = tokenizer.current_index;
        tokenizer.current_index += 1;
        tokenizer.line_number += 1;

        let mut char = tokenizer.current_char();
        if char.is_none() {
            return Ok(super::Token {
                value,
                token_type: super::TokenType::Newline,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            });
        }
        let mut unwrapped_char = char.unwrap();

        while matches!(unwrapped_char, '\n' | '\r') {
            value += &unwrapped_char.to_string();
            tokenizer.current_index += 1;
            char = tokenizer.current_char();
            if char.is_none() {
                break;
            }
            unwrapped_char = char.unwrap();
        }

        tokenizer.current_index -= 1;
        Ok(super::Token {
            value,
            token_type: super::TokenType::Newline,
            begin: begin_index,
            end: tokenizer.current_index,
            line_number: tokenizer.line_number,
            line: tokenizer.current_line(),
        })
    }
}
