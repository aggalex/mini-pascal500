use std::sync::mpsc::SendError;
use crate::ast::expression::{ExBox, Expression};
use crate::ast::program::Program;
use crate::ast::types::Type;
use crate::error::semantic_error::SemanticErrorKind;

macro_rules! validate {
    ($s:expr, $p:expr) => {
        if !$s.is_valid($p) {
            return Type::Invalid;
        }
    }
}

trait IntoSemanticErrorKind {
    type Out;
    fn into_kind(self) -> Self::Out;
}

impl<Err: Into<SemanticErrorKind>> IntoSemanticErrorKind for Vec<Err> {
    type Out = Vec<SemanticErrorKind>;
    #[inline]
    fn into_kind(self) -> Self::Out {
        self.into_iter()
            .map(|err| err.into())
            .collect()
    }
}

impl<Ok, Err: Into<SemanticErrorKind>> IntoSemanticErrorKind for Result<Ok, Err> {
    type Out = Result<Ok, SemanticErrorKind>;
    #[inline]
    fn into_kind(self) -> Self::Out {
        self.map_err(Into::into)
    }
}

fn validate_primitives<Err: Into<SemanticErrorKind>>(
    left: &impl Expression<Error = Err>,
    right: &impl Expression<Error = Err>,
    program: &Program
) -> Vec<SemanticErrorKind> {
    let mut out = left.validate(program).into_kind();
    out.append(&mut right.validate(program).into_kind());
    let left_type = left.get_type(program);
    let right_type = left.get_type(program);
    if !(left.get_type(program).is_primitive() && right.get_type(program).is_primitive()) {
        out.push(SemanticErrorKind::TypeError {
            expected: Type::PRIMITIVE.to_vec(),
            got: if left_type.is_primitive() { right_type } else { left_type }
        })
    }
    out
}

fn arithmetic_operation(left: &impl Expression,
                        right: &impl Expression,
                        program: &Program) -> Type {
    let left_type = left.get_type(program);
    let right_type = right.get_type(program);
    if left_type == right_type {
        left_type // can be either integer or real
    } else {
        Type::Real
    }
}

#[derive(Debug)]
pub struct In<Left: Expression = ExBox, Right: Expression = ExBox> {
    pub sample: Left,
    pub set: Right
}

impl<Err: Into<SemanticErrorKind>,
    EL: Expression<Error = Err>,
    ER: Expression<Error = Err>> Expression for In<EL, ER> {
    type Error = SemanticErrorKind;

    fn get_type(&self, program: &Program) -> Type {
        validate!(self, program);
        Type::Boolean
    }

    fn validate(&self, program: &Program) -> Vec<SemanticErrorKind> {
        let mut out = self.sample.validate(program).into_kind();
        if out.len() > 0 {
            return out;
        }
        out.append(&mut self.set.validate(program).into_kind());
        if out.len() > 0 {
            return out;
        }
        let set_type = self.set.get_type(program);
        if let Type::SetOf(ty) = self.set.get_type(program) {
            let sample_type = self.sample.get_type(program);
            if sample_type != *ty {
                out.push(SemanticErrorKind::TypeError {
                    expected: vec![*ty.clone()],
                    got: sample_type
                });
            }
        } else {
            out.push(SemanticErrorKind::TypeError {
                expected: vec![Type::SetOf(Box::new(Type::Invalid))],
                got: set_type
            });
        }
        out
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Err(SemanticErrorKind::InvalidLimit)
    }
}

#[derive(Debug)]
pub enum CompOp {
    Bg,
    Lt,
    Bge,
    Lte,
    Neq,
    Eq
}

#[derive(Debug)]
pub struct Comparison<Left: Expression = ExBox,
                      Right: Expression = ExBox> {
    pub left: Left,
    pub right: Right,
    pub op: CompOp
}

