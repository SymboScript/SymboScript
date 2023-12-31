use symboscript_types::parser::*;

pub fn optim_expression(expression_stmt: &Expression) -> Expression {
    match expression_stmt {
        Expression::BinaryExpression(binary_expression) => {
            optim_binary_expression(&sort_plus_binary_expression(binary_expression))
        }
        _ => expression_stmt.clone(),
    }
}

pub fn optim_expression_sub(expression_stmt: &Expression) -> Expression {
    match expression_stmt {
        Expression::BinaryExpression(binary_expression) => {
            optim_binary_expression(&sort_plus_binary_expression(binary_expression))
        }
        _ => expression_stmt.clone(),
    }
}

pub fn optim_binary_expression(binary_expression: &BinaryExpression) -> Expression {
    let left = optim_expression_sub(&binary_expression.left);
    let right = optim_expression_sub(&binary_expression.right);

    match (left.clone(), right.clone()) {
        (Expression::Literal(left), Expression::Literal(right)) => {
            match binary_expression.operator {
                BinaryOperator::Add => {
                    return Expression::Literal(Literal {
                        node: binary_expression.node.clone(),
                        value: left.value + right.value,
                    })
                }
                BinaryOperator::Substract => {
                    return Expression::Literal(Literal {
                        node: binary_expression.node.clone(),
                        value: left.value - right.value,
                    })
                }
                BinaryOperator::Multiply => {
                    return Expression::Literal(Literal {
                        node: binary_expression.node.clone(),
                        value: left.value * right.value,
                    })
                }
                BinaryOperator::Divide => {
                    return Expression::Literal(Literal {
                        node: binary_expression.node.clone(),
                        value: left.value / right.value,
                    })
                }
                _ => {}
            }
        }
        _ => {}
    }

    Expression::BinaryExpression(Box::new(BinaryExpression {
        left,
        right,
        operator: binary_expression.operator.clone(),
        node: binary_expression.node.clone(),
    }))
}

fn sort_plus_binary_expression(binary_expression: &BinaryExpression) -> BinaryExpression {
    let mut flat = flatten_plus_binary_expression(binary_expression);

    flat.sort_by(|a, b| {
        let left = match a.clone() {
            Expression::Literal(_) => -1,
            _ => 1,
        };

        let right = match b.clone() {
            Expression::Literal(_) => 1,
            Expression::BinaryExpression(_) => 2,
            _ => 0,
        };

        left.cmp(&right)
    });

    unflat_plus_binary_expression(&flat)
}

fn unflat_plus_binary_expression(flat: &Vec<Expression>) -> BinaryExpression {
    let mut flat = flat.clone();

    flat.reverse();

    let mut node = BinaryExpression {
        right: flat.pop().unwrap(),
        left: flat.pop().unwrap(),
        operator: BinaryOperator::Add,
        node: Node::new(0, 0),
    };

    flat.reverse();

    for expression in flat {
        node = BinaryExpression {
            right: expression.clone(),
            left: Expression::BinaryExpression(Box::new(node)),
            operator: BinaryOperator::Add,
            node: Node::new(0, 0),
        }
    }

    node
}

fn flatten_plus_binary_expression(binary_expression: &BinaryExpression) -> Vec<Expression> {
    let op = binary_expression.operator;

    let left = binary_expression.left.clone();
    let right = binary_expression.right.clone();

    // println!("A: {} {} {}", left, op, right);

    if op == BinaryOperator::Add {
        match (left.clone(), right.clone()) {
            (Expression::BinaryExpression(left), Expression::BinaryExpression(right)) => {
                let mut vec = vec![];

                vec.append(&mut flatten_plus_binary_expression(&left));
                vec.append(&mut flatten_plus_binary_expression(&right));

                return vec;
            }

            (_, Expression::BinaryExpression(right)) => {
                let mut vec = vec![];

                vec.push(left);
                vec.append(&mut flatten_plus_binary_expression(&right));

                return vec;
            }

            (Expression::BinaryExpression(left), _) => {
                let mut vec = vec![];

                vec.append(&mut flatten_plus_binary_expression(&left));
                vec.push(right.clone());

                return vec;
            }

            _ => {
                return vec![left, right];
            }
        }
    }

    vec![Expression::BinaryExpression(Box::new(
        binary_expression.clone(),
    ))]
}
