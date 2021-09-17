//! Abstract Syntax Tree data structure definitions, plus walker traits for tree walkers

/// All binary and unary operators
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div, 
    Mod,
    
    XOR,
    AND,
    OR,
    INV,
    ShRight,
    ShLeft,
    
    Less,
    Greater,
    Eq,
    LessEq,
    GreaterEq,
    AndAnd,
    OrOr,
    
    Not,    
}

