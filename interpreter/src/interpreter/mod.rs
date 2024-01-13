use std::{collections::HashMap, fs, path::Path};

use rand::{distributions::Alphanumeric, Rng};
use symboscript_types::{interpreter::*, lexer::*, parser::*};
use symboscript_utils::report_error;

mod macro_utils;
mod native;

use crate::loop_controls;
use symboscript_parser as parser;

use self::native::{get_values, StdLang};

pub struct Interpreter<'a> {
    /// Path of the source file
    paths: Vec<String>,

    /// Sources of programs
    sources: Vec<String>,

    ast: &'a Ast,

    scope_stack: Vec<String>,

    current_scope: String,

    vault: Vault,

    std_lang: StdLang,
}

fn get_full_path(path: &str) -> String {
    fs::canonicalize(Path::new(path))
        .unwrap()
        .display()
        .to_string()
}

impl<'a> Interpreter<'a> {
    pub fn new(path: &'a str, source: &'a str, ast: &'a Ast) -> Self {
        let vault = Vault::new();

        Self {
            paths: vec![get_full_path(path)],
            sources: vec![source.to_owned()],
            ast,
            scope_stack: vec![],
            current_scope: String::new(),
            vault,
            std_lang: get_values(),
        }
    }

    pub fn run(&mut self) {
        self.initialize();

        self.eval_ast(self.ast.clone());
    }

    fn eval_ast(&mut self, ast: Ast) {
        self.eval_block(&ast.program.body);
    }

    fn eval_block(&mut self, body: &BlockStatement) -> ControlFlow {
        for statement in body {
            let control = self.eval_statement(&statement);

            match control {
                ControlFlow::None => {}
                _ => return control,
            }
        }

        return ControlFlow::None;
    }

    fn eval_statement(&mut self, statement: &Statement) -> ControlFlow {
        match statement {
            Statement::ExpressionStatement(expr) => {
                self.eval_expression(&expr);
            }

            Statement::ReturnStatement(v) => {
                return ControlFlow::Return(self.eval_expression(&v.argument));
            }
            Statement::ThrowStatement(v) => {
                return ControlFlow::Throw(self.eval_expression(&v.argument));
            }
            Statement::ContinueStatement(_) => {
                return ControlFlow::Continue;
            }
            Statement::BreakStatement(_) => {
                return ControlFlow::Break;
            }
            Statement::YieldStatement(_) => todo!(),
            Statement::VariableDeclaration(decl) => {
                let value = if decl.is_formula {
                    Value::Ast(decl.init.clone())
                } else {
                    self.eval_expression(&decl.init)
                };

                self.declare_variable(&decl.id, value);
            }
            Statement::FunctionDeclaration(decl) => {
                self.declare_variable(&decl.id, Value::Function(decl.clone()));
            }
            Statement::ScopeDeclaration(decl) => {
                let scope = self.start_declaration_of_named_scope(&decl.id);

                self.eval_block(&decl.body);
                self.end_declaration_of_named_scope(&scope);
            }
            Statement::ContextDeclaration(decl) => {
                let scope = self.start_declaration_of_named_scope(&decl.id);
                self.declare_variable(&"this".to_owned(), Value::ScopeRef(scope.clone()));
                self.eval_block(&decl.body);
                self.end_declaration_of_named_scope(&scope);
            }
            Statement::IfStatement(if_stmt) => {
                return self.eval_if_statement(if_stmt);
            }
            Statement::ForStatement(_) => todo!(),
            Statement::WhileStatement(while_stmt) => {
                self.eval_while_statement(while_stmt);
            }
            Statement::LoopStatement(loop_stmt) => {
                self.eval_loop_statement(loop_stmt);
            }
            Statement::BlockStatement(body) => {
                self.increment_scope();
                self.eval_block(body);
                self.decrement_scope();
            }

            Statement::AssignStatement(assign_stmt) => {
                return self.eval_assign_statement(assign_stmt);
            }

            Statement::ImportStatement(import_stmt) => {
                self.eval_import_statement(import_stmt);
            }
        }

        return ControlFlow::None;
    }

