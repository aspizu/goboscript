use std::io::{
    self,
    Seek,
    Write,
};

use logos::Span;
use serde_json::json;

use super::{
    input::{
        coerce_condition,
        is_expr_boolean,
    },
    mutation::Mutation,
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
    ast::*,
    blocks::{
        BinOp,
        Repr,
        UnOp,
    },
    diagnostic::DiagnosticKind,
    misc::{
        write_comma_io,
        SmolStr,
    },
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
            .map(|proc| &s.sprite.proc_args[&proc.name])
            .is_some_and(|args| args.iter().any(|arg| &arg.name == basename))
            || s.func
                .map(|func| &s.sprite.func_args[&func.name])
                .is_some_and(|args| args.iter().any(|arg| &arg.name == basename)))
        {
            if basename == "tw_is_compiled" {
                return self.arg_impl(this_id, parent_id, "is compiled?", true);
            }
            if basename == "tw_is_turbowarp" {
                return self.arg_impl(this_id, parent_id, "is TurboWarp?", true);
            }
            if basename == "tw_is_forkphorus" {
                return self.arg_impl(this_id, parent_id, "is forkphorus?", true);
            }
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

        self.arg_impl(this_id, parent_id, &qualified_name, false)
    }

    fn arg_impl(
        &mut self,
        this_id: NodeID,
        parent_id: NodeID,
        qualified_name: &str,
        is_boolean: bool,
    ) -> io::Result<()> {
        self.begin_node(
            Node::new(
                if is_boolean {
                    "argument_reporter_boolean"
                } else {
                    "argument_reporter_string_number"
                },
                this_id,
            )
            .parent_id(parent_id),
        )?;
        self.single_field("VALUE", qualified_name)?;
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
            d.report(
                DiagnosticKind::ReprArgsCountMismatch {
                    repr: *repr,
                    given: args.len(),
                },
                span,
            );
            return Ok(());
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
                self.input(s, d, arg_name, arg_value, arg_id, false)?;
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
        opr: &Expr,
    ) -> io::Result<()> {
        if matches!(op, UnOp::Not) && !is_expr_boolean(opr, s) {
            return self.un_op(s, d, this_id, parent_id, op, &coerce_condition(opr, s));
        }
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
        self.input(s, d, op.input(), opr, opr_id, matches!(op, UnOp::Not))?;
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
        lhs: &Expr,
        rhs: &Expr,
    ) -> io::Result<()> {
        if matches!(op, BinOp::And | BinOp::Or)
            && !(is_expr_boolean(lhs, s) && is_expr_boolean(rhs, s))
        {
            return self.bin_op(
                s,
                d,
                this_id,
                parent_id,
                op,
                &coerce_condition(lhs, s),
                &coerce_condition(rhs, s),
            );
        }
        if let BinOp::Of = op {
            if let Expr::Name(name) = lhs {
                if let Some(QualifiedName::List(qualified_name, _)) = s.qualify_name(Some(d), name)
                {
                    return self.list_index(s, d, this_id, parent_id, &qualified_name, rhs);
                }
            }
        }
        if let BinOp::In = op {
            if let Expr::Name(name) = rhs {
                if let Some(QualifiedName::List(qualified_name, _)) = s.qualify_name(Some(d), name)
                {
                    return self.list_contains(s, d, this_id, parent_id, &qualified_name, lhs);
                }
            }
            // Handle struct field access: "value" in struct_list.field
            if let Expr::Dot {
                lhs,
                rhs,
                rhs_span: _,
            } = rhs
            {
                if let Expr::Name(name) = lhs.as_ref() {
                    if let Some(QualifiedName::List(list_name, _)) = s.qualify_name(Some(d), name) {
                        let list = s.get_list(&list_name).unwrap();
                        if let Some((type_name, _type_span)) = list.type_.struct_() {
                            let struct_ = s.get_struct(type_name).unwrap();
                            // Verify the field exists in the struct
                            if struct_
                                .fields
                                .iter()
                                .any(|field| field.name == rhs.as_str())
                            {
                                let qualified_name = qualify_struct_var_name(rhs, name.basename());
                                return self.list_contains(
                                    s,
                                    d,
                                    this_id,
                                    parent_id,
                                    &qualified_name,
                                    lhs,
                                );
                            }
                        }
                    }
                }
            }
        }
        let lhs_id = self.id.new_id();
        let rhs_id = self.id.new_id();
        self.begin_node(Node::new(op.opcode(), this_id).parent_id(parent_id))?;
        self.begin_inputs()?;
        let no_empty_shadow = matches!(op, BinOp::And | BinOp::Or);
        self.input(s, d, op.lhs(), lhs, lhs_id, no_empty_shadow)?;
        self.input(s, d, op.rhs(), rhs, rhs_id, no_empty_shadow)?;
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
        self.input(s, d, "INDEX", index, index_id, false)?;
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
        self.input(s, d, "ITEM", item, index_id, false)?;
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
            if struct_.fields.is_empty() {
                // For empty structs, we can't access fields[0], so we use the list name directly
                self.single_field_id("LIST", name)?;
            } else {
                let qualified_name = qualify_struct_var_name(&struct_.fields[0].name, name);
                self.single_field_id("LIST", &qualified_name)?;
            }
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
        signature: &[Arg],
        span: &Span,
        args: &[Expr],
    ) -> io::Result<()> {
        let Some(func) = s.sprite.funcs.get(name) else {
            d.report(DiagnosticKind::UnrecognizedFunction(name.clone()), span);
            return Ok(());
        };
        if signature.len() != args.len() {
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
        for (arg, arg_value) in signature.iter().zip(args) {
            match &arg.type_ {
                Type::Value => {
                    let arg_id = self.id.new_id();
                    self.input(s, d, &arg.name, arg_value, arg_id, false)?;
                    qualified_args.push((arg.name.clone(), arg_id));
                    qualified_arg_values.push(arg_value.clone());
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
                            false,
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
            Mutation::call(func.name.clone(), &qualified_args, true, false)
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
        _this_id: NodeID,
        _parent_id: NodeID,
        lhs: &Expr,
        rhs: &SmolStr,
        _rhs_span: Span,
    ) -> io::Result<()> {
        if let Expr::Name(name) = lhs {
            if let Some(_enum_) = s.get_enum(name.basename()) {
                return Ok(());
            }

            // Check if this is a struct list field access
            // First check if this is a list directly
            if let Some(list) = s.get_list(name.basename()) {
                if let Some((type_name, _type_span)) = list.type_.struct_() {
                    // This is a struct list, check if field exists in struct
                    let struct_ = s.get_struct(type_name).unwrap();
                    // Verify the field exists in the struct
                    if struct_
                        .fields
                        .iter()
                        .any(|field| field.name == rhs.as_str())
                    {
                        let qualified_name = qualify_struct_var_name(rhs, name.basename());
                        let qualified_list_name = QualifiedName::List(qualified_name, Type::Value);
                        match qualified_list_name {
                            QualifiedName::Var(qname, _) => {
                                write!(self, "[3,[12,{},{}],", json!(*qname), json!(*qname))?;
                            }
                            QualifiedName::List(qname, _) => {
                                write!(self, "[3,[13,{},{}],", json!(*qname), json!(*qname))?;
                            }
                        }
                        return write!(self, "[10, \"\"]]");
                    }
                }
            }
        }
        eprintln!("attempted to codegen Expr::Dot lhs = {lhs:#?}, rhs = {rhs:#?}");
        Ok(())
    }

    pub fn property(
        &mut self,
        s: S,
        d: D,
        this_id: NodeID,
        parent_id: NodeID,
        object: &Expr,
        property: &SmolStr,
        _span: &Span,
    ) -> io::Result<()> {
        let menu_id = self.id.new_id();
        let (object_value, is_default) = if let Expr::Value {
            value: Value::String(object_value),
            ..
        } = object
        {
            (object_value, true)
        } else {
            (
                if property == "Stage" {
                    &arcstr::literal!("backdrop #")
                } else {
                    &arcstr::literal!("x position")
                },
                false,
            )
        };
        self.begin_node(
            Node::new("sensing_of_object_menu", menu_id)
                .parent_id(this_id)
                .shadow(true),
        )?;
        self.begin_inputs()?;
        self.end_obj()?; // inputs
        self.single_field("OBJECT", object_value)?;
        self.end_obj()?; // node (sensing_of_object_menu)
        let object_id = self.id.new_id();
        self.begin_node(Node::new("sensing_of", this_id).parent_id(parent_id))?;
        self.begin_inputs()?;
        if is_default {
            write!(self, r#""OBJECT":[1,{}]"#, menu_id)?;
        } else {
            self.input_with_shadow(s, d, "OBJECT", object, object_id, menu_id)?;
        }
        self.end_obj()?; // inputs
        self.single_field("PROPERTY", property)?;
        self.end_obj()?; // node (sensing_of)
        self.expr(s, d, object, object_id, this_id)
    }
}
