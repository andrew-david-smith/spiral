#[derive(Debug)]
pub struct BinOpNode {
    pub left_node: Box<dyn super::node::Node>,
    pub right_node: Box<dyn super::node::Node>,
    pub token: super::super::tokenizer::Token,
}

impl super::node::Node for BinOpNode {
    fn represent(&self) -> String {
        format!(
            "BinOpNode<{},{},{}>",
            self.left_node.represent(),
            self.token.value,
            self.right_node.represent()
        )
    }
}
