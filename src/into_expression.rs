use nu_protocol::{
    ast::{Expr, Expression, RecordItem},
    Span, Value,
};
use nu_protocol::SpanId;
use crate::{IntoValue, NewEmpty};
use nu_protocol::ast::ListItem;
pub trait IntoExpression {
    fn into_expression(self) -> Expression;
}

pub trait ValueIntoExpression {
    fn into_expression(self) -> Expression;
    fn into_expr(self) -> Expr;
}

impl<V: IntoValue> IntoExpression for V {
    #[inline]
    fn into_expression(self) -> Expression {
        self.into_value().into_expression()
    }
}

impl ValueIntoExpression for Value {
    fn into_expression(self) -> Expression {
        let ty = self.get_type();

        Expression {
            expr: self.into_expr(),
            span: Span::empty(),
            span_id: SpanId::new(0),
            ty,
            custom_completion: None,
        }
    }

    fn into_expr(self) -> Expr {
        match self {
            Value::Bool { val, .. } => Expr::Bool(val),
            Value::Int { val, .. } => Expr::Int(val),
            Value::Float { val, .. } => Expr::Float(val),
            Value::Filesize { val, .. } => Expr::Int(val.get()),
            Value::Duration { val, .. } => Expr::Int(val),
            Value::Date { val, .. } => Expr::DateTime(val),
            Value::String { val, .. } => Expr::String(val),
            Value::Record { val, .. } => {
                let entries = val
                    .iter()  // Borrow the record instead of taking ownership
                    .map(|(col, val)| {
                        let col_expr = col.as_str().into_expression();  // Borrow col and convert
                        let val_expr = val.clone().into_expression();  // Convert `val` into an expression, which should be a Value
            
                        RecordItem::Pair(col_expr, val_expr)
                    })
                    .collect();
            
                Expr::Record(entries)
            }
            
            
            
            Value::List { vals, .. } => {
                let vals = vals
                    .into_iter()
                    .map(|v| ListItem::Item(v.into_expression()))
                    .collect();
                Expr::List(vals)
            }
            Value::Nothing { .. } => Expr::Nothing,
            Value::Error { error, .. } => Expr::String(error.to_string()),
            Value::Binary { val, .. } => Expr::Binary(val),
            Value::CellPath { val, .. } => Expr::CellPath(val),
            _ => Expr::Nothing,
        }
    }
}
