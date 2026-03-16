use std::fmt::{
    self,
    Display,
};

use super::node_id::NodeID;
use crate::misc::{
    write_comma_fmt,
    SmolStr,
};

pub struct Mutation<'a> {
    name: SmolStr,
    args: &'a Vec<(SmolStr, NodeID)>,
    warp: bool,
    is_call: bool,
}

impl<'a> Mutation<'a> {
    pub fn prototype(
        name: SmolStr,
        args: &'a Vec<(SmolStr, NodeID)>,
        warp: bool,
    ) -> Self {
        Self {
            name,
            args,
            warp,
            is_call: false,
        }
    }

    pub fn call(
        name: SmolStr,
        args: &'a Vec<(SmolStr, NodeID)>,
        warp: bool,
    ) -> Self {
        Self {
            name,
            args,
            warp,
            is_call: true,
        }
    }
}

impl Display for Mutation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#","mutation":{{"tagName":"mutation","children":[]"#)?;
        write!(f, r#","warp":"{}""#, self.warp)?;
        // For selector-style procs, proc.name is already the full proccode
        // (e.g. "foo %s bar %n").  For old-style procs (plain names), append
        // " %s" per arg to reconstruct the Scratch proccode.
        let is_selector = self.name.contains("%s")
            || self.name.contains("%n")
            || self.name.contains("%b");
        let mut proccode = self.name.to_string();
        if !is_selector {
            for _ in self.args {
                proccode.push_str(" %s");
            }
        }
        // Use serde_json to correctly escape any special characters in the proccode.
        write!(f, r#","proccode":{}"#, serde_json::to_string(&proccode).unwrap())?;
        write!(f, r#","argumentids":"["#)?;
        let mut comma = false;
        for (arg_name, _) in self.args {
            write_comma_fmt(&mut *f, &mut comma)?;
            write!(f, r#"\"{}\""#, arg_name)?;
        }
        write!(f, "]\"")?;
        if !self.is_call {
            write!(f, r#","argumentnames":"["#)?;
            let mut comma = false;
            for (arg_name, _) in self.args {
                write_comma_fmt(&mut *f, &mut comma)?;
                write!(f, r#"\"{}\""#, arg_name)?;
            }
            write!(f, "]\"")?;
            write!(f, r#","argumentdefaults":"["#)?;
            let mut comma = false;
            for _ in self.args {
                write_comma_fmt(&mut *f, &mut comma)?;
                write!(f, r#"\"\""#,)?;
            }
            write!(f, "]\"")?;
        }
        write!(f, "}}") // mutation
    }
}
