use std::error;
mod and_token_generator;
mod char_token_generator;
mod equals_token_generator;
mod function_id_token_generator;
mod greater_than_token_generator;
mod less_than_token_generator;
mod namespace_id_token_generator;
mod newline_token_generator;
mod not_token_generator;
mod number_token_generator;
mod or_token_generator;
mod plus_token_generator;
mod simple_token_generator;
mod string_token_generator;
mod token_generator;
mod type_id_token_generator;
mod whitespace_token_generator;
mod word_token_generator;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
type SpiralError<'a> = super::spiral_error::SpiralError<'a>;

pub struct Tokenizer<'a> {
    pub input: &'a str,
    pub current_index: usize,
    pub line_number: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    KeywordNamespace,
    KeywordExposing,
    KeywordImport,
    KeywordLet,
    KeywordIn,
    KeywordIf,
    KeywordElse,
    KeywordMatch,
    KeywordWhen,
    KeywordTrue,
    KeywordFalse,
    NamespaceId,
    FunctionId,
    TypeId,
    FieldId,
    VariableId,
    Integer,
    Float,
    Char,
    String,
    LeftSquareBracket,
    RightSquareBracket,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Underscore,
    Comma,
    Colon,
    Or,
    And,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Not,
    Equals,
    NotEquals,
    DoubleEquals,
    DoublePlus,
    Flow,
    Plus,
    Dash,
    ForwardSlash,
    Star,
    Caret,
    Period,
    Whitespace,
    Newline,
    LeftArrow,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub begin: usize,
    pub end: usize,
    pub line_number: usize,
    pub line: String,
}

