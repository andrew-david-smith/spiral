pub struct CharTokenGenerator {}

impl super::token_generator::TokenGenerator for CharTokenGenerator {
    fn matches(&self, input: char) -> bool {
        '\'' == input
    }

    fn generate(
        &self,
        _input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token> {
        let mut value = String::from("");
        let begin_index = tokenizer.current_index;
        tokenizer.current_index += 1;

        let mut char = tokenizer.current_char().ok_or(super::SpiralError {
            error_text: "Char must be closed",
            help_text: "",
            line_text: tokenizer.current_line(),
            begin: begin_index,
            end: begin_index,
            line_number: tokenizer.line_number,
        })?;

        while !(char == '\''
            && tokenizer
                .input
                .chars()
                .nth(tokenizer.current_index - 1)
                .unwrap()
                != '\\')
        {
            value += &char.to_string();
            tokenizer.current_index += 1;
            let char_result = tokenizer.current_char();
            if char_result.is_none() {
                break;
            }
            char = char_result.unwrap();
        }

        if char != '\'' {
            Err(Box::new(super::SpiralError {
                error_text: "Char must be closed",
                help_text: "",
                line_text: tokenizer.current_line(),
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
            }))
        } else {
            Ok(super::Token {
                value,
                token_type: super::TokenType::Char,
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
                line: tokenizer.current_line(),
            })
        }
    }
}
