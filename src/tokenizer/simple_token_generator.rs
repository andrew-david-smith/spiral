pub struct SimpleTokenGenerator {
    pub char_to_match: char,
    pub token_type: super::TokenType,
}

impl super::token_generator::TokenGenerator for SimpleTokenGenerator {
    fn matches(&self, input: char) -> bool {
        self.char_to_match == input
    }

    fn generate(
        &self,
        input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token> {
        Ok(super::Token {
            value: input.to_string(),
            token_type: self.token_type.clone(),
            begin: tokenizer.current_index,
            end: tokenizer.current_index,
            line_number: tokenizer.line_number,
            line: tokenizer.current_line(),
        })
    }
}
