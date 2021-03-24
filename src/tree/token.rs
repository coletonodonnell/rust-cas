// TODO: Add logic for VAR combination (eg. 32x) to be equivelant to [NUM(32.00), MUL, VAR("x")]
// The Token type. Includes Addition operation, Multiplication operation, Division operation, Exponent operation, Variables, Numbers (as floats) and Grouping.
use std::panic;
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    ADD,
    MUL,
    DIV,
    EXP,
    VAR(String),
    NUM(f32),
    LGROUP,
    RGROUP,
}

// Creates a Token::ADD  when applicable, otherwise return None
fn to_add(to_token: String) -> Option<Token> {
    if to_token == "+".to_string() {
        return Some(Token::ADD)
    } else {
        return None
    }
}

// Creates a Token::MUL when applicable, otherwise return None
fn to_multiply(to_token: String) -> Option<Token> {
    if to_token == "*".to_string() {
        return Some(Token::MUL)
    } else {
        return None
    }
}

// Creates a Token::DIV when applicable, otherwise return None
fn to_div(to_token: String) -> Option<Token> {
    if to_token == "/".to_string() {
        return Some(Token::DIV)
    } else {
        return None
    }
}

// Creates a Token::NUM when applicable, otherwise return None
fn to_float(to_token: String) -> Option<Token> {
    match to_token.parse::<f32>() {
            Ok(f) => return Some(Token::NUM(f)),
            Err(_e) => return None,
        };
}

// Actually does the Tokenizing. Basically it takes the String and throws it at all the to_token functions. After it makes it way through the if statements,
// it will eventually return a Token type.
fn tokenizer(to_token: String) -> Token {
    let mut a = to_float(to_token.clone());
    if a.is_none() {
        a = to_add(to_token.clone());
        if a.is_none() {
            a = to_multiply(to_token.clone());
            if a.is_none() {
                a = to_div(to_token.clone());
                if a.is_none() {
                    return Token::VAR(to_token.clone())
                } else {
                    return a.unwrap()
                }
            } else {
                return a.unwrap()
            }
        } else {
            return a.unwrap()
        }
    } else {
        return a.unwrap()
    }
}

// Called when you want to Tokenize your vector of Strings.
pub fn tokenize(string_vector: Vec<String>) -> Vec<Token> {
    let mut token_vector: Vec<Token> = Vec::new(); 
    for x in string_vector.iter() {
        let token;
        let size_of_x: usize = x.as_str().len();

        // If it begins with a "(", continue
        if x.as_str().get(0..1).unwrap() == "(" {
            let mut past: i32 = 0;
            // Push All LGROUP values.
            for a in 1..x.as_str().len() as i32 {
                if x.as_str().get(past as usize..a as usize).unwrap() == "(" {
                    past = a;
                    token_vector.push(Token::LGROUP);
                } else {
                    past = a;
                    break;
                }
            }
            // if there is stuff after "(", push it.
            if x.as_str().get(past as usize..).unwrap() != "(" {
                token = tokenizer(x.as_str().get(past as usize..).unwrap().to_string());
                token_vector.push(token);
            } else {
                token_vector.push(Token::LGROUP);
            }
            
        // Else If it ends with a ")", continue
        } else if x.as_str().get(size_of_x - 1..size_of_x).unwrap() == ")" {
            let mut past: i32 = 0;
            // if there is stuff before ")", push it.
            if x.as_str().get(0..1).unwrap().to_string() != ")" {
                token = tokenizer(x.as_str().get(0..1).unwrap().to_string());
                token_vector.push(token);
            }

            // Push all ")"
            for a in 1..(x.as_str().len() + 1) as i32 {
                if x.as_str().get(past as usize..a as usize).unwrap() == ")" {
                    token_vector.push(Token::RGROUP);
                }
                past = a;
            }
            
        // Else If it begins with a "-", continue
        } else if x.as_str().get(0..1).unwrap() == "-" {
            // If there is stuff after "-", push the equivelant expression -1 * x, where x is what follows after "-"
            if x.as_str().get(1..).unwrap().to_string().is_empty() != true { 
                token_vector.push(Token::NUM(-1.00));
                token_vector.push(Token::MUL);
                token = tokenizer(x.as_str().get(1..).unwrap().to_string());
                token_vector.push(token);
            // if there isn't stuff after "-", push + -1 *
            } else {
                token_vector.push(Token::ADD);
                token_vector.push(Token::NUM(-1.00));
                token_vector.push(Token::MUL);
            }
        // Else If it begins with a "^", continue
        } else if x.as_str().get(0..1).unwrap() == "^" {
            // Push exponent token.
            token_vector.push(Token::EXP);
            // If there is stuff after "^", push the stuff.
            if x.as_str().get(1..).unwrap().to_string().is_empty() != true {
                token_vector.push(tokenizer(x.as_str().get(1..).unwrap().to_string()));
            }
        // Else, tokenize and push x
        } else {
            token = tokenizer(x.to_string());
            token_vector.push(token);
        }
    }
    // Return the tokenized vector
    return token_vector
}

