pub trait TokenGenerator {
    fn matches(&self, input: char) -> bool;
    fn generate(
        &self,
        input: char,
        tokenizer: &mut super::Tokenizer,
    ) -> super::Result<super::Token>;
}