impl<'a> Tokenizer<'a> {
    pub fn build(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input,
            current_index: 0,
            line_number: 1,
        }
    }

    pub fn execute(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while self.input.len() > self.current_index {
            let char = self.current_char();
            if char.is_none() {
                break;
            }
            let unwrapped_char = char.unwrap();
            let mut matched = false;

            for generator in self.token_generators().iter_mut() {
                if generator.matches(unwrapped_char) {
                    tokens.push(generator.generate(unwrapped_char, self)?);
                    matched = true;
                    break;
                }
            }

            if !matched {
                return Err(Box::new(super::spiral_error::SpiralError {
                    error_text: "Unable to parse character",
                    help_text: "",
                    line_text: self.current_line(),
                    begin: self.current_index,
                    end: self.current_index,
                    line_number: self.line_number,
                }));
            }
            self.current_index += 1;
        }

        return Ok(tokens);
    }

    fn token_generators(&self) -> Vec<Box<dyn token_generator::TokenGenerator>> {
        vec![
            Box::new(type_id_token_generator::TypeIdTokenGenerator {}),
            Box::new(namespace_id_token_generator::NamespaceIdTokenGenerator {}),
            Box::new(char_token_generator::CharTokenGenerator {}),
            Box::new(string_token_generator::StringTokenGenerator {}),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '[',
                token_type: TokenType::LeftSquareBracket,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: ']',
                token_type: TokenType::RightSquareBracket,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '(',
                token_type: TokenType::LeftBracket,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: ')',
                token_type: TokenType::RightBracket,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '{',
                token_type: TokenType::LeftCurlyBracket,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '}',
                token_type: TokenType::RightCurlyBracket,
            }),
            Box::new(less_than_token_generator::LessThanTokenGenerator {}),
            Box::new(greater_than_token_generator::GreaterThanTokenGenerator {}),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '_',
                token_type: TokenType::Underscore,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: ',',
                token_type: TokenType::Comma,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: ':',
                token_type: TokenType::Colon,
            }),
            Box::new(or_token_generator::OrTokenGenerator {}),
            Box::new(and_token_generator::AndTokenGenerator {}),
            Box::new(equals_token_generator::EqualsTokenGenerator {}),
            Box::new(not_token_generator::NotTokenGenerator {}),
            Box::new(plus_token_generator::PlusTokenGenerator {}),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '-',
                token_type: TokenType::Dash,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '/',
                token_type: TokenType::ForwardSlash,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '*',
                token_type: TokenType::Star,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '^',
                token_type: TokenType::Caret,
            }),
            Box::new(simple_token_generator::SimpleTokenGenerator {
                char_to_match: '.',
                token_type: TokenType::Period,
            }),
            Box::new(whitespace_token_generator::WhitespaceTokenGenerator {}),
            Box::new(newline_token_generator::NewlineTokenGenerator {}),
            Box::new(function_id_token_generator::FunctionIdTokenGenerator {}),
            Box::new(word_token_generator::WordTokenGenerator {}),
            Box::new(number_token_generator::NumberTokenGenerator {}),
        ]
    }

    pub fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_index)
    }

    pub fn current_line(&self) -> String {
        self.input
            .lines()
            .nth(self.line_number - 1)
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespaces_should_parse_with_specific_exposes() {
        let result = parse("namespace @Maths exposing [Function variable #Type]").unwrap();
        let expected = vec![
            "KeywordNamespace",
            "Whitespace",
            "NamespaceId",
            "Whitespace",
            "KeywordExposing",
            "Whitespace",
            "LeftSquareBracket",
            "FunctionId",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "TypeId",
            "RightSquareBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn namespaces_should_parse_with_wildcard_exposes() {
        let result = parse("namespace @Maths exposing _").unwrap();
        let expected = vec![
            "KeywordNamespace",
            "Whitespace",
            "NamespaceId",
            "Whitespace",
            "KeywordExposing",
            "Whitespace",
            "Underscore",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn imports_should_parse_with_no_specific_imports() {
        let result = parse("import @Maths").unwrap();
        let expected = vec!["KeywordImport", "Whitespace", "NamespaceId"];
        assert_eq!(result, expected);
    }

    #[test]
    fn imports_should_parse_with_specific_imports() {
        let result = parse("import @Maths exposing [Function variable #Type]").unwrap();
        let expected = vec![
            "KeywordImport",
            "Whitespace",
            "NamespaceId",
            "Whitespace",
            "KeywordExposing",
            "Whitespace",
            "LeftSquareBracket",
            "FunctionId",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "TypeId",
            "RightSquareBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn function_definitions_should_parse_function_type_definition() {
        let result = parse("AddTwo : #Fn<#Int,#Int>").unwrap();
        let expected = vec![
            "FunctionId",
            "Whitespace",
            "Colon",
            "Whitespace",
            "TypeId",
            "LessThan",
            "TypeId",
            "Comma",
            "TypeId",
            "GreaterThan",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn function_definitions_should_parse_function_definition() {
        let result = parse("Main = @IO.Print(AddTwo 3)").unwrap();
        let expected = vec![
            "FunctionId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "NamespaceId",
            "Period",
            "FunctionId",
            "LeftBracket",
            "FunctionId",
            "Whitespace",
            "Integer",
            "RightBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn type_definitions_should_parse_type_definitions() {
        let result = parse("#Number = #Int || #Float").unwrap();
        let expected = vec![
            "TypeId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "TypeId",
            "Whitespace",
            "Or",
            "Whitespace",
            "TypeId",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_booleans() {
        let result = parse("true || false").unwrap();
        let expected = vec![
            "KeywordTrue",
            "Whitespace",
            "Or",
            "Whitespace",
            "KeywordFalse",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_integers() {
        let result = parse("2 + 7").unwrap();
        let expected = vec!["Integer", "Whitespace", "Plus", "Whitespace", "Integer"];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_floats() {
        let result = parse("3.14 + 4.9").unwrap();
        let expected = vec!["Float", "Whitespace", "Plus", "Whitespace", "Float"];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_characters_and_strings() {
        let result = parse("'a' ++ \"hello\"").unwrap();
        let expected = vec!["Char", "Whitespace", "DoublePlus", "Whitespace", "String"];
        assert_eq!(result, expected);
    }

    #[test]
    fn literals_should_parse_lists() {
        let result = parse("[1 2 3]").unwrap();
        let expected = vec![
            "LeftSquareBracket",
            "Integer",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Integer",
            "RightSquareBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_a_let_statement() {
        let result =
            parse("let\n  twentyFour = 3 * 8\n  sixteen = 4 ^ 2\nin\n  twentyFour + sixteen")
                .unwrap();
        let expected = vec![
            "KeywordLet",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Star",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Caret",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordIn",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "VariableId",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_an_if_statement_without_else_if_clause() {
        let result = parse("if key == 40\n  n + 1\nelse\n  n").unwrap();
        let expected = vec![
            "KeywordIf",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "DoubleEquals",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordElse",
            "Newline",
            "Whitespace",
            "VariableId",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_an_if_statement_with_an_else_if_clause() {
        let result = parse("if key == 40\n  n + 1\nelse if key == 38\n  n - 1\nelse\n  n").unwrap();
        let expected = vec![
            "KeywordIf",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "DoubleEquals",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordElse",
            "Whitespace",
            "KeywordIf",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "DoubleEquals",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "Dash",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordElse",
            "Newline",
            "Whitespace",
            "VariableId",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_a_case_statement_without_a_when_clause() {
        let result = parse("match n\nelse\n  1").unwrap();
        let expected = vec![
            "KeywordMatch",
            "Whitespace",
            "VariableId",
            "Newline",
            "KeywordElse",
            "Newline",
            "Whitespace",
            "Integer",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_a_case_statement_with_a_when_clause() {
        let result =
            parse("match n\nwhen 0\n  1\nwhen 1\n  1\nelse\n  Fib(n-1) + Fib(n-2)").unwrap();
        let expected = vec![
            "KeywordMatch",
            "Whitespace",
            "VariableId",
            "Newline",
            "KeywordWhen",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordWhen",
            "Whitespace",
            "Integer",
            "Newline",
            "Whitespace",
            "Integer",
            "Newline",
            "KeywordElse",
            "Newline",
            "Whitespace",
            "FunctionId",
            "LeftBracket",
            "VariableId",
            "Dash",
            "Integer",
            "RightBracket",
            "Whitespace",
            "Plus",
            "Whitespace",
            "FunctionId",
            "LeftBracket",
            "VariableId",
            "Dash",
            "Integer",
            "RightBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn bin_ops_should_parse_math_operators() {
        let result = parse("1 + 2 - 3 * 4 / 5 ^ 6").unwrap();
        let expected = vec![
            "Integer",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Dash",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Star",
            "Whitespace",
            "Integer",
            "Whitespace",
            "ForwardSlash",
            "Whitespace",
            "Integer",
            "Whitespace",
            "Caret",
            "Whitespace",
            "Integer",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn bin_ops_should_parse_boolean_operators() {
        let result = parse("!true || false && true").unwrap();
        let expected = vec![
            "Not",
            "KeywordTrue",
            "Whitespace",
            "Or",
            "Whitespace",
            "KeywordFalse",
            "Whitespace",
            "And",
            "Whitespace",
            "KeywordTrue",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn bin_ops_should_parse_flow_operator() {
        let result = parse("\"Hello\" |> Output").unwrap();
        let expected = vec!["String", "Whitespace", "Flow", "Whitespace", "FunctionId"];
        assert_eq!(result, expected);
    }

    #[test]
    fn bin_ops_should_parse_concatinate_operator() {
        let result = parse("' ' ++ \"Hello\"").unwrap();
        let expected = vec!["Char", "Whitespace", "DoublePlus", "Whitespace", "String"];
        assert_eq!(result, expected);
    }

    #[test]
    fn structs_should_parse_struct_type_definition() {
        let result = parse("#MyStruct = #Struct<x: #Int, y: #Int>").unwrap();
        let expected = vec![
            "TypeId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "TypeId",
            "LessThan",
            "FieldId",
            "Whitespace",
            "TypeId",
            "Comma",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "TypeId",
            "GreaterThan",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn structs_should_parse_struct_creation() {
        let result = parse("point = { x: 3, y: 4 }").unwrap();
        let expected = vec![
            "VariableId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "LeftCurlyBracket",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "Integer",
            "Comma",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "Integer",
            "Whitespace",
            "RightCurlyBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn structs_should_parse_field_access() {
        let result = parse("point.x == 3").unwrap();
        let expected = vec![
            "VariableId",
            "Period",
            "VariableId",
            "Whitespace",
            "DoubleEquals",
            "Whitespace",
            "Integer",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn structs_should_parse_struct_editing() {
        let result = parse("{ point <- x: point.x + 1, y: point.y + 1 }").unwrap();
        let expected = vec![
            "LeftCurlyBracket",
            "Whitespace",
            "VariableId",
            "Whitespace",
            "LeftArrow",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "VariableId",
            "Period",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Comma",
            "Whitespace",
            "FieldId",
            "Whitespace",
            "VariableId",
            "Period",
            "VariableId",
            "Whitespace",
            "Plus",
            "Whitespace",
            "Integer",
            "Whitespace",
            "RightCurlyBracket",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_parse_new_lines() {
        let result = parse("Main = @IO.Print\n  \"Hello World!\"").unwrap();
        let expected = vec![
            "FunctionId",
            "Whitespace",
            "Equals",
            "Whitespace",
            "NamespaceId",
            "Period",
            "FunctionId",
            "Newline",
            "Whitespace",
            "String",
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn should_raise_error_if_namespace_does_not_begin_with_a_capital_letter() {
        let result = parse("@io.Print");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_if_type_does_not_begin_with_a_capital_letter() {
        let result = parse("#type");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unknown_or_token() {
        let result = parse("true | false");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unknown_and_token() {
        let result = parse("true & false");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unknown_other_token() {
        let result = parse("true $ false");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unclosed_string() {
        let result = parse("\"unclosed string");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_unclosed_char() {
        let result = parse("'c");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_raise_error_for_invalid_floats() {
        let result = parse("3.14.6");
        assert_eq!(result.is_err(), true);
    }

    fn parse(input: &str) -> super::Result<Vec<String>> {
        let mut t = super::Tokenizer::build(input);
        let tokens = t.execute()?;
        let mut output = Vec::new();
        for token in tokens {
            output.push(format!("{:?}", token.token_type));
        }
        Ok(output)
    }
}
