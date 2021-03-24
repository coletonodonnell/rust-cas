// All necessary functions to build a token binary tree, obeying the rules of recursive PEMDAS/recursive solving (eg. work the problem backwards.)
// Thing is, with this we are removing the actual S in PEMDAS, making it more PEMDA. Subtraction is an annoying thing, and it is much easier to 
// just remove it outright in favour of multiplication of negative 1.

pub mod token;

// Node scructure, representing the Data type, and a recursive definition of Option Nodes. The data_type can be any Token, and if it is a NUM or VAR, 
// the left and right nodes will always be None. Because this is a recursive structure, we will have to add it onto the heap instead of the stack.
// We do this with the Box function.
#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    data_type: token::Token,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
}

// Splits a vector into branches
fn vector_split(mut token_vector: Vec<token::Token>, split_location: i32) -> (Vec<token::Token>, Vec<(i32, i32)>, Vec<token::Token>, Vec<(i32, i32)>) {
    // let the right branch be the split off branch, including split location
    let mut right_branch: Vec< token::Token> = token_vector.split_off(split_location as usize);
    // fix right, if need be
    let a: (Vec<token::Token>, Vec<(i32, i32)>) = token::fix_groups(right_branch);
    // declare fixed stuff
    right_branch = a.0;
    // useless currently, empty assignment
    let _ = a.1;
    // fix left, if need be
    let b: (Vec<token::Token>, Vec<(i32, i32)>) = token::fix_groups(token_vector);
    // declare fixed stuff
    token_vector = b.0;
    let left_group_locations: Vec<(i32, i32)> = b.1;
    // look through right branch once more, if it isn't a LGROUP value, delete it
    match right_branch[0] {
        token::Token::LGROUP => {}
        _ => {
            let _ = right_branch.remove(0 as usize);
        }
    }
    // fix right branch once more
    let c: (Vec<token::Token>, Vec<(i32, i32)>) = token::fix_groups(right_branch);
    right_branch = c.0;
    let right_group_locations = c.1;
    // return it!
    return (token_vector, left_group_locations, right_branch, right_group_locations)
}

