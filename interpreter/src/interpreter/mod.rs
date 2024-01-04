use std::collections::HashMap;

use symboscript_types::{interpreter::*, lexer::*, parser::*};
use symboscript_utils::report_error;

mod native;

pub struct Interpreter<'a> {
    /// Path of the source file
    path: &'a str,

    /// Source Text
    source: &'a str,

    ast: &'a Ast,

    scope_stack: Vec<String>,

    current_scope: String,

    vault: Vault,
}

impl<'a> Interpreter<'a> {
    pub fn new(path: &'a str, source: &'a str, ast: &'a Ast) -> Self {
        let vault = Vault::new();

        Self {
            path,
            source,
            ast,
            scope_stack: vec![],
            current_scope: String::new(),
            vault,
        }
    }

    pub fn run(&mut self) {
        self.initialize();

        self.eval_ast(self.ast.clone());
    }

    fn eval_ast(&mut self, ast: Ast) {
        self.eval_program_body(&ast.program.body);
    }

    fn eval_program_body(&mut self, body: &BlockStatement) {
        for statement in body {
            self.eval_statement(&statement);
        }
    }

    fn eval_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::ExpressionStatement(expr) => {
                self.eval_expression(&expr);
            }
            Statement::ReturnStatement(_) => todo!(),
            Statement::ThrowStatement(_) => todo!(),
            Statement::ContinueStatement(_) => todo!(),
            Statement::BreakStatement(_) => todo!(),
            Statement::YieldStatement(_) => todo!(),
            Statement::VariableDeclaration(decl) => {
                let value = if decl.is_formula {
                    Value::Ast(decl.init.clone())
                } else {
                    self.eval_expression(&decl.init)
                };

                self.set_variable_force(&decl.id, value);
            }
            Statement::FunctionDeclaration(_) => todo!(),
            Statement::ScopeDeclaration(decl) => {
                let scope = self.start_declaration_of_named_scope(&decl.id);
                self.eval_program_body(&decl.body);
                self.end_declaration_of_named_scope(&scope);
            }
            Statement::IfStatement(_) => todo!(),
            Statement::ForStatement(_) => todo!(),
            Statement::WhileStatement(_) => todo!(),
            Statement::LoopStatement(_) => todo!(),
            Statement::TryStatement(_) => todo!(),
            Statement::BlockStatement(body) => {
                self.increment_scope();
                self.eval_program_body(body);
                self.decrement_scope();
            }
        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::BinaryExpression(binary_expr) => self.eval_binary_expression(binary_expr),
            Expression::UnaryExpression(unary_expr) => self.eval_unary_expression(unary_expr),
            Expression::ConditionalExpression(_) => todo!(),
            Expression::CallExpression(call_expr) => self.eval_call_expression(call_expr, false),
            Expression::MemberExpression(member_expr) => self.eval_member_expression(member_expr),
            Expression::SequenceExpression(_) => todo!(),
            Expression::WordExpression(_) => todo!(),

            Expression::Literal(val) => self.match_literal(val),

            Expression::Identifier(id) => self.get_variable_value(id),

