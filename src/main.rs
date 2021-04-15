mod tree;
use tree::token as token;

fn main() {
    // Get user input
    let mut input = String::new();
    println!("Expression to simplify: ");
    std::io::stdin().read_line(&mut input).unwrap();

    // Create a vector of str objects based on the user input, splitting the expression via spaces and then collecting them. 
    let v: Vec<&str> = input.split_ascii_whitespace().collect();

    // Convert these to Strings
    let mut solid: Vec<String> = Vec::new();
    for i in v {
        solid.push(i.to_string())
    }

    // Get the length of the String vector
    let solid_length: usize = solid.len();

    // If the vector ends with a new line, remove it.
    if solid[solid_length - 1].contains("\n") {
        solid[solid_length - 1] = solid[solid_length - 1].replace("\n", "");
    }
    
    // Get our token vector.
    let a: Vec<token::Token> = token::tokenize(solid);
    println!("Token Vector: {:?}", a);
    
    // Proccess the vector.
    let b: Box<tree::Node> = tree::process(a);
    println!("After simplification: {:#?}", b);

}
