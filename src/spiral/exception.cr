class Spiral::Exception < Exception
  def initialize(@message : String, @line_number : Int32, @begins : Int32, @ends : Int32)
  end

  def message
    "Error at L#{@line_number}: #{@message}"
  end
end
