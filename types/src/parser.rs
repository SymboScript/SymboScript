use std::fmt::{self};

use crate::lexer::TokenValue;
use serde::{Deserialize, Serialize};

pub type BlockStatement = Vec<Statement>;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ast {
    pub program: Program,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Program {
    pub node: Node,
    pub body: BlockStatement,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Node {
    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,
}

impl Node {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    pub node: Node,
    pub key: Expression,
    pub value: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Statement {
    ExpressionStatement(Expression),
    ReturnStatement(ReturnStatement),
    ThrowStatement(ThrowStatement),
    ContinueStatement(Node),
    BreakStatement(Node),
    YieldStatement(YieldStatement),
    VariableDeclaration(VariableDeclarator),
    FunctionDeclaration(FunctionDeclarator),
    ScopeDeclaration(ScopeDeclarator),
    IfStatement(IfStatement),
    ForStatement(Box<ForStatement>),
    WhileStatement(WhileStatement),
    LoopStatement(LoopStatement),
    TryStatement(TryStatement),
    BlockStatement(BlockStatement),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TryStatement {
    pub node: Node,
    pub body: BlockStatement,
    pub handler: BlockStatement,
    pub finalizer: BlockStatement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoopStatement {
    pub node: Node,
    pub body: BlockStatement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhileStatement {
    pub node: Node,
    pub test: Expression,
    pub body: BlockStatement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForStatement {
    pub node: Node,
    pub init: Statement,
    pub test: Expression,
    pub update: Expression,
    pub body: BlockStatement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnStatement {
    pub node: Node,
    pub argument: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThrowStatement {
    pub node: Node,
    pub argument: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YieldStatement {
    pub node: Node,
    pub argument: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableDeclarator {
    pub node: Node,
    pub id: String,
    pub init: Expression,
    pub is_formula: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDeclarator {
    pub node: Node,
    pub id: String,
    pub params: Vec<String>,
    pub body: BlockStatement,
    pub is_async: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopeDeclarator {
    pub node: Node,
    pub id: String,
    pub body: BlockStatement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IfStatement {
    pub node: Node,
    pub test: Expression,
    pub consequent: BlockStatement,
    pub alternate: BlockStatement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Expression {
    BinaryExpression(Box<BinaryExpression>),
    UnaryExpression(Box<UnaryExpression>),
    ConditionalExpression(Box<ConditionalExpression>),
    CallExpression(Box<CallExpression>),
    MemberExpression(Box<MemberExpression>),
    SequenceExpression(Box<SequenceExpression>),
    WordExpression(Box<WordExpression>),
    Literal(TokenValue),
    Identifier(String),
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BinaryExpression {
    pub node: Node,
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnaryExpression {
    pub node: Node,
    pub operator: UnaryOperator,
    pub right: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConditionalExpression {
    pub node: Node,
    pub test: Expression,
    pub consequent: Expression,
    pub alternate: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberExpression {
    pub node: Node,
    pub object: Expression,
    pub property: Expression,
    pub is_expr: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequenceExpression {
    pub node: Node,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WordExpression {
    pub node: Node,
    pub argument: Expression,
    pub operator: BinaryOperator,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    // Binary
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Range,
    Modulo,

    And,
    Or,
    Xor,

    BitAnd,
    BitOr,
    BitXor,

    BitLeftShift,
    BitRightShift,

    Assign,
    FormulaAssign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    PowerAssign,
    ModuloAssign,

    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    // Unary
    BitNot,
    Not,
    PlusPlus,
    MinusMinus,

    Minus,
    Plus,
}

//----------Display------------

fn format_vec<T: fmt::Display>(vec: &Vec<T>, separator: &str) -> String {
    vec.iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(separator)
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.program)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.body {
            writeln!(f, "{}", statement)?;
        }

        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::ExpressionStatement(expr) => write!(f, "{};", expr),
            Statement::VariableDeclaration(expr) => write!(f, "{};", expr),
            Statement::FunctionDeclaration(expr) => write!(f, "{}", expr),
            Statement::ScopeDeclaration(expr) => write!(f, "{}", expr),
            Statement::ReturnStatement(expr) => write!(f, "{}", expr),
            Statement::ThrowStatement(expr) => write!(f, "{}", expr),
            Statement::ContinueStatement(_) => write!(f, "continue;"),
            Statement::BreakStatement(_) => write!(f, "break;"),
            Statement::YieldStatement(expr) => write!(f, "{}", expr),
            Statement::IfStatement(expr) => write!(f, "{}", expr),
            Statement::ForStatement(expr) => write!(f, "{}", expr),
            Statement::WhileStatement(expr) => write!(f, "{}", expr),
            Statement::LoopStatement(expr) => write!(f, "{}", expr),
            Statement::TryStatement(expr) => write!(f, "{}", expr),
            Statement::BlockStatement(expr) => write!(f, "{{\n{}\n}}", format_vec(expr, "\n")),
        }
    }
}

impl fmt::Display for TryStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "try {{\n{}\n}} catch {{\n{}\n}} finally {{\n{}\n}}",
            format_vec(&self.body, "\n"),
            format_vec(&self.handler, "\n"),
            format_vec(&self.finalizer, "\n")
        )
    }
}

impl fmt::Display for ThrowStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "throw {};", self.argument)
    }
}

impl fmt::Display for ForStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "for ({} {}; {}) {{\n{}\n}}",
            self.init,
            self.test,
            self.update,
            format_vec(&self.body, "\n")
        )
    }
}

impl fmt::Display for WhileStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "while ({}) {{\n{}\n}}",
            self.test,
            format_vec(&self.body, "\n")
        )
    }
}

impl fmt::Display for LoopStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop {{\n{}\n}}", format_vec(&self.body, "\n"))
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.argument)
    }
}

impl fmt::Display for YieldStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "yield {};", self.argument)
    }
}

