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


    let mut ast = make_ast("(first (list 1 (+ 2 3) 9))".to_string());
    println!("{:?}", ast);
    // let obj_vec: Vec<T> = Vec::new(): 
        // [+ [string [ int ]]]
      


    
}
fn make_ast(source: String) -> BTreeMap<i32, String> {
    // Remove all the parentheses from the source code as they won't be evaluated.
    let source_str = str::replace(&source.to_string(), "(", " ");

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
    // { 0: , 1: "list", 2: "1", 3: "+", 4: "2", 5: "3)", 6: "9))" }
    // make_ast output: {
        // 0: "first", 
            // 1: "list", 2: "1", 
                // 3: "+", 
                    // 4: "2", 5: "3", 6: "9"} 
                    // - here's where the lack of structure costs us.
                        // 1: "list", is a form that generates a cons list of an arbitrary number of values.
                        // We don't know, at this point, however, whether or not 9 is a value to be consed
                        // onto a list, or fed into our `+` operator.
                        // hack: leave closing parentheses in string, split string values if possible,
                        // if the second character is a parenthesis, we know a little bit more about nesting

    // How to logically parse this, while missing some structural information, but maintaining order.
        // The simplest operations to evaluate are primitive operations, on primitive values,
        // such as `+` and unsigned integers.
        // For these operations, we need to find the operator, and then grab the next two numbers.
            //  We can''t exactly assume two, as prefix notation accommodates arbitrary numbers of arguments.\
        // We can indicate nesting as we know strings are symbols, and symbols indicate beginnings of forms.
        // "first" indicates one level of nesting
            // We also know what this function does, so that helps.
        // "list" indicates another level 
            // We also know what this function does, so that helps.
        // We know that + is a function operator, so this also indicates a level of nesting as well.
        
}