use crate::symbol_table::{DataType, SymbolTable};
use crate::token::Span;

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),

    FunctionDef {
        name: usize,
        return_type: DataType,
        body: Vec<AstNode>,
        span: Span,
    },

    Declaration {
        var: usize,
        data_type: DataType,
        init: Option<Box<AstNode>>,
        span: Span,
    },

    Assignment {
        target: usize,
        value: Box<AstNode>,
        span: Span,
    },

    BinaryOp {
        op: BinOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
        span: Span,
    },

    Identifier(usize, Span),
    IntLiteral(i64, Span),
    FloatLiteral(f64, Span),
    StringLiteral(String, Span),
}

enum AstChild<'a> {
    Leaf(String),
    Node(&'a AstNode),
}

fn op_symbol(op: BinOp) -> char {
    match op {
        BinOp::Add => '+',
        BinOp::Sub => '-',
        BinOp::Mul => '*',
        BinOp::Div => '/',
    }
}

impl AstNode {
    pub fn render(&self, symbols: &SymbolTable) -> String {
        let mut out = String::new();
        self.render_into(&mut out, symbols, "", "", true);
        out
    }

    pub fn print(&self, symbols: &SymbolTable) {
        print!("{}", self.render(symbols));
    }

    fn render_into(
        &self,
        out: &mut String,
        symbols: &SymbolTable,
        prefix: &str,
        connector: &str,
        is_last: bool,
    ) {
        out.push_str(prefix);
        out.push_str(connector);
        out.push_str(&self.label(symbols));
        out.push('\n');

        let is_root = connector.is_empty();
        let new_prefix = if is_root {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let children = self.children();
        let n = children.len();
        for (i, child) in children.into_iter().enumerate() {
            let last = i == n - 1;
            let conn = if last { "└── " } else { "├── " };
            match child {
                AstChild::Node(node) => node.render_into(out, symbols, &new_prefix, conn, last),
                AstChild::Leaf(text) => {
                    out.push_str(&new_prefix);
                    out.push_str(conn);
                    out.push_str(&text);
                    out.push('\n');
                }
            }
        }
    }

    fn label(&self, symbols: &SymbolTable) -> String {
        match self {
            AstNode::Program(_) => "Programa".to_string(),

            AstNode::FunctionDef {
                name, return_type, ..
            } => {
                format!("Funcao ({}) : {:?}", symbols.name_of(*name), return_type)
            }

            AstNode::Declaration { var, data_type, .. } => {
                format!("Declaracao ({}) : {:?}", symbols.name_of(*var), data_type)
            }

            AstNode::Assignment { target, .. } => {
                format!("Atribuicao ({})", symbols.name_of(*target))
            }

            AstNode::BinaryOp { op, .. } => {
                format!("Expressao ({})", op_symbol(*op))
            }

            AstNode::Identifier(idx, _) => format!("ID ({})", symbols.name_of(*idx)),
            AstNode::IntLiteral(n, _) => format!("NUM ({n})"),
            AstNode::FloatLiteral(n, _) => format!("FLOAT ({n})"),
            AstNode::StringLiteral(s, _) => format!("STR ({s:?})"),
        }
    }

    fn children(&self) -> Vec<AstChild<'_>> {
        match self {
            AstNode::Program(items) => items.iter().map(AstChild::Node).collect(),

            AstNode::FunctionDef { body, .. } => body.iter().map(AstChild::Node).collect(),

            AstNode::Declaration { init, .. } => {
                init.as_deref().map(AstChild::Node).into_iter().collect()
            }

            AstNode::Assignment { value, .. } => vec![
                AstChild::Leaf("OP_ATRIB (=)".to_string()),
                AstChild::Node(value),
            ],

            AstNode::BinaryOp { left, right, .. } => {
                vec![AstChild::Node(left), AstChild::Node(right)]
            }

            AstNode::Identifier(..)
            | AstNode::IntLiteral(..)
            | AstNode::FloatLiteral(..)
            | AstNode::StringLiteral(..) => vec![],
        }
    }

    pub fn span(&self) -> Span {
        match self {
            AstNode::IntLiteral(_, s)
            | AstNode::FloatLiteral(_, s)
            | AstNode::StringLiteral(_, s)
            | AstNode::Identifier(_, s) => *s,

            AstNode::BinaryOp { span, .. }
            | AstNode::Assignment { span, .. }
            | AstNode::Declaration { span, .. }
            | AstNode::FunctionDef { span, .. } => *span,

            AstNode::Program(items) => match (items.first(), items.last()) {
                (Some(first), Some(last)) => Span {
                    start: first.span().start,
                    end: last.span().end,
                },
                _ => Span { start: 0, end: 0 },
            },
        }
    }
}
