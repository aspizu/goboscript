use std::io::{self, Seek, Write};

use logos::Span;
use smol_str::SmolStr;

use super::{
    node::Node,
    node_id::NodeID,
    sb3::{qualify_struct_var_name, QualifiedName, Sb3, D, S},
};
use crate::{
    ast::{Expr, Kwarg, Name, Stmt, Type},
    blocks::Block,
    codegen::mutation::Mutation,
    diagnostic::DiagnosticKind,
    misc::{write_comma_io, Rrc},
};

impl<T> Sb3<T>
where T: Write + Seek
{
    pub fn repeat(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        times: &Rrc<Expr>,
        body: &[Stmt],
    ) -> io::Result<()> {
        let times_id = self.id.new_id();
        let body_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "TIMES", &times.borrow(), times_id)?;
        self.substack("SUBSTACK", (!body.is_empty()).then_some(body_id))?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        self.expr(s, d, &times.borrow(), times_id, this_id)?;
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
        cond: &Rrc<Expr>,
        if_body: &[Stmt],
        else_body: &[Stmt],
    ) -> io::Result<()> {
        let cond_id = self.id.new_id();
        let if_body_id = self.id.new_id();
        let else_body_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "CONDITION", &cond.borrow(), cond_id)?;
        self.substack("SUBSTACK", (!if_body.is_empty()).then_some(if_body_id))?;
        self.substack("SUBSTACK2", (!else_body.is_empty()).then_some(else_body_id))?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        self.expr(s, d, &cond.borrow(), cond_id, this_id)?;
        self.stmts(s, d, if_body, if_body_id, Some(this_id))?;
        self.stmts(s, d, else_body, else_body_id, Some(this_id))
    }

    pub fn until(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        cond: &Rrc<Expr>,
        body: &[Stmt],
    ) -> io::Result<()> {
        let cond_id = self.id.new_id();
        let body_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "CONDITION", &cond.borrow(), cond_id)?;
        self.substack("SUBSTACK", (!body.is_empty()).then_some(body_id))?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        self.expr(s, d, &cond.borrow(), cond_id, this_id)?;
        self.stmts(s, d, body, body_id, Some(this_id))
    }

    pub fn set_var(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        value: &Rrc<Expr>,
        _type: &Type,
        _is_local: &bool,
    ) -> io::Result<()> {
        let value_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "VALUE", &value.borrow(), value_id)?;
        self.end_obj()?; // inputs
        match s.qualify_name(d, name) {
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
        self.expr(s, d, &value.borrow(), value_id, this_id)
    }

    pub fn change_var(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        value: &Rrc<Expr>,
    ) -> io::Result<()> {
        let value_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "VALUE", &value.borrow(), value_id)?;
        self.end_obj()?; // inputs
        match s.qualify_name(d, name) {
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
        self.expr(s, d, &value.borrow(), value_id, this_id)
    }

    pub fn show(&mut self, s: S, d: D, name: &Name) -> io::Result<()> {
        self.begin_inputs()?;
        self.end_obj()?; // inputs
        match s.qualify_name(d, name) {
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
        value: &Rrc<Expr>,
    ) -> io::Result<()> {
        let value_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "ITEM", &value.borrow(), value_id)?;
        self.end_obj()?; // inputs
        match s.qualify_name(d, name) {
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
        self.expr(s, d, &value.borrow(), value_id, this_id)
    }

    pub fn delete_list_index(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        index: &Rrc<Expr>,
    ) -> io::Result<()> {
        let index_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "INDEX", &index.borrow(), index_id)?;
        self.end_obj()?; // inputs
        match s.qualify_name(d, name) {
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
        self.expr(s, d, &index.borrow(), index_id, this_id)
    }

    pub fn delete_list(&mut self, s: S, d: D, name: &Name) -> io::Result<()> {
        self.begin_inputs()?;
        self.end_obj()?; // inputs
        match s.qualify_name(d, name) {
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
        index: &Rrc<Expr>,
        value: &Rrc<Expr>,
    ) -> io::Result<()> {
        let index_id = self.id.new_id();
        let value_id = self.id.new_id();
        self.begin_inputs()?;
        self.input(s, d, "INDEX", &index.borrow(), index_id)?;
        self.input(s, d, "ITEM", &value.borrow(), value_id)?;
        self.end_obj()?; // inputs
        match s.qualify_name(d, name) {
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
        self.expr(s, d, &index.borrow(), index_id, this_id)?;
        self.expr(s, d, &value.borrow(), value_id, this_id)
    }

    pub fn set_list_index(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &Name,
        index: &Rrc<Expr>,
        value: &Rrc<Expr>,
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
        args: &Vec<Kwarg>,
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
                if let Expr::Value { value, span: _ } = &*arg_value.value.borrow() {
                    menu_value = Some(value.clone());
                    continue;
                } else {
                    menu_is_default = false;
                    self.input_with_shadow(
                        s,
                        d,
                        arg_name,
                        &arg_value.value.borrow(),
                        arg_id,
                        menu_id.unwrap(),
                    )?;
                }
            } else {
                self.input(s, d, arg_name, &arg_value.value.borrow(), arg_id)?;
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
        self.end_obj()?; // node
        for (kwarg, arg_id) in args.iter().zip(arg_ids) {
            self.expr(s, d, &kwarg.value.borrow(), arg_id, this_id)?;
        }
        if let Some(menu) = block.menu() {
            self.begin_node(
                Node::new(menu.opcode, menu_id.unwrap())
                    .parent_id(this_id)
                    .shadow(true),
            )?;
            if let Some(menu_value) = menu_value {
                self.single_field(menu.input, &menu_value.to_string())?;
            } else {
                self.single_field(menu.input, menu.default)?;
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
        args: &Vec<Kwarg>,
    ) -> io::Result<()> {
        let Some(proc) = s.sprite.procs.get(name) else {
            d.report(DiagnosticKind::UnrecognizedProcedure(name.clone()), span);
            return Ok(());
        };
        if proc.args.len() != args.len() {
            d.report(
                DiagnosticKind::ProcArgsCountMismatch {
                    proc: name.clone(),
                    given: args.len(),
                },
                span,
            )
        }
        let mut qualified_args: Vec<(SmolStr, NodeID)> = Vec::new();
        let mut qualified_arg_values: Vec<Rrc<Expr>> = Vec::new();
        self.begin_inputs()?;
        for (arg, kwarg) in proc.args.iter().zip(args) {
            match &arg.type_ {
                Type::Value => {
                    let arg_id = self.id.new_id();
                    self.input(s, d, &arg.name, &kwarg.value.borrow(), arg_id)?;
                    qualified_args.push((arg.name.clone(), arg_id));
                    qualified_arg_values.push(kwarg.value.clone());
                }
                Type::Struct {
                    name: type_name,
                    span: type_span,
                } => {
                    let Some(struct_) = s.sprite.structs.get(type_name) else {
                        continue;
                    };
                    let arg_value = &*kwarg.value.borrow();
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
                            if struct_literal_fields.len() != struct_.fields.len() {
                                panic!()
                            }
                            for (struct_field, struct_literal_field) in
                                struct_.fields.iter().zip(struct_literal_fields)
                            {
                                if struct_field.name != struct_literal_field.name {
                                    panic!()
                                }
                            }
                            struct_literal_fields
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
                            &struct_literal_field.value.borrow(),
                            arg_id,
                        )?;
                        qualified_args.push((qualified_arg_name, arg_id));
                        qualified_arg_values.push(struct_literal_field.value.clone());
                    }
                }
            }
        }
        self.end_obj()?; // inputs
        write!(
            self,
            "{}",
            Mutation::call(proc.name.clone(), &qualified_args, proc.warp)
        )?;
        self.end_obj()?; // node
        for (arg, (_, arg_id)) in qualified_arg_values.iter().zip(qualified_args) {
            self.expr(s, d, &arg.borrow(), arg_id, this_id)?;
        }
        Ok(())
    }
}
