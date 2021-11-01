use crate::ast::expression::{Error, Expression};
use crate::ast::types::Type;

macro_rules! validate {
    ($s:expr) => {
        if !$s.is_valid() {
            return Type::Invalid;
        }
    }
}

fn validate_primitives(left: &impl Expression, right: &impl Expression) -> Vec<Error> {
    let mut out = left.validate();
    out.append(&mut right.validate());
    if !(left.get_type().is_primitive() && right.get_type().is_primitive()) {
        out.push(Error::InvalidOperands)
    }
    out
}

fn arithmetic_operation(left: &impl Expression, right: &impl Expression) -> Type {
    let left_type = left.get_type();
    let right_type = right.get_type();
    if left_type == right_type {
        left_type // can be either integer or real
    } else {
        Type::Real
    }
}

#[derive(Debug)]
pub struct In<Left: Expression = Box<dyn Expression>, Right: Expression = Box<dyn Expression>> {
    pub sample: Left,
    pub set: Right
}

impl<EL: Expression, ER: Expression> Expression for In<EL, ER> {
    fn get_type(&self) -> Type {
        validate!(self);
        Type::Boolean
    }

    fn validate(&self) -> Vec<Error> {
        let mut out = self.sample.validate();
        out.append(&mut self.set.validate());
        if let Type::SetOf(ty) = self.set.get_type() {
            if self.sample.get_type() != *ty {
                out.push(Error::InvalidOperands);
            }
        } else {
            out.push(Error::InvalidOperands);
        }
        out
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Err(self)
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
pub struct Comparison<Left: Expression = Box<dyn Expression>,
                      Right: Expression = Box<dyn Expression>> {
    pub left: Left,
    pub right: Right,
    pub op: CompOp
}

impl<Left: Expression, Right: Expression> Expression for Comparison<Left, Right> {
    fn get_type(&self) -> Type {
        validate!(self);
        Type::Boolean
    }

    fn validate(&self) -> Vec<Error> {
        validate_primitives(&self.left, &self.right)
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        let left = self.left.as_number()?;
        let right = self.right.as_number()?;
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
pub struct Sum<Left: Expression = Box<dyn Expression>,
               Right: Expression = Box<dyn Expression>> {
    pub left: Left,
    pub right: Right,
    pub op: SumOp
}

impl<Left: Expression, Right: Expression> Expression for Sum<Left, Right> {
    fn get_type(&self) -> Type {
        validate!(self);
        arithmetic_operation(&self.left, &self.right)
    }

    fn validate(&self) -> Vec<Error> {
        validate_primitives(&self.left, &self.right)
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        let left = self.left.as_number()?;
        let right = self.right.as_number()?;
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
pub struct Product<Left: Expression = Box<dyn Expression>,
                   Right: Expression = Box<dyn Expression>> {
    pub left: Left,
    pub right: Right,
    pub op: ProdOp
}

impl<L: Expression, R: Expression> Expression for Product<L, R> {
    fn get_type(&self) -> Type {
        validate!(self);
        if self.op == ProdOp::RDiv {
            return Type::Real
        }
        arithmetic_operation(&self.left, &self.right)
    }

    fn validate(&self) -> Vec<Error> {
        let mut out = validate_primitives(&self.left, &self.right);
        if [self.left.get_type(), self.right.get_type()].contains(&Type::Real) {
            out.push(Error::InvalidOperands);
        }
        out
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        let left = self.left.as_number()?;
        let right = self.right.as_number()?;
        Ok(match self.op {
            ProdOp::Mul => left * right,
            ProdOp::RDiv => return Err(self),
            ProdOp::Div => left / right,
            ProdOp::Mod => left % right
        })
    }
}

#[derive(Debug)]
pub struct Not<E: Expression = Box<dyn Expression>>(E);

impl<E: Expression> Expression for Not<E> {
    fn get_type(&self) -> Type {
        validate!(self);
        Type::Boolean
    }

    fn validate(&self) -> Vec<Error> {
        if self.0.get_type() != Type::Boolean {
            vec![Error::InvalidOperands]
        } else {
            vec![]
        }
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Ok((self.0.as_number()? == 0) as i64)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum LogicOp {
    And,
    Or,
}

#[derive(Debug)]
pub struct Logic<Left: Expression = Box<dyn Expression>,
                 Right: Expression = Box<dyn Expression>> {
    pub left: Left,
    pub right: Right,
    pub op: LogicOp
}

impl<L: Expression, R: Expression> Expression for Logic<L, R> {
    fn get_type(&self) -> Type {
        validate!(self);
        Type::Boolean
    }

    fn is_valid(&self) -> bool {
        self.right.get_type() == Type::Boolean
            && self.left.get_type() == Type::Boolean
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        let left = self.left.as_number()? > 0;
        let right = self.right.as_number()? > 0;
        Ok((match self.op {
            LogicOp::And => left && right,
            LogicOp::Or => left || right
        }) as i64)
    }
}

#[derive(Debug)]
pub struct Call<E: Expression = Box<dyn Expression>> {
    pub name: String,
    pub args: Vec<E>
}

impl<E: Expression> Expression for Call<E> {
    fn get_type(&self) -> Type {
        todo!()
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Err(self)
    }
}

impl<E: Expression> Expression for Vec<E> {
    fn get_type(&self) -> Type {
        Type::SetOf(Box::new(self[0].get_type()))
    }

    fn validate(&self) -> Vec<Error> {
        let ty = self[0].get_type();
        self.into_iter()
            .filter(|obj| obj.get_type() != ty)
            .map(|_| Error::InvalidOperands)
            .collect()
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Err(self)
    }
}