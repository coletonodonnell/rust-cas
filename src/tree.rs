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
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn type_declare(left: Option<Box<Node>>, right: Option<Box<Node>>) -> (Option<f32>, Option<f32>, Option<String>, Option<String>, Option<i32>, Option<i32>) {
    let left_num: Option<f32>;
    let right_num: Option<f32>;
    let left_var: Option<String>;
    let right_var: Option<String>;
    let left_type: Option<i32>;
    let right_type: Option<i32>;
    match left {
        Some(c) => {
            match c.data_type {
                token::Token::NUM(b) => {
                    left_num = Some(b);
                    left_var = None;
                    left_type = None;
                }
                token::Token::VAR(b) => {
                    left_var = Some(b);
                    left_num = None;
                    left_type = None;
                }
                token::Token::ADD => {
                    left_num = None;
                    left_var = None;
                    left_type = Some(0);
                }
                token::Token::MUL => {
                    left_num = None;
                    left_var = None;
                    left_type = Some(1);
                }
                token::Token::DIV => {
                    left_num = None;
                    left_var = None;
                    left_type = Some(2);
                }
                token::Token::EXP => {
                    left_num = None;
                    left_var = None;
                    left_type = Some(3);
                }
                _ => {
                    left_num = None;
                    left_var = None;
                    left_type = None;
                }
            }
        }
        None => {
            left_num = None;
            left_var = None;
            left_type = None;
        }
    }
    match right {
        Some(c) => {
            match c.data_type {
                token::Token::NUM(b) => {
                    right_num = Some(b);
                    right_var = None;
                    right_type = None;
                }
                token::Token::VAR(b) => {
                    right_var = Some(b);
                    right_num = None;
                    right_type = None;
                }
                token::Token::ADD => {
                    right_num = None;
                    right_var = None;
                    right_type = Some(0);
                }
                token::Token::MUL => {
                    right_num = None;
                    right_var = None;
                    right_type = Some(1);
                }
                token::Token::DIV => {
                    right_num = None;
                    right_var = None;
                    right_type = Some(2);
                }
                token::Token::EXP => {
                    right_num = None;
                    right_var = None;
                    right_type = Some(3);
                }
                _ => {
                    right_num = None;
                    right_var = None;
                    right_type = None;
                }
            }
        }
        None => {
            right_num = None;
            right_var = None;
            right_type = None;
        }
    }
    return (left_num, right_num, left_var, right_var, left_type, right_type)
}

fn create_node_from_var(var: String) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        data_type: token::Token::VAR(var),
        left: None,
        right: None
    }))
}

fn create_node_from_num(num: f32) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        data_type: token::Token::NUM(num),
        left: None,
        right: None,
    }))
}

fn stock_node(data_type: token::Token, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        data_type: data_type,
        left: left,
        right: right
    }))
}

