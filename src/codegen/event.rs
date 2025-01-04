use std::io::{self, Seek, Write};

use logos::Span;

use super::{
    node_id::NodeID,
    sb3::{Sb3, D, S},
};
use crate::{ast::Expr, misc::SmolStr};

impl<T> Sb3<T>
where T: Write + Seek
{
    pub fn on(&mut self, event: &SmolStr) -> io::Result<()> {
        self.single_field("BROADCAST_OPTION", event)?;
        self.end_obj() // node
    }

    pub fn on_flag(&mut self) -> io::Result<()> {
        self.end_obj() // node
    }

    pub fn on_key(
        &mut self,
        _s: S,
        _d: D,
        _this_id: NodeID,
        key: &SmolStr,
        _span: &Span,
    ) -> io::Result<()> {
        self.single_field("KEY_OPTION", key)?;
        self.end_obj() // node
    }

    pub fn on_click(&mut self, _s: S, _d: D, _this_id: NodeID) -> io::Result<()> {
        self.end_obj() // node
    }

    pub fn on_backdrop(
        &mut self,
        _s: S,
        _d: D,
        _this_id: NodeID,
        backdrop: &SmolStr,
        _span: &Span,
    ) -> io::Result<()> {
        self.single_field("BACKDROP_OPTION", backdrop)?;
        self.end_obj() // node
    }

    pub fn on_loudness_gt(&mut self, s: S, d: D, this_id: NodeID, value: &Expr) -> io::Result<()> {
        self.begin_inputs()?;
        self.input(s, d, "VALUE", value, this_id)?;
        self.end_obj()?; // inputs
        self.single_field("WHENGREATERTHANMENU", "LOUDNESS")?;
        self.end_obj()?; // node
        self.expr(s, d, value, this_id, this_id)
    }

    pub fn on_timer_gt(&mut self, s: S, d: D, this_id: NodeID, value: &Expr) -> io::Result<()> {
        self.begin_inputs()?;
        self.input(s, d, "VALUE", value, this_id)?;
        self.end_obj()?; // inputs
        self.single_field("WHENGREATERTHANMENU", "TIMER")?;
        self.end_obj()?; // node
        self.expr(s, d, value, this_id, this_id)
    }

    pub fn on_clone(&mut self, _s: S, _d: D, _this_id: NodeID) -> io::Result<()> {
        self.end_obj() // node
    }
}