    fn eval_import_statement(&mut self, import_stmt: &ImportStatement) {
        let source_name = if import_stmt.source.name.ends_with(".syms") {
            import_stmt.source.name.clone()
        } else {
            format!("{}.syms", import_stmt.source.name.clone())
        };

        let current_path = Path::new(self.paths.last().unwrap());

        let current_path = match current_path.parent() {
            Some(path) => path,
            None => Path::new("./"),
        };

        let file_path = current_path.join(source_name);
        let file_path = file_path.display().to_string();

        let file_contents = fs::read_to_string(&file_path);

        match file_contents {
            Ok(contents) => {
                let ast = parser::Parser::new(&file_path, &contents).parse();

                {
                    self.sources.push(contents.clone());
                    self.paths.push(file_path.clone());

                    {
                        let scope =
                            self.start_declaration_of_named_scope(&import_stmt.as_name.name);

                        // Declare standard variables
                        self.declare_variable(
                            &"__file__".to_owned(),
                            Value::Str(file_path.clone()),
                        );
                        self.declare_variable(
                            &"__name__".to_owned(),
                            Value::Str(import_stmt.as_name.name.clone()),
                        );
                        self.declare_variable(&"__module__".to_owned(), Value::Bool(true));

                        // Evaluate the AST
                        self.eval_ast(ast);
                        self.end_declaration_of_named_scope(&scope);
                    }

                    self.paths.pop();
                    self.sources.pop();
                }
            }
            Err(e) => self.report(
                &format!("Failed to import module: `{}`\n{e}", import_stmt.source),
                import_stmt.node.start,
                import_stmt.node.end,
            ),
        }
    }

    /// Only for std library usage
    fn eval_file(&mut self, file_path: &str) {
        let contents = fs::read_to_string(&file_path).unwrap();
        let ast = parser::Parser::new(&file_path, &contents).parse();
        self.eval_ast(ast);
    }

    fn eval_assign_statement(&mut self, assign_stmt: &AssignStatement) -> ControlFlow {
        let right = self.eval_expression(&assign_stmt.right);

        let var_val = self.get_variable_value_mut(&assign_stmt.left);

        match assign_stmt.operator {
            AssignOperator::Assign => {
                *var_val = right;
            }
            AssignOperator::PlusAssign => {
                *var_val += right;
            }
            AssignOperator::MinusAssign => {
                *var_val -= right;
            }
            AssignOperator::MultiplyAssign => {
                *var_val *= right;
            }
            AssignOperator::DivideAssign => {
                *var_val /= right;
            }
            AssignOperator::PowerAssign => {
                *var_val = (*var_val).pow(&right);
            }
            AssignOperator::ModuloAssign => {
                *var_val %= right;
            }
        }

        ControlFlow::None
    }

    fn eval_if_statement(&mut self, if_stmt: &IfStatement) -> ControlFlow {
        if (self.eval_expression(&if_stmt.test)).as_bool() {
            return self.eval_block(&if_stmt.consequent);
        } else {
            return self.eval_block(&if_stmt.alternate);
        }
    }

    fn eval_while_statement(&mut self, while_stmt: &WhileStatement) -> ControlFlow {
        self.increment_scope();

        while (self.eval_expression(&while_stmt.test)).as_bool() {
            loop_controls!(self, while_stmt.body);
        }

        self.decrement_scope();

        return ControlFlow::None;
    }

    fn eval_loop_statement(&mut self, loop_stmt: &LoopStatement) -> ControlFlow {
        self.increment_scope();

        loop {
            loop_controls!(self, loop_stmt.body);
        }

        self.decrement_scope();

        return ControlFlow::None;
    }

