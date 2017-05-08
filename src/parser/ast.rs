
use std::fmt;

use parser::ast_builder::FullToken;
use parser::tokenizer::Token;

#[derive(Debug, Clone, Copy)]
pub enum AstKind {
    Program,
    BlockStmt,
    VarStmt,
    EmptyStmt,
    IfStmt,
    ExprStmt,

    BinaryOpExpr,
    CondExpr,
    AssignExpr,
    CommaExpr,
    NameExpr
}
impl AstKind {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait AstNode where Self: fmt::Debug {
    fn kind(&self) -> AstKind;
    fn is_statement(&self) -> bool;
    fn is_expression(&self) -> bool;
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error>;

    fn tree_string(&self) -> String {
        let mut str = String::new();
        self.write_tree(&mut str).unwrap();
        str
    }
}

/*****************************************************************************
 **** ProgramNode ************************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct ProgramNode {
    source_elements: Vec<Box<AstNode>>
}
impl ProgramNode {
    pub fn new() -> ProgramNode {
        ProgramNode { source_elements: Vec::with_capacity(3) }
    }

    pub fn source_elements(&self) -> &Vec<Box<AstNode>> {
        &self.source_elements
    }
    pub fn add_source_element(&mut self, source_element: Box<AstNode>) {
        self.source_elements.push(source_element);
    }
}
impl AstNode for ProgramNode {
    fn kind(&self) -> AstKind {
        AstKind::Program
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        false
    }

    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("ProgramNode{")?;
        let mut first = true;
        for source_element in &self.source_elements {
            if ! first {
                w.write_str(", ")?;
            }
            first = false;
            source_element.write_tree(w)?;
        }
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** BlockStmtNode **********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct BlockStmtNode {
}
impl BlockStmtNode {
    pub fn new() -> BlockStmtNode {
        BlockStmtNode {}
    }
}
impl AstNode for BlockStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::BlockStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("Block{}")
    }
}

/*****************************************************************************
 **** VarStmtNode ************************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct VarStmtNode {
    variables: Vec<Box<FullToken>>
}
impl VarStmtNode {
    pub fn new() -> VarStmtNode {
        VarStmtNode { variables: Vec::with_capacity(1) }
    }

    pub fn variables(&self) -> &Vec<Box<FullToken>> {
        &self.variables
    }
    pub fn add_variable(&mut self, var: FullToken) {
        self.variables.push(Box::new(var));
    }
}
impl AstNode for VarStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::VarStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }

    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("Var{")?;
        let mut first = true;
        for variable in &self.variables {
            if ! first {
                w.write_str(", ")?;
            }
            first = false;
            variable.write_token(w)?;
        }
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** EmptyStmtNode **********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct EmptyStmtNode {
}
impl EmptyStmtNode {
    pub fn new() -> EmptyStmtNode {
        EmptyStmtNode {}
    }
}
impl AstNode for EmptyStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::EmptyStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }

    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("Empty{}")
    }
}

/*****************************************************************************
 **** IfStmtNode *************************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct IfStmtNode {
    cond_expr: Box<AstNode>,
    if_true_stmt: Box<AstNode>
}
impl IfStmtNode {
    pub fn new_if(cond_expr: Box<AstNode>, if_true_stmt: Box<AstNode>)
        -> IfStmtNode
    {
        IfStmtNode { cond_expr, if_true_stmt }
    }

    pub fn cond_expr(&self) -> &AstNode {
        self.cond_expr.as_ref()
    }
    pub fn if_true_stmt(&self) -> &AstNode {
        self.if_true_stmt.as_ref()
    }
}
impl AstNode for IfStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::IfStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }

    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("If(")?;
        self.cond_expr.write_tree(w)?;
        w.write_str("){")?;
        self.if_true_stmt.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** ExprStmtNode ***********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct ExprStmtNode {
    expr: Box<AstNode>
}
impl ExprStmtNode {
    pub fn new(expr: Box<AstNode>) -> ExprStmtNode {
        assert!(expr.is_expression());
        ExprStmtNode { expr }
    }

    pub fn expression(&self) -> &AstNode {
        self.expr.as_ref()
    }
}
impl AstNode for ExprStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::ExprStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("ExprStmt{")?;
        self.expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** BinaryExprNode *********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct BinaryOpExprNode {
    binary_op: FullToken,
    left_expr: Box<AstNode>,
    right_expr: Box<AstNode>
}
impl BinaryOpExprNode {
    pub fn new(binary_op: FullToken, left_expr: Box<AstNode>, right_expr: Box<AstNode>)
        -> BinaryOpExprNode
    {
        assert!(left_expr.is_expression());
        assert!(right_expr.is_expression());
        BinaryOpExprNode { binary_op, left_expr, right_expr }
    }

    pub fn binary_op(&self) -> &FullToken {
        &self.binary_op
    }
    pub fn left_expr(&self) -> &AstNode {
        self.left_expr.as_ref()
    }
    pub fn right_expr(&self) -> &AstNode {
        self.right_expr.as_ref()
    }
}
impl AstNode for BinaryOpExprNode {
    fn kind(&self) -> AstKind {
        AstKind::BinaryOpExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("BinaryOpExpr(")?;
        self.binary_op.write_token(w)?;
        w.write_str("){")?;
        self.left_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.right_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** CondExprNode ***********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct CondExprNode {
    cond_expr: Box<AstNode>,
    if_expr: Box<AstNode>,
    else_expr: Box<AstNode>
}
impl CondExprNode {
    pub fn new(cond_expr: Box<AstNode>, if_expr: Box<AstNode>, else_expr: Box<AstNode>)
        -> CondExprNode
    {
        assert!(cond_expr.is_expression());
        assert!(if_expr.is_expression());
        assert!(else_expr.is_expression());
        CondExprNode { cond_expr, if_expr, else_expr }
    }

    pub fn cond_expr(&self) -> &AstNode {
        self.cond_expr.as_ref()
    }
    pub fn if_expr(&self) -> &AstNode {
        self.if_expr.as_ref()
    }
    pub fn else_expr(&self) -> &AstNode {
        self.else_expr.as_ref()
    }
}
impl AstNode for CondExprNode {
    fn kind(&self) -> AstKind {
        AstKind::CondExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("CondExpr{")?;
        self.cond_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.if_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.else_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** AssignExprNode *********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct AssignExprNode {
    assign_op: FullToken,
    left_expr: Box<AstNode>,
    right_expr: Box<AstNode>
}
impl AssignExprNode {
    pub fn new(assign_op: FullToken, left_expr: Box<AstNode>, right_expr: Box<AstNode>)
        -> AssignExprNode
    {
        // FIXME: assert that left_expr is a valid lvalue expression.
        assert!(left_expr.is_expression());
        assert!(right_expr.is_expression());
        assert!(assign_op.kind().is_assignment_op());
        AssignExprNode { assign_op, left_expr, right_expr }
    }

    pub fn assignment_op(&self) -> &FullToken {
        &self.assign_op
    }
    pub fn left_expr(&self) -> &AstNode {
        self.left_expr.as_ref()
    }
    pub fn right_expr(&self) -> &AstNode {
        self.right_expr.as_ref()
    }
}
impl AstNode for AssignExprNode {
    fn kind(&self) -> AstKind {
        AstKind::AssignExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("AssignExpr(")?;
        self.assign_op.write_token(w)?;
        w.write_str("){")?;
        self.left_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.right_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** CommaExprNode **********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct CommaExprNode {
    left_expr: Box<AstNode>,
    right_expr: Box<AstNode>
}
impl CommaExprNode {
    pub fn new(left_expr: Box<AstNode>, right_expr: Box<AstNode>) -> CommaExprNode {
        assert!(left_expr.is_expression());
        assert!(right_expr.is_expression());
        CommaExprNode { left_expr, right_expr }
    }

    pub fn left_expr(&self) -> &AstNode {
        self.left_expr.as_ref()
    }
    pub fn right_expr(&self) -> &AstNode {
        self.right_expr.as_ref()
    }
}
impl AstNode for CommaExprNode {
    fn kind(&self) -> AstKind {
        AstKind::CommaExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("CommaExpr{")?;
        self.left_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.right_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** NameExprNode ***********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct NameExprNode {
    name: FullToken
}
impl NameExprNode {
    pub fn new(name: FullToken) -> NameExprNode {
        assert!(name.kind().is_identifier());
        NameExprNode { name }
    }

    pub fn name(&self) -> &FullToken {
        &self.name
    }
}
impl AstNode for NameExprNode {
    fn kind(&self) -> AstKind {
        AstKind::NameExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("NameExpr{")?;
        self.name.write_token(w);
        w.write_str("}")?;
        Ok(())
    }
}
