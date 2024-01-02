use symboscript_types::parser::*;

mod expressions;

pub fn optimize(ast: &Ast) -> Ast {
    Ast {
        program: optim_program(&ast.program),
    }
}

fn optim_program(program: &Program) -> Program {
    Program {
        node: program.node,
        body: optim_body(&program.body),
    }
}

fn optim_body(body: &BlockStatement) -> BlockStatement {
    let mut new_body = vec![];

    for statement in body {
        match statement {
            Statement::ExpressionStatement(expression) => {
                new_body.push(Statement::ExpressionStatement(
                    expressions::optim_expression(&expression),
                ));
            }

            _ => {
                new_body.push(statement.clone());
            }
        }
    }

    new_body
}
