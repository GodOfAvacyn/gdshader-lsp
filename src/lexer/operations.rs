use crate::lexer::{Token, TokenKind};
use TokenKind::*;

pub enum OperationType {
    Comparison,
    Equal,
    Number,
    Int,
    Bool,
}

pub trait MaybeOperator {
    fn to_unary_op(&self) -> Option<OperationType>;
    fn to_binary_op(&self) -> Option<OperationType>;
    fn to_assignment_op(&self) -> Option<OperationType>;
}
impl MaybeOperator for Token {
    fn to_unary_op(&self) -> Option<OperationType> {
        match self.kind {
            Plus | Dash => Some(OperationType::Number),
            Bang => Some(OperationType::Bool),
            Tilde => Some(OperationType::Int),
            _ => None
        }
    }

    fn to_binary_op(&self) -> Option<OperationType> {
        match self.kind {
            LeftOp | RightOp | Percent | VerticalBar | Caret | Ampersand => 
                Some(OperationType::Int),
            LeqOp | GeqOp | LeftAngle | RightAngle | And | Or => 
                Some(OperationType::Comparison),
            Plus | Dash | Star | Slash =>
                Some(OperationType::Number),
            EqOp | NeqOp =>
                Some(OperationType::Equal),
            _ => None
        }    
    }

    fn to_assignment_op(&self) -> Option<OperationType> {
        match self.kind {
            AddAssign | SubAssign | MulAssign | DivAssign | ModAssign =>
                Some(OperationType::Number),
            Equal => Some(OperationType::Equal),
            _ => None
        }
    }
}


