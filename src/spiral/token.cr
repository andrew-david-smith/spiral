class Token
  ALLOWED_TYPES = [
    "KW_NAMESPACE",
    "KW_EXPOSING",
    "KW_IMPORT",
    "KW_LET",
    "KW_IN",
    "KW_IF",
    "KW_ELSE",
    "KW_MATCH",
    "KW_WHEN",
    "KW_TRUE",
    "KW_FALSE",
    "NAMESPACE_ID",
    "FUNCTION_ID",
    "TYPE_ID",
    "FIELD_ID",
    "VARIABLE_ID",
    "INTEGER",
    "FLOAT",
    "CHAR",
    "STRING",
    "SQ_BR_L",
    "SQ_BR_R",
    "BR_L",
    "BR_R",
    "C_BR_L",
    "C_BR_R",
    "UNDERSCORE",
    "COMMA",
    "COLON",
    "OR",
    "AND",
    "LT",
    "GT",
    "LTE",
    "GTE",
    "NOT",
    "EQ",
    "NEQ",
    "DBL_EQ",
    "DBL_PLUS",
    "FLOW",
    "ADD",
    "MINUS",
    "DIV",
    "MULT",
    "POW",
    "PERIOD",
    "WHITESPACE",
    "NEWLINE",
    "LEFT_ARROW",
  ]

  getter :type

  def initialize(@value : String, @type : String, @line_number : Int32, @begins : Int32, @ends : Int32)
    unless ALLOWED_TYPES.includes?(@type)
      raise "Parser Error: Unknown type #{@type}"
    end
  end
end
