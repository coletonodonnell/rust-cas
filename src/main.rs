mod tree;
use tree::token as token;


fn main() {
    let mut input = String::new();

    println!("Input: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let v: Vec<&str> = input.split_ascii_whitespace().collect();
    let mut solid: Vec<String> = Vec::new();
    for i in v {
        solid.push(i.to_string())
    }

    let solid_length: usize = solid.len();

    if solid[solid_length - 1].contains("\n") {
        solid[solid_length - 1] = solid[solid_length - 1].replace("\n", "");
    }
    
    let a: Vec<token::Token> = token::tokenize(solid);
    // let b: (Vec<token::Token>, Vec<(i32, i32)>) = tree::fix_groups(a);
    // let c: Vec<token::Token> = b.0;
    // let d: Vec<(i32, i32)> = b.1;
    // println!("{:?}", c);
    // split_locater(c, d);

    println!("Before fix: {:?}", a);
    let b: tree::Node = tree::process(a);
    println!("After fix: {:#?}", b);

}