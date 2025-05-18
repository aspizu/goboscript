use logos::Span;

use super::{
    value::Value,
    ExceptionResult,
    Interpreter,
};
use crate::{
    ast::Expr,
    blocks::Repr,
    misc::SmolStr,
};

impl Interpreter {
    pub fn run_repr(
        &mut self,
        repr: &Repr,
        _span: &Span,
        args: &[(Option<(SmolStr, Span)>, Expr)],
    ) -> ExceptionResult<Value> {
        let mut arg_values = vec![];
        for (_arg_name, arg_expr) in args {
            let arg_value = self.run_expr(arg_expr)?;
            arg_values.push(arg_value);
        }
        match repr {
            Repr::XPosition => todo!(),
            Repr::YPosition => todo!(),
            Repr::Direction => todo!(),
            Repr::Size => todo!(),
            Repr::CostumeNumber => todo!(),
            Repr::CostumeName => todo!(),
            Repr::BackdropNumber => todo!(),
            Repr::BackdropName => todo!(),
            Repr::Volume => todo!(),
            Repr::DistanceToMousePointer => todo!(),
            Repr::DistanceTo => todo!(),
            Repr::TouchingMousePointer => todo!(),
            Repr::TouchingEdge => todo!(),
            Repr::Touching => todo!(),
            Repr::KeyPressed => todo!(),
            Repr::MouseDown => todo!(),
            Repr::MouseX => todo!(),
            Repr::MouseY => todo!(),
            Repr::Loudness => todo!(),
            Repr::Timer => todo!(),
            Repr::CurrentYear => todo!(),
            Repr::CurrentMonth => todo!(),
            Repr::CurrentDate => todo!(),
            Repr::CurrentDayOfWeek => todo!(),
            Repr::CurrentHour => todo!(),
            Repr::CurrentMinute => todo!(),
            Repr::CurrentSecond => todo!(),
            Repr::DaysSince2000 => todo!(),
            Repr::Username => todo!(),
            Repr::TouchingColor => todo!(),
            Repr::ColorIsTouchingColor => todo!(),
            Repr::Answer => todo!(),
            Repr::Random => random(arg_values),
            Repr::Contains => todo!(),
        }
    }
}

fn random(mut args: Vec<Value>) -> ExceptionResult<Value> {
    let rhs = args.pop().unwrap();
    let lhs = args.pop().unwrap();
    let mut low = lhs.clone().to_number();
    let mut high = rhs.clone().to_number();
    if low > high {
        std::mem::swap(&mut low, &mut high);
    }
    if low == high {
        return Ok(low.into());
    }
    let mut rng = rand::rng();
    let random_value = rand::Rng::random_range(&mut rng, low..=high);
    if lhs.is_integer() && rhs.is_integer() {
        Ok(random_value.floor().into())
    } else {
        Ok(random_value.into())
    }
}