fn simplify_node(node: Option<Box<Node>>) -> Option<Box<Node>> {
    match node.clone() {
        Some(a) => {
           match a.data_type {
               token::Token::NUM(_) => {
                    return Some(a)
               }
               token::Token::VAR(_) => {
                    return Some(a)
               }
               _ => {
                    // Find left and right
                    let mut left: Option<Box<Node>> = simplify_node(a.left);
                    let mut right: Option<Box<Node>> = simplify_node(a.right);

                    // Switch left vs. right if left is var and right is num if and operator is addition or multiplication
                    match a.data_type {
                        token::Token::ADD | token::Token::MUL => {
                            match left.clone().unwrap().data_type {
                                // Switch variables to the right if they are left
                                token::Token::VAR(_) => {
                                    match right.clone().unwrap().data_type {
                                        token::Token::NUM(_) => {
                                            let temp_left: Option<Box<Node>> = right;
                                            right = left;
                                            left = temp_left;
                                        }
                                        _ => {}
                                    }
                                }
                                // Switch Operaters to right if they are left
                                token::Token::DIV | token::Token::ADD | token::Token::MUL | token::Token::EXP => {
                                    match right.clone().unwrap().data_type {
                                        token::Token::NUM(_) | token::Token::VAR(_) => {
                                            let temp_left: Option<Box<Node>> = right;
                                            right = left;
                                            left = temp_left;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    
                    // decompose it (i am too lazy for matching)
                    let decomposed: (Option<f32>, Option<f32>, Option<String>, Option<String>, Option<i32>, Option<i32>) = type_declare(left.clone(), right.clone());
                    let left_num: Option<f32> = decomposed.0;
                    let right_num: Option<f32> = decomposed.1;
                    let left_var: Option<String> = decomposed.2;
                    let right_var: Option<String> = decomposed.3;
                    let left_type: Option<i32> = decomposed.4;
                    let right_type: Option<i32> = decomposed.5;

                    // decompose left_left and left_right
                    let left_left_num: Option<f32>;
                    let left_right_num: Option<f32>;
                    let left_left_var: Option<String>;
                    let left_right_var: Option<String>;

                    // decompose right_left and right_right
                    let right_left_num: Option<f32>;
                    let right_right_num: Option<f32>;
                    let right_left_var: Option<String>;
                    let right_right_var: Option<String>;
                    let right_right_type: Option<i32>;

                    // decompose right_right_left and right_right_right
                    let right_right_left_num: Option<f32>;
                    let right_right_right_num: Option<f32>;
                    let right_right_left_var: Option<String>;
                    let right_right_right_var: Option<String>;

                    // Check to see if we must decompose left_left and left_right
                    if left_type != None {
                        let left_decomposed: (Option<f32>, Option<f32>, Option<String>, Option<String>, Option<i32>, Option<i32>) = type_declare(left.clone().unwrap().left, left.clone().unwrap().right);
                        left_left_num = left_decomposed.0;
                        left_right_num = left_decomposed.1;
                        left_left_var = left_decomposed.2;
                        left_right_var = left_decomposed.3;

                    // if not, set all to None
                    } else {
                        left_left_num = None;
                        left_right_num = None;
                        left_left_var = None;
                        left_right_var = None;
                    }

                    // check to see if we must decompose right_left and right_right
                    if right_type != None {
                        let right_decomposed: (Option<f32>, Option<f32>, Option<String>, Option<String>, Option<i32>, Option<i32>) = type_declare(right.clone().unwrap().left, right.clone().unwrap().right);
                        right_left_num = right_decomposed.0;
                        right_right_num = right_decomposed.1;
                        right_left_var = right_decomposed.2;
                        right_right_var = right_decomposed.3;
                        right_right_type = right_decomposed.5;

                        // check to see if we must decompose right_right_left and right_right_right
                        if right_right_type != None {
                            let right_right_decompose: (Option<f32>, Option<f32>, Option<String>, Option<String>, Option<i32>, Option<i32>) = type_declare(right.clone().unwrap().right.clone().unwrap().left, right.clone().unwrap().right.clone().unwrap().right);
                            right_right_left_num = right_right_decompose.0;
                            right_right_right_num = right_right_decompose.1;
                            right_right_left_var = right_right_decompose.2;
                            right_right_right_var = right_right_decompose.3;

                        // if not, set all to none
                        } else {
                            right_right_left_num = None;
                            right_right_right_num = None;
                            right_right_left_var = None;
                            right_right_right_var = None;
                        }

                    // if not, set all to None
                    } else {
                        right_left_num = None;
                        right_right_num = None;
                        right_left_var = None;
                        right_right_var = None;
                        right_right_type = None;
                        right_right_left_num = None;
                        right_right_right_num = None;
                        right_right_left_var = None;
                        right_right_right_var = None;
                    }

                    // *
                    //          (OP)
                    //  (NUM)           (NUM)
                    if left_num != None && right_num != None {
                        let d: Option<Box<Node>>;
                        match a.data_type {

                            // Add the two NUMs
                            token::Token::ADD => {
                                d = create_node_from_num(left_num.unwrap() + right_num.unwrap());
                            }

                            // Multiply the two NUMs
                            token::Token::MUL => {
                                d = create_node_from_num(left_num.unwrap() * right_num.unwrap());
                            }

                            // Divide the two NUMs
                            token::Token::DIV => {
                                if right_num.unwrap() == 0.0 {
                                    panic!("Can't divide by 0!");
                                }
                                d = create_node_from_num(left_num.unwrap() / right_num.unwrap());
                            }

                            // Raise the NUM to the power of the other NUM (left to right)
                            token::Token::EXP => {
                                d = create_node_from_num(left_num.unwrap().powf(right_num.unwrap()));
                            }

                            // Else return None (shouldn't ever trigger)
                            _ => {
                                return None
                            }
                        }
                        return d

                    // *
                    //          (OP)
                    //  (VAR)           (VAR)
                    } else if left_var != None && right_var != None {
                        let d: Option<Box<Node>>;
                        match a.data_type {

                            // Add the two VARs (2.0 * VAR)
                            token::Token::ADD => {
                                d = Some(Box::new(Node {
                                    data_type: token::Token::MUL,
                                    left: create_node_from_num(2.00),
                                    right: create_node_from_var(left_var.unwrap()),
                                }));
                            }

                            // Multply the two VARs (VAR ^ 2.0)
                            token::Token::MUL => {
                                d = Some(Box::new(Node {
                                    data_type: token::Token::EXP,
                                    left: create_node_from_var(left_var.unwrap()),
                                    right: create_node_from_num(2.00),
                                }));
                            }

                            // Divide the two VARs (1.00)
                            token::Token::DIV => {
                                d = create_node_from_num(1.00);
                            }

                            // Exponent already simplified
                            token::Token::EXP => {
                                return stock_node(a.data_type, left, right)
                            }

                            // Else return None (shouldn't ever trigger)
                            _ => {
                                return None
                            }
                        }
                        return d
                        
                    // * (left vs. right sensitive expression, has to be either DIV or EXP)
                    //        (OP)
                    //  (VAR)      (NUM)
                    } else if left_var != None && right_num != None {
                        match a.data_type {
                            // If DIV, set the node to MUL, and Multiply between 1 over right NUM and VAR
                            //        (EXP or DIV) 
                            //  (VAR)             (NUM)
                            //
                            token::Token::DIV => {
                                return Some(Box::new(Node {
                                    data_type: token::Token::MUL,
                                    left: create_node_from_num(1.0 / right_num.unwrap()),
                                    right: create_node_from_var(left_var.unwrap()),
                                }))
                            }
                            token::Token::EXP => {
                                // If right_num is greater than 1.0, then just return the node
                                if right_num.unwrap() > 1.0 {
                                    return stock_node(a.data_type, left, right)
                                // If it is equal to 1.0, just return the VAR
                                } else if right_num.unwrap() == 0.0  {
                                    return create_node_from_num(1.0);
                                } else if right_num.unwrap() == 1.0 {
                                    return create_node_from_var(left_var.unwrap())
                                // If it is negative
                                } else if right_num.unwrap().is_sign_negative() {
                                    // If equal to -1.0
                                    if right_num.unwrap() == -1.0 {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::DIV,
                                            left: create_node_from_num(1.0),
                                            right: create_node_from_var(left_var.unwrap()),
                                        }))
                                    // If not equal to -1.0
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::DIV,
                                            left: create_node_from_num(1.0),
                                            right: Some(Box::new(Node {
                                                data_type: token::Token::EXP,
                                                left: create_node_from_var(left_var.unwrap()),
                                                right: create_node_from_num(right_num.unwrap().abs()),
                                            }))
                                        }))
                                    }
                                // If all of the above isn't the case, just set d equal to the node
                                } else {
                                    return stock_node(a.data_type, left, right)
                                }
                            }
                            // Else return stock_node(a.data_type, left, right), this should only ever trigger if there is an EXP or DIV, as otherwise NUM would be left and VAR would be right :)
                            _ => {
                                return stock_node(a.data_type, left, right)
                            }
                        }

                    // *
                    //          (OP)
                    //  (NUM)           (VAR)
                    } else if left_num != None && right_var != None {
                        if left_num.unwrap() == 0.00 {
                            return create_node_from_var(right_var.unwrap())
                        } else {
                            return stock_node(a.data_type, left, right)
                        }

                    // *
                    //        (OP)
                    //  (VAR)       (OP)
                    } else if left_var != None && right_type != None {
                        // +
                        //         (MUL or DIV)
                        // (VAR)          (MUL or DIV)
                        //           (NUM)            (EXP)
                        //                       (VAR)     (NUM)
                        if right_right_type != None && right_right_type.unwrap() == 3 && right_right_left_var != None && right_right_right_num != None {
                            // if we are multiplying
                            if right_type.unwrap() == 1 {
                                match a.data_type {
                                    // If it is equal to MUL
                                    token::Token::MUL => {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::MUL,
                                            left: create_node_from_num(right_left_num.unwrap()),
                                            right: Some(Box::new(Node {
                                                data_type: token::Token::EXP,
                                                left: create_node_from_var(left_var.unwrap()),
                                                right: create_node_from_num(right_right_right_num.unwrap() + 1.0),
                                            })),
                                        }))
                                    }

                                    // If it is equal to DIV
                                    token::Token::DIV => {
                                        // Find new exponent
                                        let exponent: f32 = right_right_right_num.unwrap() - 1.0;
                                        // if that exponent is equal to 1, continue

                                        if exponent == 0.0 {
                                            return create_node_from_num(1.00 / right_left_num.unwrap())
                                        } else if exponent == 1.00 {
                                            return Some(Box::new(Node {
                                                data_type: token::Token::DIV,
                                                left: create_node_from_num(1.00),
                                                right: Some(Box::new(Node {
                                                    data_type: token::Token::MUL,
                                                    left: create_node_from_num(right_left_num.unwrap()),
                                                    right: create_node_from_var(left_var.unwrap()),
                                                }))
                                            }))

                                        // If not, do the following
                                        } else {
                                            return Some(Box::new(Node {
                                                data_type: token::Token::DIV,
                                                left: create_node_from_num(1.00),
                                                right: Some(Box::new(Node {
                                                    data_type: token::Token::MUL,
                                                    left: create_node_from_num(right_left_num.unwrap()),
                                                    right: Some(Box::new(Node {
                                                        data_type: token::Token::EXP,
                                                        left: create_node_from_var(left_var.unwrap()),
                                                        right: create_node_from_num(exponent),
                                                    }))
                                                })),
                                            }))
                                        }
                                    }

                                    // Else just return the node
                                    _ => {return stock_node(a.data_type, left, right)}
                                }
                            // if we are dividing
                            } else if right_type.unwrap() == 2 {
                                match a.data_type {
                                    token::Token::MUL => {
                                        let exponent: f32;
                                        if right_right_right_num.unwrap() == 1.00 {
                                            return create_node_from_num(right_left_num.unwrap())
                                        } else {
                                            exponent = right_right_right_num.unwrap() - 1.00;
                                        }
                                        if exponent != 1.00 {
                                            return Some(Box::new(Node {
                                                    data_type: token::Token::DIV,
                                                    left: create_node_from_num(right_left_num.unwrap()),
                                                    right: Some(Box::new(Node{
                                                        data_type: token::Token::EXP,
                                                        left: create_node_from_var(left_var.unwrap()),
                                                        right: create_node_from_num(exponent),
                                                    })),
                                                }))
                                        } else {
                                            return Some(Box::new(Node {
                                                data_type: token::Token::DIV,
                                                left: create_node_from_num(right_left_num.unwrap()),
                                                right: create_node_from_var(left_var.unwrap()),
                                            }))
                                        }
                                    }
                                    token::Token::DIV => {
                                        if right_right_right_num.unwrap() == 0.00 {
                                            return Some(Box::new(Node {
                                                data_type: token::Token::DIV,
                                                left: create_node_from_var(left_var.unwrap()),
                                                right: create_node_from_num(right_left_num.unwrap()),
                                            }))
                                        } else {
                                            return Some(Box::new(Node {
                                                data_type: token::Token::DIV,
                                                left: Some(Box::new(Node {
                                                    data_type: token::Token::EXP,
                                                    left: create_node_from_var(left_var.unwrap()),
                                                    right: create_node_from_num(right_right_right_num.unwrap() + 1.00),
                                                })),
                                                right: create_node_from_num(right_left_num.unwrap()),
                                            }))
                                        }
                                    }
                                    _ => {return stock_node(a.data_type, left, right)}
                                }
                            } else {
                                return stock_node(a.data_type, left, right)
                            }

                        // +
                        //            (OP)
                        //  (VAR)            (^)
                        //             (VAR)     (NUM)
                        } else if right_type.unwrap() == 3 && right_left_var != None && right_right_num != None {
                            match a.data_type {

                                // +
                                //            (MUL)
                                //  (VAR)            (^)
                                //             (VAR)     (NUM)
                                token::Token::MUL => {
                                    return Some(Box::new(Node {
                                        data_type: token::Token::EXP,
                                        left: create_node_from_var(left_var.unwrap()),
                                        right: create_node_from_num(right_right_num.unwrap() + 1.00),
                                    }))
                                }

                                // +
                                //            (DIV)
                                //  (VAR)            (^)
                                //             (VAR)     (NUM)
                                token::Token::DIV => {
                                    let exponent: f32 = right_right_num.unwrap() - 1.00;
                                    if exponent == 1.00 {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::DIV,
                                            left: create_node_from_num(1.00),
                                            right: create_node_from_var(left_var.unwrap()),
                                        }))
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::DIV,
                                            left: create_node_from_num(1.00),
                                            right: Some(Box::new(Node {
                                                data_type: token::Token::EXP,
                                                left: create_node_from_var(left_var.unwrap()),
                                                right: create_node_from_num(exponent),
                                            }))
                                        }))
                                    }
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }
                        
                        // +
                        //          (ADD)
                        //  (VAR)           (MUL)
                        //          (-1)            (VAR)
                        } else if right_type.unwrap() == 1 && right_right_var != None && right_left_num != None && right_left_num.unwrap() == -1.0 { 
                            return create_node_from_num(0.0);
                        } else {
                            return stock_node(a.data_type, left, right)
                        }

                    // * 
                    //       (OP)
                    //  (OP)     (VAR)
                    } else if left_type != None && right_var != None {

                        // +
                        //               (DIV)
                        //      (EXP)           (VAR)
                        // (VAR)     (NUM)
                        if left_type.unwrap() == 3 && left_left_var != None && left_right_num != None {
                            match a.data_type {
                                token::Token::DIV => {
                                    if left_right_num.unwrap() == 2.00 {
                                        return create_node_from_var(left_left_var.unwrap())
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::EXP,
                                            left: create_node_from_var(left_left_var.unwrap()),
                                            right: create_node_from_num(left_right_num.unwrap() - 1.00),
                                        }))
                                    }
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }
                        
                        // +
                        //               (DIV)
                        //       (MUL)           (VAR)
                        //  (NUM)     (VAR)
                        } else if left_type.unwrap() == 1 && left_left_num != None && left_right_var != None {
                            match a.data_type {
                                token::Token::DIV => {
                                    return create_node_from_num(left_left_num.unwrap())
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }

                        } else {
                            return stock_node(a.data_type, left, right)
                        }

                    // *
                    //      (OP)
                    // (NUM)    (OP)
                    } else if left_num != None && right_type != None {
                        // +
                        //        (OP)
                        //  (NUM)       (MUL)
                        //        (NUM)      (VAR)
                        if right_type.unwrap() == 1 && right_left_num != None && right_right_var != None {
                            match a.data_type {
                                token::Token::MUL => {
                                    return Some(Box::new(Node {
                                        data_type: token::Token::MUL,
                                        left: create_node_from_num(left_num.unwrap() * right_left_num.unwrap()),
                                        right: create_node_from_var(right_right_var.unwrap()),
                                    }))
                                }
                                token::Token::DIV => {
                                    if left_num.unwrap() == right_left_num.unwrap() {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::DIV,
                                            left: create_node_from_num(1.00),
                                            right: create_node_from_var(right_right_var.unwrap()),
                                        }))
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::DIV,
                                            left: create_node_from_num(left_num.unwrap() * (1.00 / right_left_num.unwrap())),
                                            right: create_node_from_var(right_right_var.unwrap()),
                                        }))
                                    }
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }

                        // +
                        //         (OP)
                        //  (NUM)        (DIV)
                        //         (NUM)       (VAR)
                        } else if right_type.unwrap() == 2 && right_left_num != None && right_right_var != None {
                            match a.data_type {
                                token::Token::DIV => {
                                    if left_num.unwrap() == right_left_num.unwrap() {
                                        return create_node_from_var(right_right_var.unwrap())
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::DIV,
                                            left: Some(Box::new(Node {
                                                data_type: token::Token::MUL,
                                                left: create_node_from_num(left_num.unwrap()),
                                                right: create_node_from_var(right_right_var.unwrap()),
                                            })),
                                            right: create_node_from_num(right_left_num.unwrap()), 
                                        }))
                                    }
                                }
                                token::Token::MUL => {
                                    return Some(Box::new(Node {
                                        data_type: token::Token::DIV,
                                        left: create_node_from_num(left_num.unwrap() * right_left_num.unwrap()),
                                        right: create_node_from_var(right_right_var.unwrap()),
                                    }))
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }

                        // +
                        //          (OP)
                        //  (NUM)           (DIV)
                        //          (VAR)           (NUM)
                        } else if right_type.unwrap() == 2 && right_left_var != None && right_right_num != None {
                            match a.data_type {
                                token::Token::MUL => {
                                    if right_left_num.unwrap() == right_num.unwrap() {
                                        return create_node_from_var(left_left_var.unwrap())
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::MUL,
                                            left: create_node_from_num(right_num.unwrap() / left_right_num.unwrap()),
                                            right: create_node_from_var(left_left_var.unwrap()),
                                        }))
                                    }
                                }
                                token::Token::DIV => {
                                    if left_num.unwrap() == right_right_num.unwrap() {
                                        return create_node_from_var(right_left_var.unwrap())
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::MUL,
                                            left: create_node_from_num(left_num.unwrap() / right_right_num.unwrap()),
                                            right: create_node_from_var(right_left_var.unwrap()),
                                        }))
                                    }
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }
                        
                        // +
                        //       (OP)
                        // (NUM)      (EXP)
                        //       (VAR)      (NUM)
                        } else if right_type.unwrap() == 3 && right_left_var != None && right_right_num != None {
                            match a.data_type {
                                token::Token::ADD => {
                                    if left_num.unwrap() == 0.00 {
                                        return right
                                    } else {
                                        return stock_node(a.data_type, left, right)
                                    }
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }

                        // +
                        //          (OP)
                        //  (NUM)         (DIV)
                        //          (NUM)       (MUL)
                        //                (NUM)       (VAR)
                        } else if right_type != None && right_type.unwrap() == 2 && right_left_num != None && right_right_type != None && right_right_type.unwrap() == 1 && right_right_left_num != None && right_right_right_var != None {
                            match a.data_type {
                                token::Token::MUL => {
                                    return Some(Box::new(Node {
                                        data_type: token::Token::DIV,
                                        left: create_node_from_num(left_num.unwrap() * right_left_num.unwrap()),
                                        right: Some(Box::new(Node {
                                            data_type: token::Token::MUL,
                                            left: create_node_from_num(right_right_left_num.unwrap()),
                                            right: create_node_from_var(right_right_right_var.unwrap()),
                                        })),
                                    }))
                                }
                                token::Token::DIV => {
                                    return Some(Box::new(Node {
                                        data_type: token::Token::MUL,
                                        left: create_node_from_num((left_num.unwrap() * right_right_left_num.unwrap()) / right_left_num.unwrap()),
                                        right: create_node_from_var(right_right_right_var.unwrap()),
                                    }))
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }
                        
                        } else {
                            return stock_node(a.data_type, left, right)
                        }

                    // *
                    //       (OP)
                    //  (OP)      (NUM)
                    } else if left_type != None && right_num != None {
                        // +
                        //                (OP)
                        //        (MUL)           (NUM)
                        //  (NUM)       (VAR)
                        if left_type.unwrap() == 1 && left_left_num != None && left_right_var != None {
                            match a.data_type {
                                token::Token::DIV => {
                                    if left_left_num.unwrap() == right_num.unwrap() {
                                        return create_node_from_var(left_right_var.unwrap())
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::MUL,
                                            left: create_node_from_num(left_left_num.unwrap() / right_num.unwrap()),
                                            right: create_node_from_var(left_right_var.unwrap()),
                                        }))
                                    }
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }
                        // +
                        //                (OP)
                        //        (DIV)           (NUM)
                        //  (NUM)       (VAR)
                        } else if left_type.unwrap() == 2 && left_left_num != None && left_right_var != None {
                            match a.data_type {
                                token::Token::DIV => {
                                    if left_left_num.unwrap() == right_num.unwrap() {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::DIV,
                                            left: create_node_from_num(1.00),
                                            right: create_node_from_var(left_right_var.unwrap()),
                                        }))
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::MUL,
                                            left: create_node_from_num(left_left_num.unwrap() / right_num.unwrap()),
                                            right: create_node_from_var(left_right_var.unwrap()),
                                        }))
                                    }
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }
                        // +
                        //                (OP)
                        //        (DIV)           (NUM)
                        //  (VAR)       (NUM)
                        } else if left_type.unwrap() == 2 && left_left_num != None && left_right_num != None {
                            match a.data_type {
                                token::Token::DIV => {
                                    return Some(Box::new(Node {
                                        data_type: token::Token::DIV,
                                        left: create_node_from_var(left_left_var.unwrap()),
                                        right: create_node_from_num(left_right_num.unwrap() * right_num.unwrap()),
                                    }))
                                }
                                
                                _ => {return stock_node(a.data_type, left, right)}
                            }
                        } else {
                            return stock_node(a.data_type, left, right)
                        }
                    // *
                    //      (OP)
                    //  (OP)    (OP)
                    } else if left_type != None && right_type != None {
                        //                (OP)
                        //      (EXP)              (EXP)
                        // (VAR)     (NUM)   (VAR)       (NUM)
                        if left_type.unwrap() == 3 && right_type.unwrap() == 3 && left_left_var != None && left_right_num != None && right_left_var != None && right_right_num != None {
                            match a.data_type {
                                token::Token::DIV => {
                                    if left_right_num.unwrap() == right_right_num.unwrap() {
                                        return create_node_from_num(1.00)
                                    } else if left_right_num.unwrap() < right_right_num.unwrap() {
                                        if left_right_num.unwrap() == 2.00 && right_right_num.unwrap() == 3.00 {
                                            return Some(Box::new(Node {
                                                data_type: token::Token::DIV,
                                                left: create_node_from_num(1.00),
                                                right: create_node_from_var(left_left_var.unwrap()),
                                            }))
                                        } else {
                                            return Some(Box::new(Node {
                                                data_type: token::Token::DIV,
                                                left: create_node_from_num(1.00),
                                                right: Some(Box::new(Node {
                                                    data_type: token::Token::EXP,
                                                    left: create_node_from_var(left_left_var.unwrap()),
                                                    right: create_node_from_num(right_right_num.unwrap() - left_right_num.unwrap()),
                                                }))
                                            }))
                                        }
                                    } else {
                                        return Some(Box::new(Node {
                                            data_type: token::Token::EXP,
                                            left: create_node_from_var(left_left_var.unwrap()),
                                            right: create_node_from_num(left_right_num.unwrap() - right_right_num.unwrap()),
                                        }))
                                    }
                                }
                                token::Token::MUL => {
                                    return Some(Box::new(Node {
                                        data_type: token::Token::EXP,
                                        left: create_node_from_var(left_left_var.unwrap()),
                                        right: create_node_from_num(left_right_num.unwrap() + right_right_num.unwrap()),
                                    }))
                                }
                                _ => {return stock_node(a.data_type, left, right)}
                            }
                        } else {
                            return stock_node(a.data_type, left, right)
                        }


                    } else {
                        return stock_node(a.data_type, left, right)
                    }
               } 
            }
        }
        None => {return node}
    }
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
fn node_creation(raw_node: (Vec<token::Token>, Vec<(i32, i32)>, Vec<token::Token>, Vec<(i32, i32)>, token::Token)) -> Option<Box<Node>> {
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
            left: None,
            right: None,
        };

    // If this isn't the case, we need to make sure we find what goes where, left vs. right. The weighting for this operation is as follows:
    // - If there are two NUM, lesser NUM goes to the left.
    // - If there is NUM and VAR, NUM goes to the left.
    // - If there is NUM and ABSTRACT (MUL, DIV, etc.), NUM goes to the left.
    // - If there is VAR and ABSTRACT (MUL, DIV, etc.), VAR goes to the left.
    // - SPECIAL CASE: If the data_type_node is EXP, then ignore all the above.
    // - SPECIAL CASE: If the data_type_node is DIV, then ignore all the above.
    } else {
        match data_type_node {
            token::Token::EXP => {
                a = Node {
                    data_type: data_type_node,
                    left: node_creation(split_locater(left_branch, left_group_locations)),
                    right: node_creation(split_locater(right_branch, right_group_locations)),
                };
                return Some(Box::new(a));
            }
            token::Token::DIV => {
                a = Node {
                    data_type: data_type_node,
                    left: node_creation(split_locater(left_branch, left_group_locations)),
                    right: node_creation(split_locater(right_branch, right_group_locations)),
                };
                return Some(Box::new(a));
            }
            _ => {}
        }

        // Declare all needed variables for this operation
        // Raw branches, haven't been determined if they are left and right yet, and recursive, thus the branches won't be worked on till' their value is known
        let first_branch_raw: Option<Box<Node>> = node_creation(split_locater(left_branch, left_group_locations));
        let second_branch_raw: Option<Box<Node>> = node_creation(split_locater(right_branch, right_group_locations));

        // left and right processed, taken from first and second branch but determined placement.
        let left_branch_processed: Option<Box<Node>>;
        let right_branch_processed: Option<Box<Node>>;

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
            left: left_branch_processed,
            right: right_branch_processed,
        };

    }
    // Return the node
    return Some(Box::new(a))
}

pub fn process(token_vector: Vec<token::Token>) -> Box<Node> {
    let unprocessed = token::fix_groups(token_vector);
    let fixed_token_vector: Vec<token::Token> = unprocessed.0;
    let group_locations: Vec<(i32, i32)> = unprocessed.1;
    let binary_tree: Box<Node> = node_creation(split_locater(fixed_token_vector, group_locations)).unwrap();
    println!("Before Simplifcation: {:#?}", binary_tree.clone());
    let simplified = simplify_node(Some(binary_tree));
    return simplified.unwrap();
}