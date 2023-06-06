pub struct AndTokenGenerator {}

impl super::token_generator::TokenGenerator for AndTokenGenerator {
    fn matches(&self, input: char) -> bool {
        '&' == input
    }

    fn generate(
        &self,
        _input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token> {
        let mut value = tokenizer.current_char().unwrap().to_string();
        let begin_index = tokenizer.current_index;
        tokenizer.current_index += 1;

        let char = tokenizer.current_char().ok_or(super::SpiralError {
            error_text: "Unknown Character: '&'",
            help_text: "",
            file: tokenizer.input.to_string(),
            begin: begin_index,
            end: begin_index,
            line_number: tokenizer.line_number,
        })?;

        if char == '&' {
            value += &char.to_string();
            Ok(super::Token {
                value,
                token_type: super::TokenType::And,
            })
        } else {
            Err(Box::new(super::SpiralError {
                error_text: "Unknown Character: '&'",
                help_text: "",
                file: tokenizer.input.to_string(),
                begin: begin_index,
                end: begin_index,
                line_number: tokenizer.line_number,
            }))
        }
    }
}