impl fmt::Display for VariableDeclarator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "let {} {} {}",
            self.id,
            if self.is_formula == true { ":=" } else { "=" },
            self.init
        )
    }
}

impl fmt::Display for FunctionDeclarator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}fn {}({}) {{\n{}\n}}",
            if self.is_async { "async " } else { "" },
            self.id,
            format_vec(&self.params, ", "),
            format_vec(&self.body, "\n")
        )
    }
}

impl fmt::Display for ScopeDeclarator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "scope {} {{\n{}\n}}",
            self.id,
            format_vec(&self.body, "\n")
        )
    }
}

impl fmt::Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if {} {{\n{}\n}} else {{\n{}\n}}",
            self.test,
            format_vec(&self.consequent, "\n"),
            format_vec(&self.alternate, "\n")
        )
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::BinaryExpression(expr) => write!(f, "({})", expr),
            Expression::Literal(token) => write!(f, "{}", token),
            Expression::Identifier(token) => write!(f, "{}", token),

            Expression::UnaryExpression(expr) => write!(f, "({})", expr),
            Expression::ConditionalExpression(expr) => write!(f, "({})", expr),
            Expression::CallExpression(expr) => write!(f, "({})", expr),
            Expression::MemberExpression(expr) => write!(f, "({})", expr),
            Expression::WordExpression(expr) => write!(f, "({})", expr),
            Expression::SequenceExpression(expr) => {
                let len = expr.expressions.len();
                let mut k = 0;
                write!(f, "[")?;
                for expr in &expr.expressions {
                    k += 1;
                    write!(f, "{}", expr)?;
                    if k < len {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Expression::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.left, self.operator, self.right)
    }
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.right)
    }
}

impl fmt::Display for ConditionalExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ? {} : {}",
            self.test, self.consequent, self.alternate
        )
    }
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.callee, self.arguments)
    }
}

impl fmt::Display for MemberExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_expr {
            write!(f, "{}.[{}]", self.object, self.property)
        } else {
            write!(f, "{}.{}", self.object, self.property)
        }
    }
}

impl fmt::Display for SequenceExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for expr in &self.expressions {
            write!(f, "{},", expr)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for WordExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.operator, self.argument)
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Power => write!(f, "^"),
            BinaryOperator::Range => write!(f, ".."),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
            BinaryOperator::Xor => write!(f, "^"),
            BinaryOperator::BitAnd => write!(f, "&"),
            BinaryOperator::BitOr => write!(f, "|"),
            BinaryOperator::BitXor => write!(f, "bxor"),
            BinaryOperator::BitLeftShift => write!(f, "<<"),
            BinaryOperator::BitRightShift => write!(f, ">>"),
            BinaryOperator::Assign => write!(f, "="),
            BinaryOperator::FormulaAssign => write!(f, ":="),
            BinaryOperator::PlusAssign => write!(f, "+="),
            BinaryOperator::MinusAssign => write!(f, "-="),
            BinaryOperator::MultiplyAssign => write!(f, "*="),
            BinaryOperator::DivideAssign => write!(f, "/="),
            BinaryOperator::PowerAssign => write!(f, "^="),
            BinaryOperator::ModuloAssign => write!(f, "%="),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::Less => write!(f, "<"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::MinusMinus => write!(f, "--"),
            UnaryOperator::BitNot => write!(f, "~"),
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::PlusPlus => write!(f, "++"),
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Plus => write!(f, "+"),
        }
    }
}
