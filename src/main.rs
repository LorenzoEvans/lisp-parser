use std::collections::BTreeMap;
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
    
fn main() {
    let sourcestr = str::replace("(first (list 1 (+ 2 3) 9))", "(", " ").replace(")", " ");

    let mut ast_map = BTreeMap::new();
    
    let ast_tree_1: Vec<&str> = sourcestr.split(" ").collect();

    let mut counter = 0;
    for i in 0..ast_tree_1.len() {
        if ast_tree_1[i] != "" {
            ast_map.insert(counter, ast_tree_1[i]);
            counter += 1;
        }
    }

    println!("ast_map: {:?}", ast_map);

}


