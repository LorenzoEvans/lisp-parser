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

    println!("make_ast output: {:?}",make_ast("(first (list 1 (+ 2 3) 9))".to_string()));
    
}
fn make_ast(source: String) -> BTreeMap<i32, String> {
    // Remove all the parentheses from the source code as they won't be evaluated.
    let source_str = str::replace(&source.to_string(), "(", " ").replace(")", " ");

    let mut ast_tree = BTreeMap::new(); // Generated a new ast_map from a BTreeMap to
                                       // preserve order in which we encounter 'tokens'. 
    
    let ast_str: Vec<&str> = source_str.split(" ").collect(); // Split our source string on spaces
                                                                

    let mut counter = 0; // Create a counter to track the order of our tokens
                        // as the iterator in the following for loop will contain skips
                        // from removed spaces and parentheses.

    for i in 0..ast_str.len() { // Create iterator over our string for indexing.

        if ast_str[i] != "" { // We don't want any superfluous spaces.

            ast_tree.insert(counter, ast_str[i].to_string()); // Insert our token into our map, with an int key indicating
                                                // the tokens 'position'.
            counter += 1; // Manually increment the counter upon insertion.
        }
    }

    return ast_tree

}