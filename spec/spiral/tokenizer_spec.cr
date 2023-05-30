require "../spec_helper"
require "../../src/spiral/tokenizer.cr"
require "../../src/spiral/token.cr"
require "../../src/spiral/tokenizer_exception.cr"

describe Spiral::Tokenizer do
  context "namespaces" do
    it "should parse with specific exposes" do
      parse("namespace @Maths exposing [Function variable #Type]").should eq(
        ["KW_NAMESPACE",
         "WHITESPACE",
         "NAMESPACE_ID",
         "WHITESPACE",
         "KW_EXPOSING",
         "WHITESPACE",
         "SQ_BR_L",
         "FUNCTION_ID",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "TYPE_ID",
         "SQ_BR_R"]
      )
    end

    it "should parse with wildcard exposes" do
      parse("namespace @Maths exposing _").should eq(
        ["KW_NAMESPACE",
         "WHITESPACE",
         "NAMESPACE_ID",
         "WHITESPACE",
         "KW_EXPOSING",
         "WHITESPACE",
         "UNDERSCORE"]
      )
    end
  end

  context "imports" do
    it "should parse with no specific imports" do
      parse("import @Maths").should eq(["KW_IMPORT", "WHITESPACE", "NAMESPACE_ID"])
    end

    it "should parse with specific imports" do
      parse("import @Maths exposing [Function variable #Type]").should eq(
        ["KW_IMPORT",
         "WHITESPACE",
         "NAMESPACE_ID",
         "WHITESPACE",
         "KW_EXPOSING",
         "WHITESPACE",
         "SQ_BR_L",
         "FUNCTION_ID",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "TYPE_ID",
         "SQ_BR_R"]
      )
    end
  end

  context "Function Definition" do
    it "should parse function type definition" do
      parse("AddTwo : #Fn<#Int,#Int>").should eq(
        ["FUNCTION_ID",
         "FUNCTION_ID",
         "WHITESPACE",
         "COLON",
         "WHITESPACE",
         "TYPE_ID",
         "LT",
         "TYPE_ID",
         "COMMA",
         "TYPE_ID",
         "GT"]
      )
    end

    it "should parse function definition" do
      parse("Main = @IO.Print(AddTwo 3)").should eq(
        ["FUNCTION_ID",
         "WHITESPACE",
         "EQ",
         "WHITESPACE",
         "NAMESPACE_ID",
         "PERIOD",
         "FUNCTION_ID",
         "BR_L",
         "FUNCTION_ID",
         "FUNCTION_ID",
         "WHITESPACE",
         "INTEGER",
         "BR_R"]
      )
    end
  end

  context "Type Definition" do
    it "should parse type definitions" do
      parse("#Number = #Int || #Float").should eq(
        ["TYPE_ID",
         "WHITESPACE",
         "EQ",
         "WHITESPACE",
         "TYPE_ID",
         "WHITESPACE",
         "OR",
         "WHITESPACE",
         "TYPE_ID"]
      )
    end
  end

  context "literals" do
    it "should parse booleans" do
      parse("true || false").should eq(
        ["KW_TRUE", "WHITESPACE", "OR", "WHITESPACE", "VARIABLE_ID"]
      )
    end

    it "should parse integers" do
      parse("2 + 4").should eq(
        ["INTEGER", "WHITESPACE", "ADD", "WHITESPACE", "INTEGER"]
      )
    end

    it "should parse floats" do
      parse("3.14 + 6.9").should eq(
        ["FLOAT", "WHITESPACE", "ADD", "WHITESPACE", "FLOAT"]
      )
    end

    it "should parse characters and strings" do
      parse("'a' ++ \"hello\"").should eq(
        ["CHAR", "WHITESPACE", "DBL_PLUS", "WHITESPACE", "STRING"]
      )
    end

    it "should parse lists" do
      parse("[1 2 3]").should eq(
        ["SQ_BR_L",
         "INTEGER",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "INTEGER",
         "SQ_BR_R"]
      )
    end
  end

  context "let statements" do
    it "should parse a let statement" do
      parse("let\n  twentyFour = 3 * 8\n  sixteen = 4 ^ 2\nin\n  twentyFour + sixteen").should eq(
        ["KW_LET",
         "NEWLINE",
         "WHITESPACE",
         "VARIABLE_ID",
         "FUNCTION_ID",
         "WHITESPACE",
         "EQ",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "MULT",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "EQ",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "POW",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "KW_IN",
         "NEWLINE",
         "WHITESPACE",
         "VARIABLE_ID",
         "FUNCTION_ID",
         "WHITESPACE",
         "ADD",
         "WHITESPACE",
         "VARIABLE_ID"]
      )
    end
  end

  context "if statements" do
    it "should parse without else if clause" do
      parse("if key == 40\n  n + 1\nelse\n  n").should eq(
        ["KW_IF",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "DBL_EQ",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "ADD",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "KW_ELSE",
         "NEWLINE",
         "WHITESPACE",
         "VARIABLE_ID"]
      )
    end

    it "should parse with else if clause" do
      parse("if key == 40\n  n + 1\nelse if key == 38\n  n - 1\nelse\n  n").should eq(
        ["KW_IF",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "DBL_EQ",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "ADD",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "KW_ELSE",
         "WHITESPACE",
         "KW_IF",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "DBL_EQ",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "MINUS",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "KW_ELSE",
         "NEWLINE",
         "WHITESPACE",
         "VARIABLE_ID"]
      )
    end
  end

  context "case statements" do
    it "should parse without when clauses" do
      parse("match n\nelse\n  1").should eq(
        ["KW_MATCH",
         "WHITESPACE",
         "VARIABLE_ID",
         "NEWLINE",
         "KW_ELSE",
         "NEWLINE",
         "WHITESPACE",
         "INTEGER"]
      )
    end

    it "should parse with when clauses" do
      parse("match n\nwhen 0\n  1\nwhen 1\n  1\nelse\n  Fib(n-1) + Fib(n-2)").should eq(
        ["KW_MATCH",
         "WHITESPACE",
         "VARIABLE_ID",
         "NEWLINE",
         "KW_WHEN",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "KW_WHEN",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "WHITESPACE",
         "INTEGER",
         "NEWLINE",
         "KW_ELSE",
         "NEWLINE",
         "WHITESPACE",
         "FUNCTION_ID",
         "BR_L",
         "VARIABLE_ID",
         "MINUS",
         "INTEGER",
         "BR_R",
         "WHITESPACE",
         "ADD",
         "WHITESPACE",
         "FUNCTION_ID",
         "BR_L",
         "VARIABLE_ID",
         "MINUS",
         "INTEGER",
         "BR_R"]
      )
    end
  end

  context "binOps" do
    it "should parse simple maths operators" do
      parse("1 + 2 - 3 * 4 / 5 ^ 6").should eq(
        ["INTEGER",
         "WHITESPACE",
         "ADD",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "MINUS",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "MULT",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "DIV",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "POW",
         "WHITESPACE",
         "INTEGER"]
      )
    end

    it "should parse boolean operators" do
      parse("!true || false && true").should eq(
        ["NOT",
         "KW_TRUE",
         "WHITESPACE",
         "OR",
         "WHITESPACE",
         "KW_FALSE",
         "WHITESPACE",
         "AND",
         "WHITESPACE",
         "VARIABLE_ID"]
      )
    end

    it "should parse flow operator" do
      parse("\"Hello\" |> Output").should eq(
        ["STRING", "WHITESPACE", "FLOW", "WHITESPACE", "FUNCTION_ID"]
      )
    end

    it "should parse concatinate operator" do
      parse("' ' ++ \"Hello\"").should eq(
        ["CHAR", "WHITESPACE", "DBL_PLUS", "WHITESPACE", "STRING"]
      )
    end
  end

  context "structs" do
    it "should parse struct type definition" do
      parse("#MyStruct = #Struct<x: #Int, y: #Int>").should eq(
        ["TYPE_ID",
         "WHITESPACE",
         "EQ",
         "WHITESPACE",
         "TYPE_ID",
         "LT",
         "VARIABLE_ID",
         "WHITESPACE",
         "TYPE_ID",
         "COMMA",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "TYPE_ID",
         "GT"]
      )
    end

    it "should parse struct creation" do
      parse("point = { x: 3, y: 4 }").should eq(
        ["VARIABLE_ID",
         "WHITESPACE",
         "EQ",
         "WHITESPACE",
         "C_BR_L",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "INTEGER",
         "COMMA",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "C_BR_R"]
      )
    end

    it "should parse struct field access" do
      parse("point.x == 3").should eq(
        ["VARIABLE_ID",
         "PERIOD",
         "VARIABLE_ID",
         "WHITESPACE",
         "DBL_EQ",
         "WHITESPACE",
         "INTEGER"]
      )
    end

    it "should parse struct editing" do
      parse("{ point <- x: point.x + 1, y: point.y + 1 }").should eq(
        ["C_BR_L",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "LEFT_ARROW",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "VARIABLE_ID",
         "PERIOD",
         "VARIABLE_ID",
         "WHITESPACE",
         "ADD",
         "WHITESPACE",
         "INTEGER",
         "COMMA",
         "WHITESPACE",
         "VARIABLE_ID",
         "WHITESPACE",
         "VARIABLE_ID",
         "PERIOD",
         "VARIABLE_ID",
         "WHITESPACE",
         "ADD",
         "WHITESPACE",
         "INTEGER",
         "WHITESPACE",
         "C_BR_R"]
      )
    end
  end

  it "should parse new lines" do
    parse("Main = @IO.Print\n  \"Hello World!\"").should eq(
      ["FUNCTION_ID",
       "WHITESPACE",
       "EQ",
       "WHITESPACE",
       "NAMESPACE_ID",
       "PERIOD",
       "FUNCTION_ID",
       "NEWLINE",
       "WHITESPACE",
       "STRING"]
    )
  end

  context "errors" do
    it "should raise error if namespace does not begin with a capital letter" do
      expect_raises(Spiral::TokenizerException) do
        parse("@io.Print")
      end
    end

    it "should raise error if type does not begin with a capital letter" do
      expect_raises(Spiral::TokenizerException) do
        parse("#type")
      end
    end

    it "should raise error if | is called" do
      expect_raises(Spiral::TokenizerException) do
        parse("true | false")
      end
    end

    it "should raise error if & is called" do
      expect_raises(Spiral::TokenizerException) do
        parse("true & false")
      end
    end
  end
end

def parse(input : String)
  Spiral::Tokenizer.tokenize(input).map do |token|
    token.type
  end
end