impl<Err: Into<SemanticErrorKind>,
    Left: Expression<Error = Err>,
    Right: Expression<Error = Err>> Expression for Comparison<Left, Right> {
    type Error = SemanticErrorKind;

    fn get_type(&self, program: &Program) -> Type {
        validate!(self, program);
        Type::Boolean
    }

    fn validate(&self, program: &Program) -> Vec<SemanticErrorKind> {
        validate_primitives(&self.left, &self.right, program)
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        let left = self.left.as_number(program).into_kind()?;
        let right = self.right.as_number(program).into_kind()?;
        Ok(if match self.op {
            CompOp::Bg => left > right,
            CompOp::Lt => left < right,
            CompOp::Bge => left >= right,
            CompOp::Lte => left <= right,
            CompOp::Neq => left != right,
            CompOp::Eq => left == right
        } { 1 } else { 0 })
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum SumOp {
    Add,
    Sub,
}

impl Into<bool> for SumOp {
    fn into(self) -> bool {
        self == SumOp::Add
    }
}

#[derive(Debug)]
pub struct Sum<Left: Expression = ExBox,
               Right: Expression = ExBox> {
    pub left: Left,
    pub right: Right,
    pub op: SumOp
}

impl<Err: Into<SemanticErrorKind>,
    Left: Expression<Error = Err>,
    Right: Expression<Error = Err>> Expression for Sum<Left, Right> {
    type Error = SemanticErrorKind;

    fn get_type(&self, program: &Program) -> Type {
        validate!(self, program);
        arithmetic_operation(&self.left, &self.right, program)
    }

    fn validate(&self, program: &Program) -> Vec<SemanticErrorKind> {
        validate_primitives(&self.left, &self.right, program)
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        let left = self.left.as_number(program).into_kind()?;
        let right = self.right.as_number(program).into_kind()?;
        Ok(match self.op {
            SumOp::Add => left + right,
            SumOp::Sub => left - right
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ProdOp {
    Mul,
    RDiv,
    Div,
    Mod
}

#[derive(Debug)]
pub struct Product<Left: Expression = ExBox,
                   Right: Expression = ExBox> {
    pub left: Left,
    pub right: Right,
    pub op: ProdOp
}

impl<Err: Into<SemanticErrorKind>,
    L: Expression<Error = Err>,
    R: Expression<Error = Err>> Expression for Product<L, R> {
    type Error = SemanticErrorKind;

    fn get_type(&self, program: &Program) -> Type {
        validate!(self, program);
        if self.op == ProdOp::RDiv {
            return Type::Real
        }
        arithmetic_operation(&self.left, &self.right, program)
    }

    fn validate(&self, program: &Program) -> Vec<SemanticErrorKind> {
        let mut out = validate_primitives(&self.left, &self.right, program);
        if out.len() > 0 {
            return out;
        }
        let left_type = self.left.get_type(program);
        let right_type = self.right.get_type(program);
        if self.op == ProdOp::Div && [&left_type, &right_type].contains(&&Type::Real) {
            const ALLOWED: &[Type] = &[Type::Integer, Type::Char, Type::Boolean];
            out.push(SemanticErrorKind::TypeError {
                expected: ALLOWED.to_vec(),
                got: if ALLOWED.contains(&left_type) { right_type } else { left_type }
            });
        }
        out
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        let left = self.left.as_number(program).into_kind()?;
        let right = self.right.as_number(program).into_kind()?;
        Ok(match self.op {
            ProdOp::Mul => left * right,
            ProdOp::RDiv => return Err(SemanticErrorKind::InvalidLimit),
            ProdOp::Div => left / right,
            ProdOp::Mod => left % right
        })
    }
}

#[derive(Debug)]
pub struct Not<E: Expression = ExBox>(E);

impl<Err: Into<SemanticErrorKind>,
    Exp: Expression<Error = Err>> Expression for Not<Exp> {
    type Error = SemanticErrorKind;

    fn get_type(&self, program: &Program) -> Type {
        validate!(self, program);
        Type::Boolean
    }

    fn validate(&self, program: &Program) -> Vec<SemanticErrorKind> {
        let mut out = self.0.validate(program).into_kind();
        if out.len() > 0 {
            return out
        }
        let ty = self.0.get_type(program);
        if ty != Type::Boolean {
            out.push(SemanticErrorKind::TypeError {
                expected: vec![Type::Boolean],
                got: ty
            })
        }
        out
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Ok((self.0.as_number(program).into_kind()? == 0) as i64)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum LogicOp {
    And,
    Or,
}

#[derive(Debug)]
pub struct Logic<Left: Expression = ExBox,
                 Right: Expression = ExBox> {
    pub left: Left,
    pub right: Right,
    pub op: LogicOp
}

impl<Err: Into<SemanticErrorKind>,
    L: Expression<Error = Err>,
    R: Expression<Error = Err>> Expression for Logic<L, R> {
    type Error = SemanticErrorKind;

    fn get_type(&self, program: &Program) -> Type {
        validate!(self, program);
        Type::Boolean
    }

    fn is_valid(&self, program: &Program) -> bool {
        self.right.get_type(program) == Type::Boolean
            && self.left.get_type(program) == Type::Boolean
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        let left = self.left.as_number(program).into_kind()? > 0;
        let right = self.right.as_number(program).into_kind()? > 0;
        Ok((match self.op {
            LogicOp::And => left && right,
            LogicOp::Or => left || right
        }) as i64)
    }
}

#[derive(Debug)]
pub struct Call<E: Expression = ExBox> {
    pub name: String,
    pub args: Vec<E>
}

impl<Err: Into<SemanticErrorKind>,
    Exp: Expression<Error = Err>> Expression for Call<Exp> {
    type Error = SemanticErrorKind;

    fn get_type(&self, _: &Program) -> Type {
        todo!()
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Err(SemanticErrorKind::InvalidLimit)
    }
}

impl<Err: Into<SemanticErrorKind>,
    E: Expression<Error = Err>> Expression for Vec<E> {
    type Error = SemanticErrorKind;

    fn get_type(&self, program: &Program) -> Type {
        Type::SetOf(Box::new(self[0].get_type(program)))
    }

    fn validate(&self, program: &Program) -> Vec<SemanticErrorKind> {
        let ty = self[0].get_type(program);
        let mut out = self.into_iter()
            .flat_map(|el| el.validate(program).into_kind())
            .collect::<Vec<SemanticErrorKind>>();
        if out.len() > 0 {
            return out;
        }
        out.append(&mut self.into_iter()
            .filter_map(|obj| {
                let ety = obj.get_type(program);
                if ety != ty {
                    Some(ety)
                } else {
                    None
                }
            })
            .map(|got| SemanticErrorKind::TypeError {
                expected: vec![ty.clone()],
                got
            })
            .collect());
        out
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Err(SemanticErrorKind::InvalidLimit)
    }
}