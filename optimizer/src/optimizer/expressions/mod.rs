use symboscript_types::{lexer::*, parser::*};

pub fn optim_expression(expression_stmt: &Expression) -> Expression {
    match expression_stmt {
        Expression::BinaryExpression(binary_expression) => {
            optim_binary_expression(&*binary_expression)
        }
        _ => expression_stmt.clone(),
    }
}

pub fn optim_binary_expression(binary_expression: &BinaryExpression) -> Expression {
    let left = optim_expression(&binary_expression.left);
    let right = optim_expression(&binary_expression.right);

    match (left.clone(), right.clone()) {
        (Expression::Literal(left), Expression::Literal(right)) => {
            match binary_expression.operator {
                TokenKind::Plus => return Expression::Literal(left + right),
                TokenKind::Minus => return Expression::Literal(left - right),
                TokenKind::Multiply => return Expression::Literal(left * right),
                TokenKind::Divide => return Expression::Literal(left / right),
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