    fn eval_expression(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::BinaryExpression(binary_expr) => self.eval_binary_expression(binary_expr),
            Expression::UnaryExpression(unary_expr) => self.eval_unary_expression(unary_expr),
            Expression::ConditionalExpression(_) => todo!(),
            Expression::CallExpression(call_expr) => self.eval_call_expression(call_expr),
            Expression::MemberExpression(member_expr) => self.eval_member_expression(member_expr),
            Expression::SequenceExpression(_) => todo!(),
            Expression::WordExpression(_) => todo!(),

            Expression::Literal(val) => self.match_literal(val),

            Expression::Identifier(id) => self.get_variable_value(id),

            Expression::None(_) => Value::None,
        }
    }

    fn set_native_value(&mut self, name: &str, value: Value) {
        self.vault
            .get_mut(&format!("std$0.&{name}$0"))
            .unwrap()
            .values
            .insert("$value".to_owned(), value);
    }

    fn eval_member_expression(&mut self, member_expr: &MemberExpression) -> Value {
        let object = self.eval_expression(&member_expr.object);

        let object: Identifier = match object {
            Value::ScopeRef(ref_name) => Identifier {
                name: ref_name.clone(),
                node: member_expr.node.clone(),
            },
            Value::Sequence(_) => self.native_id("sequence", object, member_expr.node.clone()),
            Value::None => self.native_id("none", object, member_expr.node.clone()),
            Value::Number(_) => self.native_id("number", object, member_expr.node.clone()),
            Value::Bool(_) => self.native_id("bool", object, member_expr.node.clone()),
            Value::Str(_) => self.native_id("str", object, member_expr.node.clone()),
            Value::Ast(_) => self.native_id("ast", object, member_expr.node.clone()),
            Value::Err(_) => self.native_id("err", object, member_expr.node.clone()),
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

        let property = match &member_expr.property {
            Expression::Identifier(id) => {
                if member_expr.is_expr {
                    let property = self.eval_expression(&member_expr.property);

                    self.get_variable_value(&Identifier {
                        name: property.to_string(),
                        node: member_expr.node.clone(),
                    })
                } else {
                    self.get_variable_value(id)
                }
            }
            Expression::CallExpression(call_expr) => self.eval_call_expression(call_expr),
            _ => {
                let property = self.eval_expression(&member_expr.property);

                self.get_variable_value(&Identifier {
                    name: property.to_string(),
                    node: member_expr.node.clone(),
                })
            }
        };

        self.exit_named_scope();

        property
    }

    fn native_id(&mut self, name: &str, value: Value, node: Node) -> Identifier {
        self.set_native_value(name, value);
        Identifier {
            name: format!("std$0.&{name}$0").to_owned(),
            node,
        }
    }

    fn eval_call_expression(&mut self, call_expr: &CallExpression) -> Value {
        let var = self.get_variable_value(&Identifier {
            name: call_expr.callee.clone(),
            node: call_expr.node.clone(),
        });

        let args = match &call_expr.arguments {
            Expression::SequenceExpression(seq_exp) => seq_exp,
            _ => unreachable!("Arguments can only be sequence expressions"),
        };

        let args = args
            .expressions
            .iter()
            .map(|expr| self.eval_expression(expr))
            .collect::<Vec<Value>>();

        let result = match var {
            Value::NativeFunction(name) => native::run_function(self, &call_expr, &name, &args),
            Value::Function(declarator) => {
                if declarator.params.len() != args.len() {
                    self.report(
                        &format!(
                            "Expected {} arguments, got {}",
                            declarator.params.len(),
                            args.len()
                        ),
                        call_expr.node.start,
                        call_expr.node.end,
                    );
                    unreachable!("Report ends proccess");
                }
                self.increment_scope();

                for (i, variable) in declarator.params.iter().enumerate() {
                    self.declare_variable(&variable, args[i].clone());
                }

                let control = self.eval_block(&declarator.body);

                self.decrement_scope();
                match control {
                    ControlFlow::Return(val) => val,
                    ControlFlow::Throw(val) => Value::Err(format!("{}", val)),
                    _ => Value::None,
                }
            }

            _ => {
                self.report(
                    &format!("`{}` is not a function", call_expr.callee),
                    call_expr.node.start,
                    call_expr.node.end,
                );
                unreachable!("Report ends proccess");
            }
        };

        result
    }

    fn eval_unary_expression(&mut self, expression: &UnaryExpression) -> Value {
        let right = self.eval_expression(&expression.right);

        match expression.operator {
            UnaryOperator::Plus => right,
            UnaryOperator::Minus => -right,
            UnaryOperator::Not => !right,
            UnaryOperator::BitNot => !right,
            UnaryOperator::PlusPlus => right + Value::Number(1.0),
            UnaryOperator::MinusMinus => right - Value::Number(1.0),
        }
    }

    fn eval_binary_expression(&mut self, expression: &BinaryExpression) -> Value {
        let left = match &expression.left {
            Expression::Identifier(id) => self.get_variable_value(id),
            _ => self.eval_expression(&expression.left),
        };

        let right = self.eval_expression(&expression.right);

        match expression.operator {
            BinaryOperator::Add => left + right,
            BinaryOperator::Substract => left - right,
            BinaryOperator::Multiply => left * right,
            BinaryOperator::Divide => left / right,
            BinaryOperator::Power => left.pow(&right),
            BinaryOperator::Range => left.range(&right),

            BinaryOperator::Modulo => left % right,

            BinaryOperator::And => left.and(&right),
            BinaryOperator::Or => left.or(&right),
            BinaryOperator::Xor => left.xor(&right),

            BinaryOperator::BitAnd => left.bit_and(&right),
            BinaryOperator::BitOr => left.bit_or(&right),
            BinaryOperator::BitXor => left.bit_xor(&right),

            BinaryOperator::BitLeftShift => left << right,
            BinaryOperator::BitRightShift => left >> right,

            BinaryOperator::Equal => left.equal(&right),
            BinaryOperator::NotEqual => left.not_equal(&right),
            BinaryOperator::Less => left.less(&right),
            BinaryOperator::LessEqual => left.less_equal(&right),
            BinaryOperator::Greater => left.greater(&right),
            BinaryOperator::GreaterEqual => left.greater_equal(&right),
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

    fn get_cur_value(&mut self, id: &String) -> Value {
        let (scope_name, _) = self.parse_current_scope();
        self.vault
            .get(&format!("{}$0", scope_name))
            .unwrap()
            .values
            .get(id)
            .unwrap()
            .clone()
    }

    fn get_variable_value_mut(&mut self, identifier: &Identifier) -> &mut Value {
        let id = identifier.name.clone();

        let mut scope_index = None;

        for (i, scope) in self.scope_stack.iter().enumerate().rev() {
            if self.vault.get(scope).unwrap().values.contains_key(&id) {
                scope_index = Some(i);
                break;
            }
        }

        match scope_index {
            Some(index) => {
                let scope = &self.scope_stack[index];
                self.vault
                    .get_mut(scope)
                    .unwrap()
                    .values
                    .get_mut(&id)
                    .unwrap()
            }
            None => {
                self.report(
                    &format!("Variable `{}` not found", id),
                    identifier.node.start,
                    identifier.node.end,
                );
                unreachable!("Report ends process");
            }
        }
    }

    // fn get_variable_scope(&self, identifier: &Identifier) -> String {
    //     for scope in self.scope_stack.iter().rev() {
    //         let var = self.vault.get(scope).unwrap().values.get(&identifier.name);

    //         match var {
    //             Some(_) => return scope.clone(),
    //             None => {}
    //         }
    //     }

    //     self.report(
    //         &format!("Variable `{identifier}` not found"),
    //         identifier.node.start,
    //         identifier.node.end,
    //     );
    //     unreachable!("Report ends proccess");
    // }

    fn declare_variable(&mut self, identifier: &String, value: Value) {
        self.get_curr_scope_values_mut()
            .insert(identifier.clone(), value);
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

        // Initialize standard variables
        self.declare_variable(&"__file__".to_owned(), Value::Str(self.paths[0].clone()));
        self.declare_variable(&"__name__".to_owned(), Value::Str("main".to_owned()));
        self.declare_variable(&"__module__".to_owned(), Value::Bool(false));

        //Include std ref to global
        self.send_scope_ref("std$0");

        native::io::inject(self.get_curr_scope_values_mut()); // Inject io to global too
    }

    fn add_std_lib(&mut self) {
        native::inject(self);
    }

    fn gen_id(&mut self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }

    fn start_declaration_of_id_scope(&mut self) -> String {
        let new_scope = self.gen_id();

        self.start_declaration_of_named_scope(&new_scope)
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
        report_error(
            self.paths.last().unwrap(),
            self.sources.last().unwrap(),
            error,
            start,
            end,
        );
    }

    // fn report_str(&self, error: &str) {
    //     eprintln!("{}", error);
    // }
}
