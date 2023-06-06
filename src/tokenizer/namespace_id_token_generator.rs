pub struct NamespaceIdTokenGenerator {}

impl super::token_generator::TokenGenerator for NamespaceIdTokenGenerator {
    fn matches(&self, input: char) -> bool {
        '@' == input
    }

    fn generate(
        &self,
        _input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token> {
        let mut value = tokenizer.current_char().unwrap().to_string();
        let begin_index = tokenizer.current_index;
        tokenizer.current_index += 1;

        let mut char = tokenizer.current_char().ok_or(super::SpiralError {
            error_text: "'@' must be followed by a capital letter",
            help_text: "",
            file: tokenizer.input.to_string(),
            begin: begin_index,
            end: begin_index,
            line_number: tokenizer.line_number,
        })?;

        if !char.is_ascii_uppercase() {
            return Err(Box::new(super::SpiralError {
                error_text: "Namespace must begin with capital letter",
                help_text: "",
                file: tokenizer.input.to_string(),
                begin: begin_index,
                end: tokenizer.current_index,
                line_number: tokenizer.line_number,
            }));
        }

        while char.is_ascii_uppercase() || char.is_ascii_lowercase() {
            value += &char.to_string();
            tokenizer.current_index += 1;
            let char_result = tokenizer.current_char();
            if char_result.is_none() {
                break;
            }
            char = char_result.unwrap();
        }

        tokenizer.current_index -= 1;
        Ok(super::Token {
            value,
            token_type: super::TokenType::NamespaceId,
        })
    }
}
