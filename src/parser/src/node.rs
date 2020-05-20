use crate::ast::AstIndex;
use std::fmt;

pub type ConstantIndex = u32;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Empty,
    Id(ConstantIndex),
    Lookup(Vec<LookupNode>),
    Copy(AstIndex),
    BoolTrue,
    BoolFalse,
    Number0,
    Number1,
    Number(ConstantIndex),
    Str(ConstantIndex),
    Num2(Vec<AstIndex>),
    Num4(Vec<AstIndex>),
    List(Vec<AstIndex>),
    Range {
        start: AstIndex,
        end: AstIndex,
        inclusive: bool,
    },
    RangeFrom {
        start: AstIndex,
    },
    RangeTo {
        end: AstIndex,
        inclusive: bool,
    },
    RangeFull,
    Map(Vec<(ConstantIndex, AstIndex)>),
    MainBlock {
        body: Vec<AstIndex>,
        local_count: usize,
    },
    Block(Vec<AstIndex>),
    Expressions(Vec<AstIndex>),
    CopyExpression(AstIndex),
    Negate(AstIndex),
    Function(Function),
    Call {
        function: AstIndex,
        args: Vec<AstIndex>,
    },
    Assign {
        target: AssignTarget,
        expression: AstIndex,
    },
    MultiAssign {
        targets: Vec<AssignTarget>,
        expressions: AstIndex,
    },
    Op {
        // TODO rename -> BinaryOp
        op: AstOp,
        lhs: AstIndex,
        rhs: AstIndex,
    },
    If(AstIf),
    For(AstFor),
    While {
        condition: AstIndex,
        body: AstIndex,
    },
    Until {
        condition: AstIndex,
        body: AstIndex,
    },
    Break,
    Continue,
    Return,
    ReturnExpression(AstIndex),
    Debug {
        expression_string: ConstantIndex,
        expression: AstIndex,
    },
}

impl Default for Node {
    fn default() -> Self {
        Node::Empty
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Node::*;
        match self {
            Empty => write!(f, "Empty"),
            Id(_) => write!(f, "Id"),
            Lookup(_) => write!(f, "Lookup"),
            Copy(_) => write!(f, "Copy"),
            BoolTrue => write!(f, "BoolTrue"),
            BoolFalse => write!(f, "BoolFalse"),
            Number(_) => write!(f, "Number"),
            Number0 => write!(f, "Number0"),
            Number1 => write!(f, "Number1"),
            Str(_) => write!(f, "Str"),
            Num2(_) => write!(f, "Num2"),
            Num4(_) => write!(f, "Num4"),
            List(_) => write!(f, "List"),
            Range { .. } => write!(f, "Range"),
            RangeFrom { .. } => write!(f, "RangeFrom"),
            RangeTo { .. } => write!(f, "RangeTo"),
            RangeFull => write!(f, "RangeFull"),
            Map(_) => write!(f, "Map"),
            MainBlock { .. } => write!(f, "MainBlock"),
            Block(_) => write!(f, "Block"),
            Expressions(_) => write!(f, "Expressions"),
            CopyExpression(_) => write!(f, "CopyExpression"),
            Negate(_) => write!(f, "Negate"),
            Function(_) => write!(f, "Function"),
            Call { .. } => write!(f, "Call"),
            Assign { .. } => write!(f, "Assign"),
            MultiAssign { .. } => write!(f, "MultiAssign"),
            Op { .. } => write!(f, "Op"),
            If(_) => write!(f, "If"),
            For(_) => write!(f, "For"),
            While { .. } => write!(f, "While"),
            Until { .. } => write!(f, "Until"),
            Break => write!(f, "Break"),
            Continue => write!(f, "Continue"),
            Return => write!(f, "Return"),
            ReturnExpression(_) => write!(f, "ReturnExpression"),
            Debug { .. } => write!(f, "Debug"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub args: Vec<ConstantIndex>,
    pub captures: Vec<ConstantIndex>,
    pub local_count: usize,
    pub body: AstIndex,
    pub is_instance_function: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstFor {
    pub args: Vec<ConstantIndex>, // TODO Vec<Option<ConstantIndex>>
    pub ranges: Vec<AstIndex>,
    pub condition: Option<AstIndex>,
    pub body: AstIndex,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstIf {
    pub condition: AstIndex,
    pub then_node: AstIndex,
    pub else_if_blocks: Vec<(AstIndex, AstIndex)>,
    pub else_node: Option<AstIndex>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AstOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    And,
    Or,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scope {
    Global,
    Local,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LookupNode {
    Id(ConstantIndex),
    Index(AstIndex),
    Call(Vec<AstIndex>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignTarget {
    pub target_index: AstIndex,
    pub scope: Scope,
}
