#[derive(Debug)]
pub struct IntNode {
    pub token: super::super::tokenizer::Token,
}

impl super::node::Node for IntNode {
    fn represent(&self) -> String {
        format!("IntNode<{}>", self.token.value)
    }
}
