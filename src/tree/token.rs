// TODO: Add logic for VAR combination (eg. 32x) to be equivelant to [NUM(32.00), MUL, VAR("x")]
// The Token type. Includes Addition operation, Multiplication operation, Division operation, Exponent operation, Variables, Numbers (as floats) and Grouping.
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
            // Push the GROUP Token.
            token_vector.push(Token::LGROUP);
            // if there is stuff after "(", push it.
            if x.as_str().get(1..).unwrap().to_string().is_empty() != true {
                token = tokenizer(x.as_str().get(1..).unwrap().to_string());
                token_vector.push(token);
            }
        // Else If it ends with a ")", continue
        } else if x.as_str().get(size_of_x - 1..size_of_x).unwrap() == ")" {
            // if there is stuff before ")", push it.
            if x.as_str().get(0..size_of_x - 1).unwrap().to_string().is_empty() != true {
                token = tokenizer(x.as_str().get(0..size_of_x - 1).unwrap().to_string());
                token_vector.push(token);
            }
            // Push the GROUP Token.
            token_vector.push(Token::RGROUP);
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