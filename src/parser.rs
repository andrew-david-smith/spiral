use std::error;
mod bin_op_node;
mod int_node;
mod node;
mod unary_op_node;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
type SpiralError<'a> = super::spiral_error::SpiralError<'a>;
type TokenType = super::tokenizer::TokenType;

pub struct Parser {
    pub current_index: usize,
    pub tokens: Vec<super::tokenizer::Token>,
}

impl Parser {
    pub fn build(tokens: Vec<super::tokenizer::Token>) -> Parser {
        Parser {
            current_index: 0,
            tokens,
        }
    }

    pub fn execute(&mut self) -> Result<Box<dyn node::Node>> {
        self.expression()
    }

    fn current_token(&self) -> Option<&super::tokenizer::Token> {
        self.tokens.get(self.current_index)
    }

    fn previous_token(&self) -> Option<&super::tokenizer::Token> {
        if self.current_index > 1 {
            self.tokens.get(self.current_index - 1)
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.current_index += 1;
    }

    fn advance_through_whitespace(&mut self) {
        self.advance();
        let mut token = self.current_token();
        while token.is_some()
            && matches!(
                token.unwrap().token_type,
                TokenType::Whitespace | TokenType::Newline
            )
        {
            self.advance();
            token = self.current_token();
            if token.is_none() {
                break;
            }
        }
    }

    fn factor(&mut self) -> Result<Box<dyn node::Node>> {
        let token_result = self.current_token();
        if !token_result.is_some() {
            return Err(Box::new(SpiralError {
                error_text: "Expected a factor. Valid Factors are: Int",
                help_text: "",
                line_text: self.previous_token().unwrap().line.clone(),
                begin: self.previous_token().unwrap().begin,
                end: self.previous_token().unwrap().end,
                line_number: self.previous_token().unwrap().line_number,
            }));
        }
        let token = token_result.unwrap();

        if matches!(token.token_type, TokenType::Plus | TokenType::Dash) {
            let cloned_token = token.clone();
            self.advance_through_whitespace();
            let node = unary_op_node::UnaryOpNode {
                token: cloned_token,
                node: self.factor()?,
            };
            return Ok(Box::new(node));
        }

        if matches!(token.token_type, TokenType::Integer) {
            let node = int_node::IntNode {
                token: token.clone(),
            };
            self.advance_through_whitespace();
            return Ok(Box::new(node));
        }

        if matches!(token.token_type, TokenType::LeftBracket) {
            let cloned_token = token.clone();
            self.advance_through_whitespace();
            let expression = self.expression()?;

            let t_result = self.current_token();
            if !t_result.is_some() {
                return Err(Box::new(SpiralError {
                    error_text: "Unclosed bracket",
                    help_text: "Please close the bracket",
                    line_text: cloned_token.line.to_string(),
                    begin: cloned_token.begin,
                    end: self.previous_token().unwrap().end,
                    line_number: cloned_token.line_number,
                }));
            }

            let t = t_result.unwrap();

            if matches!(t.token_type, TokenType::RightBracket) {
                self.advance_through_whitespace();
                return Ok(expression);
            } else {
                return Err(Box::new(SpiralError {
                    error_text: "Unclosed bracket",
                    help_text: "Please close the bracket",
                    line_text: cloned_token.line.to_string(),
                    begin: cloned_token.begin,
                    end: t.end,
                    line_number: cloned_token.line_number,
                }));
            }
        }

        return Err(Box::new(SpiralError {
            error_text: "Unknown Factor",
            help_text: "",
            line_text: self.previous_token().unwrap().line.clone(),
            begin: self.previous_token().unwrap().begin,
            end: self.previous_token().unwrap().end,
            line_number: self.previous_token().unwrap().line_number,
        }));
    }

    fn term(&mut self) -> Result<Box<dyn node::Node>> {
        self.bin_op_node(vec![TokenType::Star, TokenType::ForwardSlash], "factor")
    }

    fn bin_op_node(&mut self, types: Vec<TokenType>, func: &str) -> Result<Box<dyn node::Node>> {
        let mut left = self.bin_op_function(func)?;

        while self.current_token().is_some()
            && types
                .iter()
                .any(|t| t == &self.current_token().unwrap().token_type)
        {
            let operation_token = self.current_token().unwrap().clone();
            self.advance_through_whitespace();
            let right = self.bin_op_function(func)?;
            left = Box::new(bin_op_node::BinOpNode {
                left_node: left,
                right_node: right,
                token: operation_token,
            });
        }

        return Ok(left);
    }

    // fn unary_op_node(&mut self, types: Vec<TokenType>, func : &str) -> Result<Box<dyn node::Node>> {
    // }

    fn bin_op_function(&mut self, input: &str) -> Result<Box<dyn node::Node>> {
        if input == "factor" {
            self.factor()
        } else {
            self.term()
        }
    }

    fn expression(&mut self) -> Result<Box<dyn node::Node>> {
        self.bin_op_node(vec![TokenType::Plus, TokenType::Dash], "term")
    }
}
