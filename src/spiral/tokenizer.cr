require "./tokenizer_exception.cr"

class Spiral::Tokenizer
  getter tokens : Array(Token)

  def self.tokenize(input : String)
    new(input).tokens
  end

  def initialize(@input : String)
    @current_index = 0
    @tokens = [] of Token
    @line_number = 1
    create_tokens
  end

  def create_tokens
    while @input.size > @current_index
      if current_token == '#'
        create_type_id
      elsif current_token == '@'
        create_namespace_id
      elsif current_token == '\''
        create_char
      elsif current_token == '"'
        create_string
      elsif current_token == '['
        @tokens << Token.new(current_token.to_s, "SQ_BR_L", @line_number, @current_index, @current_index)
      elsif current_token == ']'
        @tokens << Token.new(current_token.to_s, "SQ_BR_R", @line_number, @current_index, @current_index)
      elsif current_token == '('
        @tokens << Token.new(current_token.to_s, "BR_L", @line_number, @current_index, @current_index)
      elsif current_token == ')'
        @tokens << Token.new(current_token.to_s, "BR_R", @line_number, @current_index, @current_index)
      elsif current_token == '{'
        @tokens << Token.new(current_token.to_s, "C_BR_L", @line_number, @current_index, @current_index)
      elsif current_token == '}'
        @tokens << Token.new(current_token.to_s, "C_BR_R", @line_number, @current_index, @current_index)
      elsif current_token == '<'
        create_less_than
      elsif current_token == '>'
        create_greater_than
      elsif current_token == '_'
        @tokens << Token.new(current_token.to_s, "UNDERSCORE", @line_number, @current_index, @current_index)
      elsif current_token == ','
        @tokens << Token.new(current_token.to_s, "COMMA", @line_number, @current_index, @current_index)
      elsif current_token == ':'
        @tokens << Token.new(current_token.to_s, "COLON", @line_number, @current_index, @current_index)
      elsif current_token == '|'
        create_from_or
      elsif current_token == '&'
        create_from_and
      elsif current_token == '='
        create_from_equals
      elsif current_token == '!'
        create_from_not
      elsif current_token == '+'
        create_from_plus
      elsif current_token == '-'
        @tokens << Token.new(current_token.to_s, "MINUS", @line_number, @current_index, @current_index)
      elsif current_token == '/'
        @tokens << Token.new(current_token.to_s, "DIV", @line_number, @current_index, @current_index)
      elsif current_token == '*'
        @tokens << Token.new(current_token.to_s, "MULT", @line_number, @current_index, @current_index)
      elsif current_token == '^'
        @tokens << Token.new(current_token.to_s, "POW", @line_number, @current_index, @current_index)
      elsif current_token == '.'
        @tokens << Token.new(current_token.to_s, "PERIOD", @line_number, @current_index, @current_index)
      elsif current_token == ' '
        create_whitespace
      elsif ['\n', '\r'].includes?(current_token)
        create_newlines
      elsif current_token.ascii_uppercase?
        create_function_id
      elsif current_token.ascii_lowercase?
        create_word
      elsif current_token.ascii_number?
        create_number
      end

      @current_index += 1
    end
  end

  private def current_token
    @input[@current_index]
  end

  private def create_from_or
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if current_token == '|'
      value += current_token
      @tokens << Token.new(value, "OR", @line_number, start_index, @current_index)
    elsif current_token == '>'
      value += current_token
      @tokens << Token.new(value, "FLOW", @line_number, start_index, @current_index)
    else
      raise Spiral::TokenizerException.new(
        "Unknown character '|'",
        @line_number, start_index, @current_index, @input,
        "Did you mean '||' or '|>'?"
      )
    end
  end

  private def create_from_and
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if current_token == '&'
      value += current_token
      @tokens << Token.new(value, "AND", @line_number, start_index, @current_index)
    else
      raise Spiral::TokenizerException.new(
        "Unknown character '&'",
        @line_number, start_index, @current_index, @input,
        "Did you mean '&&'?"
      )
    end
  end

  private def create_from_equals
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if current_token == '='
      value += current_token
      @tokens << Token.new(value, "DBL_EQ", @line_number, start_index, @current_index)
    else
      @current_index -= 1
      @tokens << Token.new(value, "EQ", @line_number, start_index, @current_index)
    end
  end

  private def create_from_not
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if current_token == '='
      value += current_token
      @tokens << Token.new(value, "NEQ", @line_number, start_index, @current_index)
    else
      @current_index -= 1
      @tokens << Token.new(value, "NOT", @line_number, start_index, @current_index)
    end
  end

  private def create_from_plus
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if current_token == '+'
      value += current_token
      @tokens << Token.new(value, "DBL_PLUS", @line_number, start_index, @current_index)
    else
      @current_index -= 1
      @tokens << Token.new(value, "ADD", @line_number, start_index, @current_index)
    end
  end

  private def create_less_than
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if @input.size > @current_index && current_token == '='
      value += current_token
      @tokens << Token.new(value, "LTE", @line_number, start_index, @current_index)
    elsif @input.size > @current_index && current_token == '-'
      value += current_token
      @tokens << Token.new(value, "LEFT_ARROW", @line_number, start_index, @current_index)
    else
      @current_index -= 1
      @tokens << Token.new(value, "LT", @line_number, start_index, @current_index)
    end
  end

  private def create_greater_than
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if @input.size > @current_index && current_token == '='
      value += current_token
      @tokens << Token.new(value, "GTE", @line_number, start_index, @current_index)
    else
      @current_index -= 1
      @tokens << Token.new(value, "GT", @line_number, start_index, @current_index)
    end
  end

  private def create_char
    value = ""
    start_index = @current_index
    @current_index += 1

    while !(current_token == '\'' && @input[@current_index - 1] != '\\')
      value += current_token
      @current_index += 1
      if (@current_index >= @input.size)
        break
      end
    end

    @tokens << Token.new(value, "CHAR", @line_number, start_index, @current_index)
  end

  private def create_string
    value = ""
    start_index = @current_index
    @current_index += 1

    while !(current_token == '"' && @input[@current_index - 1] != '\\')
      value += current_token
      @current_index += 1
      if (@current_index >= @input.size)
        break
      end
    end

    @tokens << Token.new(value, "STRING", @line_number, start_index, @current_index)
  end

  private def create_namespace_id
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if !current_token.ascii_uppercase?
      raise Spiral::TokenizerException.new(
        "Namespace does not begin with a capital letter",
        @line_number, start_index, @current_index, @input,
        "Namespaces must always begin with a capital letter"
      )
    end

    while current_token.ascii_lowercase? || current_token.ascii_uppercase?
      value += current_token
      @current_index += 1
      if (@current_index >= @input.size)
        break
      end
    end

    @tokens << Token.new(value, "NAMESPACE_ID", @line_number, start_index, @current_index)
    @current_index -= 1
  end

  private def create_type_id
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if !current_token.ascii_uppercase?
      raise Spiral::TokenizerException.new(
        "Type does not begin with a capital letter",
        @line_number, start_index, @current_index, @input,
        "Types must always begin with a capital letter"
      )
    end

    while current_token.ascii_lowercase? || current_token.ascii_uppercase?
      value += current_token
      @current_index += 1
      if (@current_index >= @input.size)
        break
      end
    end

    @tokens << Token.new(value, "TYPE_ID", @line_number, start_index, @current_index)
    @current_index -= 1
  end

  private def create_whitespace
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if (@current_index >= @input.size)
      @tokens << Token.new(value, "WHITESPACE", @line_number, start_index, @current_index)
      return
    end

    while [' '].includes?(current_token)
      value += current_token
      @current_index += 1
      if (@current_index >= @input.size)
        break
      end
    end

    @tokens << Token.new(value, "WHITESPACE", @line_number, start_index, @current_index)
    @current_index -= 1
  end

  private def create_newlines
    value = current_token.to_s
    start_index = @current_index
    @line_number += 1
    @current_index += 1

    if (@current_index >= @input.size)
      @tokens << Token.new(value, "NEWLINE", @line_number, start_index, @current_index)
      return
    end

    while ['\n', '\r'].includes?(current_token)
      value += current_token
      @line_number += 1
      @current_index += 1
      if (@current_index >= @input.size)
        break
      end
    end

    @tokens << Token.new(value, "NEWLINE", @line_number, start_index, @current_index)
    @current_index -= 1
  end

  private def create_function_id
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    while current_token.ascii_lowercase?
      value += current_token
      @current_index += 1
      if (@current_index >= @input.size)
        break
      end
    end

    @tokens << Token.new(value, "FUNCTION_ID", @line_number, start_index, @current_index)
    @current_index -= 1
  end

  private def create_word
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1

    if @input.size > @current_index
      while current_token.ascii_lowercase? || current_token == ':'
        value += current_token
        @current_index += 1
        if (@current_index >= @input.size)
          break
        end
        if current_token == ':'
          break
        end
      end
    end

    if (@current_index >= @input.size)
      @tokens << Token.new(value, "VARIABLE_ID", @line_number, start_index, @current_index)
      return
    end

    if current_token == ':'
      @tokens << Token.new(value, "FIELD_ID", @line_number, start_index, @current_index)
    elsif value == "namespace"
      @tokens << Token.new(value, "KW_NAMESPACE", @line_number, start_index, @current_index)
    elsif value == "exposing"
      @tokens << Token.new(value, "KW_EXPOSING", @line_number, start_index, @current_index)
    elsif value == "import"
      @tokens << Token.new(value, "KW_IMPORT", @line_number, start_index, @current_index)
    elsif value == "let"
      @tokens << Token.new(value, "KW_LET", @line_number, start_index, @current_index)
    elsif value == "in"
      @tokens << Token.new(value, "KW_IN", @line_number, start_index, @current_index)
    elsif value == "if"
      @tokens << Token.new(value, "KW_IF", @line_number, start_index, @current_index)
    elsif value == "else"
      @tokens << Token.new(value, "KW_ELSE", @line_number, start_index, @current_index)
    elsif value == "match"
      @tokens << Token.new(value, "KW_MATCH", @line_number, start_index, @current_index)
    elsif value == "when"
      @tokens << Token.new(value, "KW_WHEN", @line_number, start_index, @current_index)
    elsif value == "true"
      @tokens << Token.new(value, "KW_TRUE", @line_number, start_index, @current_index)
    elsif value == "false"
      @tokens << Token.new(value, "KW_FALSE", @line_number, start_index, @current_index)
    else
      @tokens << Token.new(value, "VARIABLE_ID", @line_number, start_index, @current_index)
    end
    @current_index -= 1
  end

  private def advance
    @current_index += 1
    if (@current_index >= @input.size)
      nil
    else
      current_token
    end
  end

  private def create_number
    value = current_token.to_s
    start_index = @current_index
    @current_index += 1
    period_used = false

    if @input.size > @current_index
      while (current_token.ascii_number? || current_token == '.')
        if current_token == '.'
          if !period_used
            period_used = true
          else
            raise Spiral::TokenizerException.new(
              "Number contains multiple periods",
              @line_number, start_index, @current_index + 1, @input,
              "Numbers may only contain 1 or 0 periods"
            )
          end
        end

        value += current_token
        @current_index += 1
        if (@current_index >= @input.size)
          break
        end
      end
    end

    if period_used
      @tokens << Token.new(value, "FLOAT", @line_number, start_index, @current_index)
    else
      @tokens << Token.new(value, "INTEGER", @line_number, start_index, @current_index)
    end
    @current_index -= 1
  end
end