// Locate splits, return them as a tuple
fn split_locater(token_vector: Vec<token::Token>, group_locations: Vec<(i32, i32)>) -> (Vec<token::Token>, Vec<(i32, i32)>, Vec<token::Token>, Vec<(i32, i32)>, token::Token) {
    // Declare conditions
    let mut add_condition: Option<i32> = None;
    let mut div_condition: Option<i32> = None;
    let mut mul_condition: Option<i32> = None;
    let mut exp_condition: Option<i32> = None;
    let mut pass_bool: bool = false;

    // Declare return values, including branches, groups, and data type
    let data_type: token::Token;
    let left_branch: Vec<token::Token>;
    let left_group_locations: Vec<(i32, i32)>;
    let right_branch: Vec<token::Token>;
    let right_group_locations: Vec<(i32, i32)>;
    // If the token_vector is length 1, it always has a VAR or NUM value, so just return that as a node, with empty left and rights and such
    if token_vector.len() == 1 {
        match token_vector[0] {
            _ => {
                left_branch = Vec::new();
                left_group_locations = Vec::new();
                right_branch = Vec::new();
                right_group_locations = Vec::new();
                return (left_branch, left_group_locations, right_branch, right_group_locations, token_vector[0].clone());
            }
        }
    // else, continue
    } else {
        // for a in the token_vector length
        for a in 0..token_vector.len() as i32 {
            // Find each right thing, marking them down, if they are located in parenthesis pass them. 
            // These do represent the weightings as well, but these could've been written in any order.
            match token_vector[a as usize] {
                token::Token::ADD => {
                    for b in &group_locations {
                        if a > b.0 && a < b.1 {
                            pass_bool = true;
                            break;
                        }
                    }
                    if pass_bool == false {
                        add_condition = Some(a);
                    }
                    pass_bool = false;
                }
                token::Token::DIV => {
                    for b in &group_locations {
                        if a > b.0 && a < b.1 {
                            pass_bool = true;
                            break;
                        }
                    }
                    if pass_bool == false {
                        div_condition = Some(a);
                    }
                    pass_bool = false;
                }
                token::Token::MUL => {
                    for b in &group_locations {
                        if a > b.0 && a < b.1 {
                            pass_bool = true;
                            break;
                        }
                    }
                    if pass_bool == false {
                        mul_condition = Some(a);
                    }
                    pass_bool = false;
                }
                token::Token::EXP => {
                    for b in &group_locations {
                        if a > b.0 && a < b.1 {
                            pass_bool = true;
                            break;
                        }
                    }
                    if pass_bool == false {
                        exp_condition = Some(a);
                    }
                    pass_bool = false;
                }
                _ => {}
            }
        }
    }

    // Declare the splits
    let splits;

    // Match each one according to the weight, and if they exist, just return them. This system ensures proper reverse PEMDAS weighting. 
    match add_condition {
        Some(_) => {
            data_type = token::Token::ADD;
            splits = vector_split(token_vector, add_condition.unwrap());
            left_branch = splits.0;
            left_group_locations = splits.1;
            right_branch = splits.2;
            right_group_locations = splits.3;
            return (left_branch, left_group_locations, right_branch, right_group_locations, data_type);
        }
        None => {}
    }
    // Because MUL and DIV are actually weighted the same, we must locate the right most of each. I opted to look for this within the mul_condition match, by first matching mul_condition
    // matching div_condition, and then if both are true, comparing their weightings. This ensures that we never unwrap a None value, and it also will return the proper weighting.
    match mul_condition {
        Some(_) => {
            match div_condition {
                Some(_) => {
                    if mul_condition.unwrap() > div_condition.unwrap() {
                        data_type = token::Token::MUL;
                        splits = vector_split(token_vector, mul_condition.unwrap());
                        left_branch = splits.0;
                        left_group_locations = splits.1;
                        right_branch = splits.2;
                        right_group_locations = splits.3;
                        return (left_branch, left_group_locations, right_branch, right_group_locations, data_type);
                    } else {
                        data_type = token::Token::DIV;
                        splits = vector_split(token_vector, div_condition.unwrap());
                        left_branch = splits.0;
                        left_group_locations = splits.1;
                        right_branch = splits.2;
                        right_group_locations = splits.3;
                        return (left_branch, left_group_locations, right_branch, right_group_locations, data_type);
                    }
                }
                None => {
                    data_type = token::Token::MUL;
                    splits = vector_split(token_vector, mul_condition.unwrap());
                    left_branch = splits.0;
                    left_group_locations = splits.1;
                    right_branch = splits.2;
                    right_group_locations = splits.3;
                    return (left_branch, left_group_locations, right_branch, right_group_locations, data_type);
                }
            }
        }
        None => {}
    }
    match div_condition {
        Some(_) => {
            data_type = token::Token::DIV;
            splits = vector_split(token_vector, div_condition.unwrap());
            left_branch = splits.0;
            left_group_locations = splits.1;
            right_branch = splits.2;
            right_group_locations = splits.3;
            return (left_branch, left_group_locations, right_branch, right_group_locations, data_type);
        }
        None => {}
    }
    match exp_condition {
        Some(_) => {
            data_type = token::Token::EXP;
            splits = vector_split(token_vector, exp_condition.unwrap());
            left_branch = splits.0;
            left_group_locations = splits.1;
            right_branch = splits.2;
            right_group_locations = splits.3;
            return (left_branch, left_group_locations, right_branch, right_group_locations, data_type);
        }
        // Rust wants to see a return even here. This will never run if things go as they should.
        None => {
            println!("Should never print");
            data_type = token::Token::EXP;
            splits = vector_split(token_vector, 0);
            left_branch = splits.0;
            left_group_locations = splits.1;
            right_branch = splits.2;
            right_group_locations = splits.3;
            return (left_branch, left_group_locations, right_branch, right_group_locations, data_type);
        }
    }

}

