use std::io::{
    self,
    Seek,
    Write,
};

use logos::Span;

use super::{
    input::coerce_condition,
    node::Node,
    node_id::NodeID,
    sb3::{
        qualify_struct_var_name,
        QualifiedName,
        Sb3,
        D,
        S,
    },
};
use crate::{
    ast::{
        Arg,
        Expr,
        Name,
        Proc,
        Stmt,
        Type,
    },
    blocks::Block,
    codegen::mutation::Mutation,
    diagnostic::DiagnosticKind,
    misc::{
        write_comma_io,
        SmolStr,
    },
};

impl<T> Sb3<T>
where T: Write + Seek
{
    pub fn repeat(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        times: &Expr,
        body: &[Stmt],
    ) -> io::Result<()> {
        let times_id = self.id.new_id();
        let body_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "TIMES", times, times_id, false)?;
        self.substack("SUBSTACK", (!body.is_empty()).then_some(body_id))?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        self.expr(s, d, times, times_id, this_id)?;
        self.stmts(s, d, body, body_id, Some(this_id))
    }

    pub fn forever(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        body: &[Stmt],
        _span: &Span,
    ) -> io::Result<()> {
        let body_id = self.id.new_id();
        self.begin_inputs()?;
        self.substack("SUBSTACK", (!body.is_empty()).then_some(body_id))?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        self.stmts(s, d, body, body_id, Some(this_id))
    }

    pub fn branch(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        cond: &Expr,
        if_body: &[Stmt],
        else_body: &[Stmt],
    ) -> io::Result<()> {
        let cond = coerce_condition(cond, s);
        let cond_id = self.id.new_id();
        let if_body_id = self.id.new_id();
        let else_body_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "CONDITION", &cond, cond_id, true)?;
        self.substack("SUBSTACK", (!if_body.is_empty()).then_some(if_body_id))?;
        self.substack("SUBSTACK2", (!else_body.is_empty()).then_some(else_body_id))?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        self.expr(s, d, &cond, cond_id, this_id)?;
        self.stmts(s, d, if_body, if_body_id, Some(this_id))?;
        self.stmts(s, d, else_body, else_body_id, Some(this_id))
    }

    pub fn until(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        cond: &Expr,
        body: &[Stmt],
    ) -> io::Result<()> {
        let cond = coerce_condition(cond, s);
        let cond_id = self.id.new_id();
        let body_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "CONDITION", &cond, cond_id, true)?;
        self.substack("SUBSTACK", (!body.is_empty()).then_some(body_id))?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        self.expr(s, d, &cond, cond_id, this_id)?;
        self.stmts(s, d, body, body_id, Some(this_id))
    }

    pub fn set_var(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        value: &Expr,
        _type: &Type,
        _is_local: &bool,
        _is_cloud: &bool,
    ) -> io::Result<()> {
        let value_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "VALUE", value, value_id, false)?;
        self.end_obj()?; // inputs
        match s.qualify_name(Some(d), name) {
            Some(QualifiedName::Var(qualified_name, _)) => {
                self.single_field_id("VARIABLE", &qualified_name)?
            }
            Some(QualifiedName::List(..)) => {
                d.report(
                    DiagnosticKind::UnrecognizedVariable(name.basename().clone()),
                    &name.span(),
                );
            }
            None => {}
        }
        self.end_obj()?; // node
        self.expr(s, d, value, value_id, this_id)
    }

    pub fn change_var(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        value: &Expr,
    ) -> io::Result<()> {
        let value_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "VALUE", value, value_id, false)?;
        self.end_obj()?; // inputs
        match s.qualify_name(Some(d), name) {
            Some(QualifiedName::Var(qualified_name, _)) => {
                self.single_field_id("VARIABLE", &qualified_name)?
            }
            Some(QualifiedName::List(..)) => {
                d.report(
                    DiagnosticKind::UnrecognizedVariable(name.basename().clone()),
                    &name.span(),
                );
            }
            None => {}
        }
        self.end_obj()?; // node
        self.expr(s, d, value, value_id, this_id)
    }

    pub fn show(&mut self, s: S, d: D, name: &Name) -> io::Result<()> {
        self.begin_inputs()?;
        self.end_obj()?; // inputs
        match s.qualify_name(Some(d), name) {
            Some(QualifiedName::Var(qualified_name, _)) => {
                self.single_field_id("VARIABLE", &qualified_name)?
            }
            Some(QualifiedName::List(qualified_name, _)) => {
                self.single_field_id("LIST", &qualified_name)?
            }
            None => {}
        }
        self.end_obj() // node
    }

    pub fn hide(&mut self, s: S, d: D, name: &Name) -> io::Result<()> {
        self.show(s, d, name)
    }

    pub fn add_to_list(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        value: &Expr,
    ) -> io::Result<()> {
        let value_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "ITEM", value, value_id, false)?;
        self.end_obj()?; // inputs
        match s.qualify_name(Some(d), name) {
            Some(QualifiedName::List(qualified_name, _)) => {
                self.single_field_id("LIST", &qualified_name)?
            }
            Some(QualifiedName::Var(..)) => {
                d.report(
                    DiagnosticKind::UnrecognizedList(name.basename().clone()),
                    &name.span(),
                );
            }
            None => {}
        }
        self.end_obj()?; // node
        self.expr(s, d, value, value_id, this_id)
    }

    pub fn delete_list_index(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        index: &Expr,
    ) -> io::Result<()> {
        let index_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "INDEX", index, index_id, false)?;
        self.end_obj()?; // inputs
        match s.qualify_name(Some(d), name) {
            Some(QualifiedName::List(qualified_name, _)) => {
                self.single_field_id("LIST", &qualified_name)?
            }
            Some(QualifiedName::Var(..)) => {
                d.report(
                    DiagnosticKind::UnrecognizedList(name.basename().clone()),
                    &name.span(),
                );
            }
            None => {}
        }
        self.end_obj()?; // node
        self.expr(s, d, index, index_id, this_id)
    }

    pub fn delete_list(&mut self, s: S, d: D, name: &Name) -> io::Result<()> {
        self.begin_inputs()?;
        self.end_obj()?; // inputs
        match s.qualify_name(Some(d), name) {
            Some(QualifiedName::List(qualified_name, _)) => {
                self.single_field_id("LIST", &qualified_name)?
            }
            Some(QualifiedName::Var(..)) => {
                d.report(
                    DiagnosticKind::UnrecognizedList(name.basename().clone()),
                    &name.span(),
                );
            }
            None => {}
        }
        self.end_obj() // node
    }

    pub fn list_insert(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        index: &Expr,
        value: &Expr,
    ) -> io::Result<()> {
        let index_id = self.id.new_id();
        let value_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "INDEX", index, index_id, false)?;
        self.input(s, d, "ITEM", value, value_id, false)?;
        self.end_obj()?; // inputs
        match s.qualify_name(Some(d), name) {
            Some(QualifiedName::List(qualified_name, _)) => {
                self.single_field_id("LIST", &qualified_name)?
            }
            Some(QualifiedName::Var(..)) => {
                d.report(
                    DiagnosticKind::UnrecognizedList(name.basename().clone()),
                    &name.span(),
                );
            }
            None => {}
        }
        self.end_obj()?; // node
        self.expr(s, d, index, index_id, this_id)?;
        self.expr(s, d, value, value_id, this_id)
    }

    pub fn set_list_index(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        index: &Expr,
        value: &Expr,
    ) -> io::Result<()> {
        self.list_insert(s, d, this_id, name, index, value)
    }

    pub fn block(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        block: &Block,
        span: &Span,
        args: &[Expr],
    ) -> io::Result<()> {
        if block.args().len() != args.len() {
            d.report(
                DiagnosticKind::BlockArgsCountMismatch {
                    block: *block,
                    given: args.len(),
                },
                span,
            )
        }
        self.begin_inputs()?;
        let arg_ids: Vec<NodeID> = (&mut self.id).take(args.len()).collect();
        let menu_id = block.menu().map(|_| self.id.new_id());
        let mut menu_value = None;
        let mut menu_is_default = menu_id.is_some();
        for ((&arg_name, arg_value), &arg_id) in block.args().iter().zip(args).zip(&arg_ids) {
            if block.menu().is_some_and(|menu| menu.input == arg_name) {
                if let Expr::Value {
                    value,
                    span: arg_span,
                } = &arg_value
                {
                    // Validate costume names for switch_costume blocks
                    if let Block::SwitchCostume = block {
                        let costume_name = value.to_string();
                        if !s.sprite.costumes.iter().any(|c| c.name == costume_name) {
                            d.report(DiagnosticKind::InvalidCostumeName(costume_name), arg_span);
                        }
                    }
                    // Validate backdrop names for switch_backdrop blocks
                    if let Block::SwitchBackdrop = block {
                        let backdrop_name = value.to_string();
                        let stage = s.stage.unwrap_or(s.sprite);
                        if !stage.costumes.iter().any(|c| c.name == backdrop_name) {
                            d.report(DiagnosticKind::InvalidBackdropName(backdrop_name), arg_span);
                        }
                    }
                    menu_value = Some(value.clone());
                    continue;
                } else {
                    menu_is_default = false;
                    self.input_with_shadow(s, d, arg_name, arg_value, arg_id, menu_id.unwrap())?;
                }
            } else {
                self.input(s, d, arg_name, arg_value, arg_id, false)?;
            }
        }
        if menu_is_default {
            write_comma_io(&mut self.zip, &mut self.inputs_comma)?;
            write!(
                self,
                r#""{}":[1,{}]"#,
                block.menu().unwrap().input,
                menu_id.unwrap()
            )?;
        }
        self.end_obj()?; // inputs
        if let Some(fields) = block.fields() {
            write!(self, r#","fields":{fields}"#)?;
        }
        if let Block::StopOtherScripts = block {
            self.write_all(
                b",\"mutation\":{\"tagName\":\"mutation\",\"children\": [],\"hasnext\": \"true\"}",
            )?;
        }
        self.end_obj()?; // node
        for (arg, arg_id) in args.iter().zip(arg_ids) {
            self.expr(s, d, arg, arg_id, this_id)?;
        }
        if let Some(menu) = block.menu() {
            self.begin_node(
                Node::new(menu.opcode, menu_id.unwrap())
                    .parent_id(this_id)
                    .shadow(true),
            )?;
            if let Some(menu_value) = menu_value {
                self.single_field(menu.field, &menu_value.to_string())?;
            } else {
                self.single_field(menu.field, menu.default)?;
            }
            self.end_obj()?; // node
        }
        Ok(())
    }

    pub fn proc_call(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &SmolStr,
        span: &Span,
        args: &[Expr],
    ) -> io::Result<()> {
        if name == "log" {
            return self.proc_call_impl(
                &Proc::new(
                    "\u{200b}\u{200b}log\u{200b}\u{200b}".into(),
                    span.clone(),
                    false,
                ),
                &[Arg::new("arg0".into(), span.clone(), Type::Value, None)],
                s,
                d,
                this_id,
                name,
                span,
                args,
                true,
            );
        }
        let Some(proc) = s.sprite.procs.get(name) else {
            if name == "breakpoint" {
                return self.proc_call_impl(
                    &Proc::new(
                        "\u{200b}\u{200b}breakpoint\u{200b}\u{200b}".into(),
                        span.clone(),
                        false,
                    ),
                    &[],
                    s,
                    d,
                    this_id,
                    name,
                    span,
                    args,
                    true,
                );
            }
            if name == "error" {
                return self.proc_call_impl(
                    &Proc::new(
                        "\u{200b}\u{200b}error\u{200b}\u{200b}".into(),
                        span.clone(),
                        false,
                    ),
                    &[Arg::new("arg0".into(), span.clone(), Type::Value, None)],
                    s,
                    d,
                    this_id,
                    name,
                    span,
                    args,
                    true,
                );
            }
            if name == "warn" {
                return self.proc_call_impl(
                    &Proc::new(
                        "\u{200b}\u{200b}warn\u{200b}\u{200b}".into(),
                        span.clone(),
                        false,
                    ),
                    &[Arg::new("arg0".into(), span.clone(), Type::Value, None)],
                    s,
                    d,
                    this_id,
                    name,
                    span,
                    args,
                    true,
                );
            }
            d.report(DiagnosticKind::UnrecognizedProcedure(name.clone()), span);
            return Ok(());
        };
        self.proc_call_impl(
            proc,
            s.sprite
                .proc_args
                .get(&proc.name)
                .map(|v| v.as_slice())
                .unwrap_or_default(),
            s,
            d,
            this_id,
            name,
            span,
            args,
            false,
        )
    }

    fn proc_call_impl(
        &mut self,
        proc: &Proc,
        signature: &[Arg],
        s: S,
        d: D,
        this_id: NodeID,
        name: &SmolStr,
        span: &Span,
        args: &[Expr],
        compact: bool,
    ) -> io::Result<()> {
        if signature.len() != args.len() {
            d.report(
                DiagnosticKind::ProcArgsCountMismatch {
                    proc: name.clone(),
                    given: args.len(),
                },
                span,
            )
        }
        let mut qualified_args: Vec<(SmolStr, NodeID)> = Vec::new();
        let mut qualified_arg_values: Vec<&Expr> = Vec::new();
        self.begin_inputs()?;
        for (arg, arg_value) in signature.iter().zip(args) {
            match &arg.type_ {
                Type::Value => {
                    let arg_id = self.id.new_id();
                    self.input(s, d, &arg.name, arg_value, arg_id, false)?;
                    qualified_args.push((arg.name.clone(), arg_id));
                    qualified_arg_values.push(arg_value);
                }
                Type::Struct {
                    name: type_name,
                    span: type_span,
                } => {
                    let Some(struct_) = s.get_struct(type_name) else {
                        continue;
                    };
                    let struct_literal_fields = match arg_value {
                        Expr::StructLiteral {
                            name: struct_literal_name,
                            span: struct_literal_span,
                            fields: struct_literal_fields,
                        } => {
                            if struct_literal_name != &struct_.name {
                                d.report(
                                    DiagnosticKind::TypeMismatch {
                                        expected: arg.type_.clone(),
                                        given: Type::Struct {
                                            name: struct_literal_name.clone(),
                                            span: struct_literal_span.clone(),
                                        },
                                    },
                                    type_span,
                                );
                                continue;
                            }
                            let mut fields = vec![];
                            for struct_field in &struct_.fields {
                                fields.push(
                                    struct_literal_fields
                                        .iter()
                                        .find(|f| f.name == struct_field.name)
                                        .unwrap(),
                                );
                            }
                            fields
                        }
                        _ => {
                            continue;
                        }
                    };
                    for (field, struct_literal_field) in
                        struct_.fields.iter().zip(struct_literal_fields)
                    {
                        let qualified_arg_name = qualify_struct_var_name(&field.name, &arg.name);
                        let arg_id = self.id.new_id();
                        self.input(
                            s,
                            d,
                            &qualified_arg_name,
                            &struct_literal_field.value,
                            arg_id,
                            false,
                        )?;
                        qualified_args.push((qualified_arg_name, arg_id));
                        qualified_arg_values.push(&struct_literal_field.value);
                    }
                }
            }
        }
        self.end_obj()?; // inputs
        write!(
            self,
            "{}",
            Mutation::call(proc.name.clone(), &qualified_args, proc.warp, compact)
        )?;
        self.end_obj()?; // node
        for (arg, (_, arg_id)) in qualified_arg_values.iter().zip(qualified_args) {
            self.expr(s, d, arg, arg_id, this_id)?;
        }
        Ok(())
    }
}
