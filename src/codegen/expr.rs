use std::io::{self, Seek, Write};

use logos::Span;

use super::{
    mutation::Mutation,
    node::Node,
    node_id::NodeID,
    sb3::{qualify_struct_var_name, QualifiedName, Sb3, D, S},
};
use crate::{
    ast::*,
    blocks::{BinOp, Repr, UnOp},
    diagnostic::DiagnosticKind,
    misc::{write_comma_io, SmolStr},
};

impl<T> Sb3<T>
where T: Write + Seek
{
    pub fn arg(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        parent_id: NodeID,
        name: &Name,
    ) -> io::Result<()> {
        let basename = name.basename();

        if !(s
            .proc
            .is_some_and(|proc| proc.args.iter().any(|arg| &arg.name == basename))
            || s.func
                .is_some_and(|func| func.args.iter().any(|arg| &arg.name == basename)))
        {
            d.report(
                DiagnosticKind::UnrecognizedArgument(basename.clone()),
                &name.span(),
            );
            return Ok(());
        }

        let qualified_name = match name.fieldname() {
            Some(fieldname) => qualify_struct_var_name(fieldname, basename),
            None => basename.clone(),
        };
        self.begin_node(
            Node::new("argument_reporter_string_number", this_id).parent_id(parent_id),
        )?;
        self.single_field("VALUE", &qualified_name)?;
        self.end_obj() // node
    }

    pub fn repr(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        parent_id: NodeID,
        repr: &Repr,
        span: &Span,
        args: &[Expr],
    ) -> io::Result<()> {
        if args.len() != repr.args().len() {
            todo!()
        }
        self.begin_node(Node::new(repr.opcode(), this_id).parent_id(parent_id))?;
        let arg_ids: Vec<NodeID> = (&mut self.id).take(args.len()).collect();
        let menu_id = repr.menu().map(|_| self.id.new_id());
        let mut menu_value = None;
        let mut menu_is_default = menu_id.is_some();
        self.begin_inputs()?;
        for ((&arg_name, arg_value), &arg_id) in repr.args().iter().zip(args).zip(&arg_ids) {
            if repr.menu().is_some_and(|menu| menu.input == arg_name) {
                if let Expr::Value { value, span: _ } = arg_value {
                    menu_value = Some(value.clone());
                    continue;
                } else {
                    menu_is_default = false;
                    self.input_with_shadow(s, d, arg_name, arg_value, arg_id, menu_id.unwrap())?;
                }
            } else {
                self.input(s, d, arg_name, arg_value, arg_id)?;
            }
        }
        if menu_is_default {
            write_comma_io(&mut self.zip, &mut self.inputs_comma)?;
            write!(
                self,
                r#""{}":[1,{}]"#,
                repr.menu().unwrap().input,
                menu_id.unwrap()
            )?;
        }
        self.end_obj()?; // inputs
        if let Some(fields) = repr.fields() {
            write!(self, r#","fields":{fields}"#)?;
        }
        self.end_obj()?; // node
        for (arg, arg_id) in args.iter().zip(arg_ids) {
            self.expr(s, d, arg, arg_id, this_id)?;
        }
        if let Some(menu) = repr.menu() {
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

    pub fn un_op(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        parent_id: NodeID,
        op: &UnOp,
        _span: &Span,
        opr: &Expr,
    ) -> io::Result<()> {
        if matches!(op, UnOp::Length) {
            if let Expr::Name(Name::Name { name, .. }) = opr {
                if s.sprite.lists.contains_key(name)
                    || s.stage.is_some_and(|stage| stage.lists.contains_key(name))
                {
                    return self.list_length(s, this_id, parent_id, name);
                }
            }
        }
        let opr_id = self.id.new_id();
        self.begin_node(Node::new(op.opcode(), this_id).parent_id(parent_id))?;
        self.begin_inputs()?;
        self.input(s, d, op.input(), opr, opr_id)?;
        self.end_obj()?; // inputs
        if let Some(fields) = op.fields() {
            write!(self, r#","fields":{fields}"#)?;
        }
        self.end_obj()?; // node
        self.expr(s, d, opr, opr_id, this_id)
    }

    pub fn bin_op(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        parent_id: NodeID,
        op: &BinOp,
        _span: &Span,
        lhs: &Expr,
        rhs: &Expr,
    ) -> io::Result<()> {
        if let BinOp::Of = op {
            if let Expr::Name(name) = lhs {
                if let Some(QualifiedName::List(qualified_name, _)) = s.qualify_name(d, name) {
                    return self.list_index(s, d, this_id, parent_id, &qualified_name, rhs);
                }
            }
        }
        if let BinOp::In = op {
            if let Expr::Name(name) = rhs {
                if let Some(QualifiedName::List(qualified_name, _)) = s.qualify_name(d, name) {
                    return self.list_contains(s, d, this_id, parent_id, &qualified_name, lhs);
                }
            }
        }
        let lhs_id = self.id.new_id();
        let rhs_id = self.id.new_id();
        self.begin_node(Node::new(op.opcode(), this_id).parent_id(parent_id))?;
        self.begin_inputs()?;
        self.input(s, d, op.lhs(), lhs, lhs_id)?;
        self.input(s, d, op.rhs(), rhs, rhs_id)?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        self.expr(s, d, lhs, lhs_id, this_id)?;
        self.expr(s, d, rhs, rhs_id, this_id)
    }

    pub fn list_index(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        parent_id: NodeID,
        name: &str,
        index: &Expr,
    ) -> io::Result<()> {
        let index_id = self.id.new_id();
        self.begin_node(Node::new("data_itemoflist", this_id).parent_id(parent_id))?;
        self.begin_inputs()?;
        self.input(s, d, "INDEX", index, index_id)?;
        self.end_obj()?; // inputs
        self.single_field_id("LIST", name)?;
        self.end_obj()?; // node
        self.expr(s, d, index, index_id, this_id)
    }

    pub fn list_contains(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        parent_id: NodeID,
        name: &str,
        item: &Expr,
    ) -> io::Result<()> {
        let index_id = self.id.new_id();
        self.begin_node(Node::new("data_itemnumoflist", this_id).parent_id(parent_id))?;
        self.begin_inputs()?;
        self.input(s, d, "ITEM", item, index_id)?;
        self.end_obj()?; // inputs
        self.single_field_id("LIST", name)?;
        self.end_obj()?; // node
        self.expr(s, d, item, index_id, this_id)
    }

    fn list_length(
        &mut self,
        s: S,
        this_id: NodeID,
        parent_id: NodeID,
        name: &str,
    ) -> io::Result<()> {
        self.begin_node(Node::new("data_lengthoflist", this_id).parent_id(parent_id))?;
        let list = s.get_list(name).unwrap();
        if let Some((type_name, _type_span)) = list.type_.struct_() {
            let struct_ = s.get_struct(type_name).unwrap();
            let qualified_name = qualify_struct_var_name(&struct_.fields[0].name, name);
            self.single_field_id("LIST", &qualified_name)?;
        } else {
            self.single_field_id("LIST", name)?;
        }
        self.end_obj() // node
    }

    pub fn func_call(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        name: &SmolStr,
        span: &Span,
        args: &[Expr],
    ) -> io::Result<()> {
        let Some(func) = s.sprite.funcs.get(name) else {
            d.report(DiagnosticKind::UnrecognizedFunction(name.clone()), span);
            return Ok(());
        };
        if func.args.len() != args.len() {
            d.report(
                DiagnosticKind::FuncArgsCountMismatch {
                    func: name.clone(),
                    given: args.len(),
                },
                span,
            )
        }
        let mut qualified_args: Vec<(SmolStr, NodeID)> = vec![];
        let mut qualified_arg_values: Vec<Expr> = vec![];
        self.begin_inputs()?;
        for (arg, kwarg) in func.args.iter().zip(args) {
            match &arg.type_ {
                Type::Value => {
                    let arg_id = self.id.new_id();
                    self.input(s, d, &arg.name, kwarg, arg_id)?;
                    qualified_args.push((arg.name.clone(), arg_id));
                    qualified_arg_values.push(kwarg.clone());
                }
                Type::Struct {
                    name: type_name,
                    span: type_span,
                } => {
                    let Some(struct_) = s.sprite.structs.get(type_name) else {
                        continue;
                    };
                    let arg_value = kwarg;
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
                            &struct_literal_field.value,
                            arg_id,
                        )?;
                        qualified_args.push((qualified_arg_name, arg_id));
                        qualified_arg_values.push(struct_literal_field.value.as_ref().clone());
                    }
                }
            }
        }
        self.end_obj()?; // inputs
        write!(
            self,
            "{}",
            Mutation::call(func.name.clone(), &qualified_args, true)
        )?;
        self.end_obj()?; // node
        for (arg, (_, arg_id)) in qualified_arg_values.iter().zip(qualified_args) {
            self.expr(s, d, arg, arg_id, this_id)?;
        }
        Ok(())
    }

    pub fn expr_dot(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        parent_id: NodeID,
        lhs: &Expr,
        rhs: &SmolStr,
        rhs_span: Span,
    ) -> io::Result<()> {
        if let Expr::Name(name) = lhs {
            if let Some(enum_) = s.get_enum(name.basename()) {
                return Ok(());
            }
        }
        panic!("attempted to codegen Expr::Dot lhs = {lhs:#?}, rhs = {rhs:#?}")
    }
}
