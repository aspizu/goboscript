use inquire::Text;
use logos::Span;

use super::{
    ExceptionResult,
    Interpreter,
};
use crate::{
    ast::{
        Expr,
        Value,
    },
    blocks::Block,
    throw,
};

impl Interpreter {
    pub fn run_block(&mut self, block: &Block, span: &Span, args: &[Expr]) -> ExceptionResult<()> {
        let mut arg_values = vec![];
        for arg_expr in args {
            let arg_value = self.run_expr(arg_expr)?;
            arg_values.push(arg_value);
        }
        match block {
            Block::Move => todo!(),
            Block::TurnLeft => todo!(),
            Block::TurnRight => todo!(),
            Block::GotoRandomPosition => todo!(),
            Block::GotoMousePointer => todo!(),
            Block::Goto1 => todo!(),
            Block::Goto2 => todo!(),
            Block::Glide3 => todo!(),
            Block::Glide2 => todo!(),
            Block::GlideToRandomPosition => todo!(),
            Block::GlideToMousePointer => todo!(),
            Block::PointInDirection => todo!(),
            Block::PointTowardsMousePointer => todo!(),
            Block::PointTowardsRandomDirection => todo!(),
            Block::PointTowards => todo!(),
            Block::ChangeX => todo!(),
            Block::SetX => todo!(),
            Block::ChangeY => todo!(),
            Block::SetY => todo!(),
            Block::IfOnEdgeBounce => todo!(),
            Block::SetRotationStyleLeftRight => todo!(),
            Block::SetRotationStyleDoNotRotate => todo!(),
            Block::SetRotationStyleAllAround => todo!(),
            Block::Say2 => todo!(),
            Block::Say1 => {
                let [message] = arguments(arg_values, span)?;
                say1(message)
            }
            Block::Think2 => todo!(),
            Block::Think1 => todo!(),
            Block::SwitchCostume => todo!(),
            Block::NextCostume => todo!(),
            Block::SwitchBackdrop => todo!(),
            Block::PreviousBackdrop => todo!(),
            Block::RandomBackdrop => todo!(),
            Block::NextBackdrop => todo!(),
            Block::SetSize => todo!(),
            Block::ChangeSize => todo!(),
            Block::ChangeColorEffect => todo!(),
            Block::ChangeFisheyeEffect => todo!(),
            Block::ChangeWhirlEffect => todo!(),
            Block::ChangePixelateEffect => todo!(),
            Block::ChangeMosaicEffect => todo!(),
            Block::ChangeBrightnessEffect => todo!(),
            Block::ChangeGhostEffect => todo!(),
            Block::SetColorEffect => todo!(),
            Block::SetFisheyeEffect => todo!(),
            Block::SetWhirlEffect => todo!(),
            Block::SetPixelateEffect => todo!(),
            Block::SetMosaicEffect => todo!(),
            Block::SetBrightnessEffect => todo!(),
            Block::SetGhostEffect => todo!(),
            Block::ClearGraphicEffects => todo!(),
            Block::Show => todo!(),
            Block::Hide => todo!(),
            Block::GotoFront => todo!(),
            Block::GotoBack => todo!(),
            Block::GoForward => todo!(),
            Block::GoBackward => todo!(),
            Block::PlaySoundUntilDone => todo!(),
            Block::StartSound => todo!(),
            Block::StopAllSounds => todo!(),
            Block::ChangePitchEffect => todo!(),
            Block::ChangePanEffect => todo!(),
            Block::SetPitchEffect => todo!(),
            Block::SetPanEffect => todo!(),
            Block::ChangeVolume => todo!(),
            Block::SetVolume => todo!(),
            Block::ClearSoundEffects => todo!(),
            Block::Broadcast => todo!(),
            Block::BroadcastAndWait => todo!(),
            Block::Wait => todo!(),
            Block::WaitUntil => todo!(),
            Block::StopAll => todo!(),
            Block::StopThisScript => {
                throw!("__stop_this_script__");
            }
            Block::StopOtherScripts => todo!(),
            Block::DeleteThisClone => todo!(),
            Block::Clone0 => todo!(),
            Block::Clone1 => todo!(),
            Block::Ask => {
                let [question] = arguments(arg_values, span)?;
                ask(self, question)
            }
            Block::SetDragModeDraggable => todo!(),
            Block::SetDragModeNotDraggable => todo!(),
            Block::ResetTimer => todo!(),
            Block::EraseAll => todo!(),
            Block::Stamp => todo!(),
            Block::PenDown => todo!(),
            Block::PenUp => todo!(),
            Block::SetPenColor => todo!(),
            Block::ChangePenSize => todo!(),
            Block::SetPenSize => todo!(),
            Block::SetPenHue => todo!(),
            Block::SetPenSaturation => todo!(),
            Block::SetPenBrightness => todo!(),
            Block::SetPenTransparency => todo!(),
            Block::ChangePenHue => todo!(),
            Block::ChangePenSaturation => todo!(),
            Block::ChangePenBrightness => todo!(),
            Block::ChangePenTransparency => todo!(),
            Block::Rest => todo!(),
            Block::SetTempo => todo!(),
            Block::ChangeTempo => todo!(),
        }
    }
}

pub fn arguments<const N: usize>(
    arg_values: Vec<Value>,
    span: &Span,
) -> ExceptionResult<[Value; N]> {
    if arg_values.len() != N {
        throw!(
            format!("Expected {} arguments, but got {}.", N, arg_values.len()),
            span.clone()
        );
    }

    match arg_values.try_into() {
        Ok(array) => Ok(array),
        Err(_) => {
            throw!(
                "Failed to convert arguments to fixed-size array.",
                span.clone()
            );
        }
    }
}

fn say1(message: Value) -> ExceptionResult<()> {
    println!("{}", message.to_string());
    Ok(())
}

fn ask(this: &mut Interpreter, question: Value) -> ExceptionResult<()> {
    this.answer = Text::new(&question.to_string()).prompt().unwrap().into();
    Ok(())
}
