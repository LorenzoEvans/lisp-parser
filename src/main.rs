fn main() {
    println!("{:?}", make_ast(tokenize(String::from("(first (list 1 (+ 2 3) 9))"))))
}

#[derive(Debug,)]
struct Token{content: (String, usize)}
#[derive(Debug)]
enum TokenType {
    Opening(Token),
    Closing(Token),
    Char(Token),
    Operator(Token),
    Operand(Token),
    WhiteSpace(Token)
}
fn tokenize(code: String) -> Vec<TokenType> {

    let mut symbol_stack: Vec<TokenType> = Vec::new();
    let mut ast_string = String::new();

    for (i, c) in code.char_indices() {
        if c.is_alphabetic() {
            // let identifier = c.to_string() + &i.to_string();
            symbol_stack.push(TokenType::Char(Token{content:( c.to_string(), i)}));
        }
        match c {
            '(' => {
                symbol_stack.push(TokenType::Opening(Token{content:( c.to_string(), i)}));
                ast_string.push('[');
            }
            ')' => {
                symbol_stack.push(TokenType::Closing(Token{content:( c.to_string(), i)}));
                ast_string.push(']');
            }
            ' ' => {
                
                symbol_stack.push(TokenType::WhiteSpace(Token{content:(" ".to_string(), i)}));
            }
            '+' => {
                symbol_stack.push(TokenType::Operator(Token{content:( c.to_string(), i)}));
                ast_string.push(' ');
                ast_string.push('+');
                ast_string.push(' ');
            }
            x => {
                // handle alphabets via wild card,
                // let everything else drop possible.
                if x.is_numeric() {
                    symbol_stack.push(TokenType::Operand(Token{content:( c.to_string(), i)}));
                    ast_string.push(x);
                }
                else if c.is_alphabetic() {
                    // let identifier = c.to_string() + &i.to_string();
                    symbol_stack.push(TokenType::Char(Token{content:( c.to_string(), i)}));
                }
                else {
                    continue
                }
            }
        }
    }
    symbol_stack
}

fn make_ast(symbol_stack: Vec<TokenType>) -> (){
    for i in symbol_stack {
        let mut ast = vec![];
        match &mut i {
            TokenType::Opening(i) => {
            ast.push("(")
            }
            TokenType::Closing(i) => {
                ast.push(")")
            }
            TokenType::Char(i) => {
                let (token, index) = i;
                // ast.push()
            }
        }
        println!("{:?}", i);
    }
}

   // *** We're skipping the numbers ***


    // We have to parse a string.
        // A string is just a vector of bytes.
    // We want to match certain parts of the string abstractly,
    // and capture them in a nested format.
    // Alright so we can capture individual chars, as well as whitespace,
    // in terms of the Lisp code, we need to differentiate between:
        // delimiters
        // symbols
        // operators
            // whether or not operators are unary or binary
        // operands
        // whitespace
