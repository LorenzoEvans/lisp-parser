use std::collections::HashMap;

fn main() {
    println!("{:?}", output_ast(make_ast(tokenize(String::from("(first (list 1 (+ 2 3) 9))")))))
}

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
    // whitespacex

#[derive(Debug,)]
struct Token{content: (String, usize)} // A token is our smallest building block.
#[derive(Debug)]
enum TokenType { // We use an enum, to differentiate between the types of tokens we'll encounter. 
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
            x => { // Using this wildcard to filter out characters and numbers because matching all the utf string characters is highly not feasible.
                if x.is_numeric() {
                    symbol_stack.push(TokenType::Operand(Token{content:( c.to_string(), i)}));
                    ast_string.push(x);
                }
                else if c.is_alphabetic() {
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

fn make_ast(symbol_stack: Vec<TokenType>) -> Vec<(String, usize)> {
    // Now that we're able to unwrap our enums, 
    // and push them into a vector, without whitespace or parentheses.
    // What we need to do now is figure out how to use the index of each character 
    // to rebuild the ast in order. 
    //  Generally, we can rely on numerical ordering, even with the gaps, because we don't need to know
    // what numbers lie between 3 and 9, to know that 9 is greater than 3's, 
    // or that the distance can't be greater than the number of 3's that can fit in 9.
        // The distance between integers x and y can never be greater than the number of x's that can fit inside y.
        // Can we *prove* this, though? *save for later*
    // We have to take care to cast our operands back to integer values.
    // # Issue One: We need to generate vectors, for each symbol we encounter.
        // # We can get away with fixed-arrays, where the expected length is one symbol, and the entire
        // subsequent nested forms- the issue this brings up is *typing*.
    // Other than that, we have to figure out looping over our tuples, which increase in order,
    // allowing us to check for a number skip, due to our missing whitespaces. (This seems shaky, relying on the removal of an empty char)
    
    let mut ast: Vec<(String, usize)> = Vec::new();
    for i in symbol_stack {
        
        match i {
            TokenType::Opening(Token { content }) => {
                continue 
            }
            TokenType::Char(Token { content }) => {
                ast.push((String::from(content.0), content.1))
            }
            TokenType::Operator(Token { content }) => {
                ast.push((String::from(content.0), content.1))
            }
            TokenType::Operand(Token { content }) => {
                ast.push((String::from(content.0), content.1))
            }
            _ => {
                continue
            }
        }
    }
    ast

}
// [("f", 1), ("i", 2), ("r", 3), ("s", 4), ("t", 5), ("l", 8), ("i", 9), ("s", 10), ("t", 11), ("1", 13), ("+", 16), ("2", 18), ("3", 20), ("9", 23)]
fn output_ast(ast: Vec<(String, usize)>) -> Vec<String> {

    let mut cur_idx = 0; // manual iterator for peeking forward
    let mut s_tree = vec![];
    for i in 0..ast.len() - 1 { // take the length so that our iterator is the item
        if (ast[i + 1].1 - ast[i].1) > 1 { // if the next items second value minus the prev items second value
                                          // is greater than 1, we've hit a skip in our string count and finished a symbol.
            // println!("index: {}", i);

        }
        cur_idx = ast[i].1;
        s_tree.push(ast[i].0.clone());
        // println!("{}", cur_idx);
    }
    s_tree.push(ast[ast.len() - 1].0.clone());
    s_tree
}