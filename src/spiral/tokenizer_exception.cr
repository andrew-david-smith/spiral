require "./exception"

class Spiral::TokenizerException < Spiral::Exception
  RED    = "\033[0;31m"
  YELLOW = "\033[1;33m"
  NC     = "\033[0m"

  def initialize(@message : String, @line_number : Int32, @begins : Int32, @ends : Int32, @file : String, @help : String)
  end

  def message
    ["", error_message, show_line, highlight_issue, @help, ""].join("\n\r")
  end

  private def error_message
    "#{YELLOW}Parse Error at L#{@line_number}: #{@message}#{NC}"
  end

  private def show_line
    @file.lines[@line_number - 1]
  end

  private def highlight_issue
    RED + " " * @begins + "^" * (@ends - @begins) + NC
  end
end