            Expression::None(_) => Value::None,
        }
    }

    fn eval_member_expression(&mut self, member_expr: &MemberExpression) -> Value {
        let object = self.eval_expression(&member_expr.object);

        let object: Identifier = match object {
            Value::ScopeRef(ref_name) => Identifier {
                name: ref_name.clone(),
                node: member_expr.node.clone(),
            },
            Value::Sequence(_) => todo!(),
            _ => {
                self.report(
                    &format!("is not a scope"),
                    member_expr.node.start,
                    member_expr.node.end,
                );
                unreachable!("Report ends proccess");
            }
        };

        self.enter_named_scope(&object.name);

        let left = match &member_expr.property {
            Expression::Identifier(id) => {
                if member_expr.is_expr {
                    self.eval_expression(&member_expr.property)
                } else {
                    self.get_variable_value(id)
                }
            }
            Expression::CallExpression(call_expr) => self.eval_call_expression(call_expr, true),
            _ => self.eval_expression(&member_expr.property),
        };

        self.exit_named_scope();

        left
    }

    fn eval_call_expression(&mut self, call_expr: &CallExpression, member: bool) -> Value {
        let var = self.get_variable_value(&Identifier {
            name: call_expr.callee.clone(),
            node: call_expr.node.clone(),
        });

        let args = match &call_expr.arguments {
            Expression::SequenceExpression(seq_exp) => seq_exp,
            _ => unreachable!("Arguments can only be sequence expressions"),
        };

        let exited;

        if member {
            exited = self.exit_named_scope();
        } else {
            exited = String::new();
        }

        let args = args
            .expressions
            .iter()
            .map(|expr| self.eval_expression(expr))
            .collect::<Vec<Value>>();

        if member {
            self.enter_named_scope(&exited);
        }

        match var {
            Value::NativeFunction(name) => native::run_function(self, &name, args),
            Value::Function(_) => todo!(),

            _ => {
                self.report(
                    &format!("`{}` is not a function", call_expr.callee),
                    call_expr.node.start,
                    call_expr.node.end,
                );
                unreachable!("Report ends proccess")
            }
        }
    }

    fn eval_unary_expression(&mut self, expression: &UnaryExpression) -> Value {
        let right = self.eval_expression(&expression.right);

        match expression.operator {
            UnaryOperator::Plus => right,
            UnaryOperator::Minus => -right,
            UnaryOperator::Not => !right,
            UnaryOperator::BitNot => todo!(),
            UnaryOperator::PlusPlus => right + Value::Number(1.0),
            UnaryOperator::MinusMinus => right - Value::Number(1.0),
        }
    }

    fn eval_binary_expression(&mut self, expression: &BinaryExpression) -> Value {
        let left = self.eval_expression(&expression.left);
        let right = self.eval_expression(&expression.right);

        match expression.operator {
            BinaryOperator::Add => left + right,
            BinaryOperator::Substract => left - right,
            BinaryOperator::Multiply => left * right,
            BinaryOperator::Divide => left / right,
            BinaryOperator::Power => self.pow(left, right),
            BinaryOperator::Range => self.range(left, right),

            BinaryOperator::Modulo => left % right,

            BinaryOperator::And => todo!(),
            BinaryOperator::Or => todo!(),
            BinaryOperator::Xor => todo!(),

            BinaryOperator::BitAnd => todo!(),
            BinaryOperator::BitOr => todo!(),
            BinaryOperator::BitXor => todo!(),

            BinaryOperator::BitLeftShift => left << right,
            BinaryOperator::BitRightShift => left >> right,

            BinaryOperator::Assign => todo!(),
            BinaryOperator::PlusAssign => todo!(),
            BinaryOperator::MinusAssign => todo!(),
            BinaryOperator::MultiplyAssign => todo!(),
            BinaryOperator::DivideAssign => todo!(),
            BinaryOperator::PowerAssign => todo!(),
            BinaryOperator::ModuloAssign => todo!(),

            BinaryOperator::Equal => todo!(),
            BinaryOperator::NotEqual => todo!(),
            BinaryOperator::Less => todo!(),
            BinaryOperator::LessEqual => todo!(),
            BinaryOperator::Greater => todo!(),
            BinaryOperator::GreaterEqual => todo!(),
        }
    }

    fn match_literal(&mut self, literal: &Literal) -> Value {
        match &literal.value {
            TokenValue::None => Value::None,
            TokenValue::Number(val) => Value::Number(*val),
            TokenValue::Str(val) => Value::Str(val.clone()),
            TokenValue::Bool(val) => Value::Bool(*val),
            TokenValue::Identifier(id) => self.get_variable_value(&Identifier {
                node: Node::new(literal.node.start, literal.node.end),
                name: id.clone(),
            }),
        }
    }

    /// Gets the value of a variable from the current scope to the global scope if it doesn't exist in the current scope
    fn get_variable_value(&mut self, identifier: &Identifier) -> Value {
        let id = identifier.name.clone();

        for scope in self.scope_stack.iter().rev() {
            let var = self.vault.get(scope).unwrap().values.get(&id);

            match var {
                Some(var) => return var.clone(),
                None => {}
            }
        }

        self.report(
            &format!("Variable `{identifier}` not found"),
            identifier.node.start,
            identifier.node.end,
        );
        unreachable!("Report ends proccess");
    }

    fn set_variable_force(&mut self, identifier: &String, value: Value) {
        self.get_curr_scope_values_mut()
            .insert(identifier.clone(), value);
    }

    fn pow(&mut self, left: Value, right: Value) -> Value {
        match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left.powf(right)),
            _ => Value::None,
        }
    }

    fn range(&mut self, left: Value, right: Value) -> Value {
        match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                let left = left.round() as usize;
                let right = right.round() as usize;

                let val = (left..=right).collect::<Vec<usize>>();
                Value::Sequence(val.into_iter().map(|p| Value::Number(p as f64)).collect())
            }
            _ => Value::None,
        }
    }

    fn initialize(&mut self) {
        // Add std library
        self.vault.insert("std$0".to_owned(), ScopeValue::new());
        self.scope_stack.push("std$0".to_owned());
        self.update_current_scope();
        self.add_std_lib();

        // Initialize global
        self.vault.insert("global$0".to_owned(), ScopeValue::new());
        self.scope_stack.push("global$0".to_owned());
        self.update_current_scope();

        //Include std ref to global
        self.send_scope_ref("std$0");

        native::io::inject(self.get_curr_scope_values_mut()); // Inject io to global too
    }

    fn add_std_lib(&mut self) {
        native::inject(self);
    }

    /// Initializes a new named scope
    fn start_declaration_of_named_scope(&mut self, name: &str) -> String {
        let new_scope = format!("{}.{}$0", self.current_scope, name);
        self.init_scope(new_scope.clone());
        new_scope
    }

    fn end_declaration_of_named_scope(&mut self, name: &str) {
        self.exit_named_scope();
        self.send_scope_ref(name);
    }

    fn enter_named_scope(&mut self, name: &str) {
        self.scope_stack.push(name.to_owned());
        self.update_current_scope();
    }

    /// Exits the current named scope
    fn exit_named_scope(&mut self) -> String {
        // named scopes not clears when exiting
        // named scopes cleared only when decrementing scope
        let deleted = self.scope_stack.pop().unwrap();
        self.update_current_scope();

        deleted
    }

    /// Adds a reference to the current scope
    fn send_scope_ref(&mut self, name: &str) {
        let local_name = self.parse_local_name(name);
        self.get_curr_scope_values_mut()
            .insert(local_name, Value::ScopeRef(name.to_owned()));
        self.get_curr_scope_refs_mut().push(name.to_owned());
    }

    fn parse_local_name(&self, name: &str) -> String {
        name.rsplit_once(".")
            .unwrap_or(("", name))
            .1
            .rsplit_once("$")
            .unwrap()
            .0
            .to_owned()
    }

    /// Increments the current scope
    fn increment_scope(&mut self) {
        let (scope_name, num) = self.parse_current_scope();

        let new_scope = format!("{}${}", scope_name, num + 1);

        self.init_scope(new_scope);
    }

    /// Decrements the current scope and deletes named scopes in the current scope
    fn decrement_scope(&mut self) {
        let scope = self.current_scope.clone();

        self.remove_refs(&scope);

        self.vault.remove(&scope);
        self.scope_stack.pop();

        self.update_current_scope();
    }

    /// Removes all references and subreferences in scope
    fn remove_refs(&mut self, scope_name: &str) {
        for ref_name in self.get_scope_refs_mut(scope_name).clone() {
            self.remove_refs(&ref_name);
            self.vault.remove(&ref_name);
        }
    }

    /// Initializes the current scope
    fn init_scope(&mut self, scope_name: String) {
        self.vault.insert(scope_name.clone(), ScopeValue::new());
        self.scope_stack.push(scope_name);
        self.update_current_scope();
    }

    /// Parses the current scope name and number
    fn parse_current_scope(&mut self) -> (String, usize) {
        let (scope_name, num) = self.current_scope.rsplit_once("$").unwrap();
        let num = num.parse::<usize>().unwrap();

        (scope_name.to_owned(), num)
    }

    /// Gets the current scope values
    fn get_curr_scope_values_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self
            .vault
            .get_mut(self.current_scope.as_str())
            .unwrap()
            .values
    }

    fn get_scope_refs_mut(&mut self, scope_name: &str) -> &mut Vec<String> {
        &mut self.vault.get_mut(scope_name).unwrap().named_scope_refs
    }

    /// Gets the current named scopes in the current scope (mutable)
    fn get_curr_scope_refs_mut(&mut self) -> &mut Vec<String> {
        &mut self
            .vault
            .get_mut(self.current_scope.as_str())
            .unwrap()
            .named_scope_refs
    }

    /// Gets the current named scopes in the current scope
    // fn get_curr_scope_refs(&self) -> &Vec<String> {
    //     &self
    //         .vault
    //         .get(self.current_scope.as_str())
    //         .unwrap()
    //         .named_scope_refs
    // }

    /// Updates the current scope
    fn update_current_scope(&mut self) {
        self.current_scope = self.scope_stack.last().unwrap().clone();
    }

    /// Reports an interpreter error
    fn report(&self, error: &str, start: usize, end: usize) {
        report_error(self.path, self.source, error, start, end);
    }

    // fn report_str(&self, error: &str) {
    //     eprintln!("{}", error);
    // }
}
