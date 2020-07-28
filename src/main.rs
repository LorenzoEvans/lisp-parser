use std::collections::HashMap;
use regex::Regex;
use std::fmt;
use regex::RegexSet;
use lazy_static::lazy_static;
fn main() {
    // println!("{:?}", make_tree(String::from("(first (list 1 (+ 2 3) 9))")));
    let regexstr = make_tree(String::from("(first (list 1 (+ 2 3) 9))"));
    let sourcestr = str::replace("(first (list 1 (+ 2 3) 9))", "(", " ");

    let re = Regex::new(r"[A-Za-z]").unwrap();
    // let nextr = sourcestr.replace_all("(", " ");
    let stringval: Vec<&str> = sourcestr.rsplit(" ").collect();
    let astvec: Vec<&str> = stringval.clone();
    let mut astmap: HashMap<usize, &str> = HashMap::new();
    for i in 0..astvec.len() {
        astmap.insert(i, astvec[i]);
    }

    // println!("regexstr {:?}", regexstr);
    // let vecstr: Vec<&str> = stringval.collect();

    // let sourcestr2 = String::from("(first (list 1 (+ 2 3) 9))");
    let ast_tree_1: Vec<&str> = sourcestr.split("(").collect();
    println!("ast tree: {:?}", ast_tree_1);

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

fn make_tree(source: String) {
    // so we want to loop through and grab a symbol 
    // we want to throw that symbol in a hashmap (so we can preserve nested structure)
    // once we grab a symbol, we need to know if the string is done.
    // if not, we continue looping (maybe with a next call, maybe typical iterator)
    // we can match for symbols as strings of chars, operators as non-alphanumeric symbols, and numbers as numbers
    
    // let mut ast: HashMap<i32, > = HashMap::new();
    // let mut symbol_vec = Vec::new();
    
    let re = Regex::new(r"\w+[^\(^\)[\+]]").unwrap();
    let mut str_vec: Vec<&str> = Vec::new();
    for caps in re.captures_iter(&source) {
        if let Some(cap) = caps.get(0) {
            let word = cap.as_str();
            str_vec.push(word); 
        }
        // let ast_symbol = symbol.to_string();
        // ast.insert(counter, symbol.as_str());
        // symbol_vec.push(symbol);
    }
    // for caps in symbol_vec {
    //     if let Some(cap) = caps.get(0) {
    //         let word = cap.as_str();
    //         str_vec.push(word);
    //     }
    // }
    println!("str_vec {:?}", str_vec);
}