// The actual creation of a node, including logic to determine left and right weighting.
fn node_creation(raw_node: (Vec<token::Token>, Vec<(i32, i32)>, Vec<token::Token>, Vec<(i32, i32)>, token::Token)) -> Option<Node> {
    let left_branch: Vec<token::Token> = raw_node.0;
    let left_group_locations: Vec<(i32, i32)> = raw_node.1;
    let right_branch: Vec<token::Token> = raw_node.2;
    let right_group_locations: Vec<(i32, i32)> = raw_node.3;
    let data_type_node: token::Token = raw_node.4;
    let a: Node;

    // If both of the branches are empty (eg. this is a VAR or NUM) just return this as a complete node (left and rights are empty.)
    if left_branch.is_empty() && right_branch.is_empty() {
        a = Node {
            data_type: data_type_node,
            left: Box::new(None),
            right: Box::new(None),
        };
    // If this isn't the case, we need to make sure we find what goes where, left vs. right. The weighting for this operation is as follows:
    // - If there are two NUM, lesser NUM goes to the left.
    // - If there is NUM and VAR, NUM goes to the left.
    // - If there is NUM and ABSTRACT (MUL, DIV, etc.), NUM goes to the left.
    // - If there is VAR and ABSTRACT (MUL, DIV, etc.), VAR goes to the left.
    // - SPECIAL CASE: If the data_type_node is EXP, then ig nore all the above.
    } else {
        match data_type_node {
            token::Token::EXP => {
                a = Node {
                    data_type: data_type_node,
                    left: Box::new(node_creation(split_locater(left_branch, left_group_locations))),
                    right: Box::new(node_creation(split_locater(right_branch, right_group_locations))),
                };
                return Some(a);
            }
            _ => {}
        }
        // Declare all needed variables for this operation
        // Raw branches, haven't been determined if they are left and right yet, and recursive, thus the branches won't be worked on till' their value is known
        let first_branch_raw: Option<Node> = node_creation(split_locater(left_branch, left_group_locations));
        let second_branch_raw: Option<Node> = node_creation(split_locater(right_branch, right_group_locations));
        // left and right processed, taken from first and second branch but determined placement.
        let left_branch_processed: Option<Node>;
        let right_branch_processed: Option<Node>;
        // number and a bool to say if it exists or not, as well as an Option float. The option is neccesary for the decleration of the number, even if it exists or not.
        let first_num: Option<f32>;
        let first_num_bool: bool;
        let second_num: Option<f32>;
        let second_num_bool: bool;
        // if a variable is the data_type in either first or second branch.
        let first_variable: bool;
        let second_variable: bool;

        // Match first data_type
        match first_branch_raw.clone().unwrap().data_type {
            token::Token::NUM(a) => {
                first_num = Some(a);
                first_num_bool = true;
                first_variable = false;
            }
            token::Token::VAR(_) => {
                first_variable = true;
                first_num_bool = false;
                first_num = None;
            }
            _ => {
                first_variable = false;
                first_num_bool = false;
                first_num = None;
            }
        }

        // match second data_type
        match second_branch_raw.clone().unwrap().data_type {
            token::Token::NUM(a) => {
                second_num = Some(a);
                second_num_bool = true;
                second_variable = false;
            }
            token::Token::VAR(_) => {
                second_variable = true;
                second_num_bool = false;
                second_num = None;
            }
            _ => {
                second_variable = false;
                second_num_bool = false;
                second_num = None;
            }
        }

        // if NUM is valid for both first and second data_type
        if first_num_bool == true && second_num_bool == true {
            // if first NUM is less than or equal to second num
            if first_num.unwrap() <= second_num.unwrap() {
                left_branch_processed = first_branch_raw;
                right_branch_processed = second_branch_raw;
            // else, second NUM is less than first num
            } else {
                left_branch_processed = second_branch_raw;
                right_branch_processed = first_branch_raw;
            }
        // if NUM exists in only first
        } else if first_num_bool == true {
            left_branch_processed = first_branch_raw;
            right_branch_processed = second_branch_raw;
        // if NUM exists in only second
        } else if second_num_bool == true {
            left_branch_processed = second_branch_raw;
            right_branch_processed = first_branch_raw;
        // if VAR in both
        } else if first_variable == true && second_variable == true {
            left_branch_processed = first_branch_raw;
            right_branch_processed = second_branch_raw;
        // if VAR in only first
        } else if first_variable == true {
            left_branch_processed = first_branch_raw;
            right_branch_processed = second_branch_raw;
        // if VAR in only second
        } else if second_variable == true {
            left_branch_processed = second_branch_raw;
            right_branch_processed = first_branch_raw;
        // ABSTRACT in both
        } else {
            left_branch_processed = first_branch_raw;
            right_branch_processed = second_branch_raw;
        }
        // Declare the node
        a = Node {
            data_type: data_type_node,
            left: Box::new(left_branch_processed),
            right: Box::new(right_branch_processed),
        };

    }
    // Return the node
    return Some(a)
}


pub fn process(token_vector: Vec<token::Token>) -> Node {
    let unprocessed = token::fix_groups(token_vector);
    let fixed_token_vector: Vec<token::Token> = unprocessed.0;
    let group_locations: Vec<(i32, i32)> = unprocessed.1;
    let binary_tree: Node = node_creation(split_locater(fixed_token_vector, group_locations)).unwrap();
    return binary_tree
}