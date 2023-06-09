#[derive(Debug)]
pub struct UnaryOpNode {
    pub node: Box<dyn super::node::Node>,
    pub token: super::super::tokenizer::Token,
}

impl super::node::Node for UnaryOpNode {
    fn represent(&self) -> String {
        format!(
            "UnaryOpNode<{},{}>",
            self.token.value,
            self.node.represent()
        )
    }
}