fn rm_sides_add_mul(mut token_vector: Vec<Token>, mut group_locations: Vec<(i32, i32)>) -> (Vec<Token>, Vec<(i32, i32)>) {
    // Check for "useless group" (on the outskirts of the equation) and removes them, repeats just in case there are multiple of them.
    while group_locations.is_empty() != true && group_locations[0].0 == 0 && group_locations[0].1 == (token_vector.len() - 1) as i32 {
        token_vector.remove(0 as usize);
        token_vector.remove((token_vector.len() - 1) as usize);
        group_locations.remove(0 as usize);
        group_locations = find_groups(token_vector.clone());
    }


    let a: i32 = group_locations.len() as i32;
    let mut i: i32 = 0;
    let mut b: bool = false;

    while a > i {
        for a in group_locations.clone() {
            if a.0 > 0 {
                match token_vector[(a.0 - 1) as usize] {
                    // Check for NUM, VAR, or RGROUP value before LGROUP value, and if so add a MUL Value between them.
                    Token::NUM(_) => {
                        token_vector.insert(a.0 as usize, Token::MUL);
                        b = true;
                        break;
                    }
                    Token::VAR(_) => {
                        token_vector.insert(a.0 as usize, Token::MUL);
                        b = true;
                        break;
                    }
                    Token::RGROUP => {
                        token_vector.insert(a.0 as usize, Token::MUL);
                        b = true;
                        break;
                    }
                    // If there is an ADD parameter before a LGROUP, remove the LGROUP and its respective RGROUP value.
                    Token::ADD => {
                        token_vector.remove(a.1 as usize);
                        token_vector.remove(a.0 as usize);
                        b = true;
                        break;
                    }
                    _ => {}

                }
            }
        }
        if b == true {
            group_locations = find_groups(token_vector.clone());
            b = false;
        }
        i += 1;
    }

    return (token_vector, group_locations)
}

// Find groupings.
fn find_groups(token_vector: Vec<Token>) -> Vec<(i32, i32)> {
    let mut total_group: i32 = 0;

    // find total
    for i in 0..token_vector.len() as i32 {
        match token_vector[i as usize] {
            Token::LGROUP => {
                total_group += 1;
            } Token::RGROUP => {
                total_group += 1;
            }
            _ => {}
        }
    }

    // If the total group is uneven, crash.
    if total_group % 2 != 0 {
        panic!("There must be an even number of group symbols!")
    }

    // Declare group_locations
    let mut group_locations: Vec<(i32, i32)> = Vec::new();

    // if there isn't any groups at all, just return the empty vector
    if total_group == 0 {
        return group_locations
    }

    // declare variables logic for the locating of parenthesis beginning and end locations
    let mut unsorted_lgroup_locations: Vec<i32> = Vec::new();
    let mut left_right_value: usize = 0;
    let mut right_right_value: usize = 0;

    // search for all left values before the first right
    for i in 0..token_vector.len() as i32 {
        match token_vector[i as usize] {
            Token::LGROUP => {
                unsorted_lgroup_locations.push(i);
            } Token::RGROUP => {
                left_right_value = i as usize;
                // if there is only one left value, push it and return group_locations
                if total_group / 2 == 1 {
                    group_locations.push((unsorted_lgroup_locations.pop().unwrap(), left_right_value as i32));
                    return group_locations
                }
                // break unless so as to search for the rest
                break;
            }
            _ => {}
        }
    }

    // Loop over until the group locations are empty
    while unsorted_lgroup_locations.is_empty() != true {
        // push right left and known right
        group_locations.push((unsorted_lgroup_locations.pop().unwrap(), left_right_value as i32));

        // look for next right value between left and token_vector.len(), whilst also marking down left values
        for i in left_right_value as i32 + 1..token_vector.len() as i32 {
            match token_vector[i as usize] {
                Token::LGROUP => {
                    unsorted_lgroup_locations.push(i as i32);
                }
                Token::RGROUP => {
                    if i != left_right_value as i32 {
                        right_right_value = i as usize;
                        break;
                    }
                }
                _ => {}
            }
        }
        // set left_right_value as the right_right_value, thus moving the block over.
        left_right_value = right_right_value;
    }

    let mut sorted_group_locations: Vec<(i32, i32)> = Vec::new();
    sorted_group_locations.push((-1, -1));
    let mut next: (i32, i32);
    let mut insertion: i32;
    let mut k: i32;
    let mut copy: (i32, i32);
    for i in 0..group_locations.len() as i32 {
        next = group_locations[i as usize];
        insertion = 0;
        k = i;
        while k > 0 && insertion == 0 {
            if next.0 > sorted_group_locations[k as usize - 1].0 {
                insertion = k;
            } else {
                if k == sorted_group_locations.len() as i32 {
                    sorted_group_locations.push(sorted_group_locations[k as usize - 1]);
                } else {
                    copy = sorted_group_locations[(k as usize) - 1];
                    let _ = std::mem::replace(&mut sorted_group_locations[k as usize], copy);
                }
            }
            k -= 1;
        }
        if insertion == sorted_group_locations.len() as i32 {
            sorted_group_locations.push(next);
        } else {
            let _ = std::mem::replace(&mut sorted_group_locations[insertion as usize], next);
        }
    }

    return sorted_group_locations
}

// Orchestrates the group fixes and returns the fixed Vector and Group locations.
pub fn fix_groups(mut token_vector: Vec<Token>) -> (Vec<Token>, Vec<(i32, i32)>) {
    let mut group_locations: Vec<(i32, i32)> = find_groups(token_vector.clone());
    if group_locations.is_empty() != true {
        let a: (Vec<Token>, Vec<(i32, i32)>);
        a = rm_sides_add_mul(token_vector.clone(), group_locations.clone());
        token_vector = a.0;
        group_locations = a.1;
    }
    return (token_vector, group_locations)
}