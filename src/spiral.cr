require "./spiral/tokenizer"
require "./spiral/token"

module Spiral
  VERSION = "0.1.0"

  begin
    puts Tokenizer.tokenize("Main = @IO.Print \"Hello World!\"")
    puts Tokenizer.tokenize("5 + 4.5 + @Math.pi")
  rescue e : Spiral::Exception
    puts e.to_s
  end
end
