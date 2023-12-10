use pest::iterators::Pair;

use crate::{structure::{ImportedModule, Argument, ArgumentKind, Expression, UnaryExpression, UnaryExpressionKind, BinaryExpression, AffixExpression, FunctionCall, MemberAccess, MemberAccessKind, TypeKind, VarDeclaration, Declaration, Type, DeclarationKind}, Rule};

pub fn handle_import_stmt(pair: Pair<'_, Rule>) -> ImportedModule {
    let mut module = ImportedModule::default();
    let module_name = pair.into_inner().next().unwrap();

    module.full_path = module_name.as_str().to_string();
    module.parent = module_name
        .clone()
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .to_string();
    module.name = module_name
        .into_inner()
        .last()
        .unwrap()
        .as_str()
        .to_string();

    module
}

pub fn handle_args_list(pair: Pair<'_, Rule>) -> Vec<Argument> {
    let mut args_list = Vec::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::expression => {
                let mut argument = Argument::default();
                argument.argument_kind = ArgumentKind::EXPRESSION;
                argument.expression = Some(handle_expression(inner_pair));
                args_list.push(argument);
            }
            Rule::identifier => {
                let mut argument = Argument::default();
                argument.argument_kind = ArgumentKind::IDENTIFIER;
                argument.identifier = Some(inner_pair.as_str().to_string());
                args_list.push(argument);
            }
            _ => {}
        }
    }

    args_list
}

pub fn handle_expression(pair: Pair<'_, Rule>) -> Expression {
    let mut expression = Expression::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::binary_expression => {
                expression.binary_expression = Some(handle_binary_expression(inner_pair))
            }
            Rule::unary_expression => {
                expression.unary_expression = Some(handle_unary_expression(inner_pair))
            }
            Rule::prefix_expression => {
                expression.prefix_expression = Some(handle_prefix_expression(inner_pair))
            }
            Rule::postfix_expression => {
                expression.postfix_expression = Some(handle_prefix_expression(inner_pair))
            }
            Rule::member_access => {
                expression.member_access = Some(handle_member_access(inner_pair))
            }
            _ => {}
        }
    }

    expression
}

pub fn handle_unary_expression(pair: Pair<'_, Rule>) -> UnaryExpression {
    let mut unary_expression = UnaryExpression::default();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => {
                unary_expression.string_value = Some(handle_identifier(inner_pair));
                unary_expression.kind = UnaryExpressionKind::IDENTIFIER;
            }
            Rule::literal => {
                unary_expression.string_value = Some(handle_literal(inner_pair));
                unary_expression.kind = UnaryExpressionKind::LITERAL;
            }
            Rule::function_call => {
                unary_expression.function_call = Some(handle_function_call(inner_pair));
                unary_expression.kind = UnaryExpressionKind::FUNCTIONCALL;
            }
            Rule::expression => {
                unary_expression.expression = Some(Box::new(handle_expression(inner_pair)));
                unary_expression.kind = UnaryExpressionKind::EXPRESSION;
            }
            _ => {}
        }
    }

    unary_expression
}

pub fn handle_binary_expression(pair: Pair<'_, Rule>) -> BinaryExpression {
    let mut binary_expression = BinaryExpression::default();
    let mut idx = 0;
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::unary_expression => {
                idx += 1;

                if idx < 1 {
                    binary_expression.first_term = handle_unary_expression(inner_pair);
                } else {
                    binary_expression.second_term = handle_unary_expression(inner_pair);
                }
            }
            Rule::binary_operator => {
                binary_expression.operator = handle_binary_operator(inner_pair);
            }
            _ => {}
        }
    }

    binary_expression
}

pub fn handle_prefix_expression(pair: Pair<'_, Rule>) -> AffixExpression {
    let mut affix_expression = AffixExpression::default();

    affix_expression
}

pub fn handle_postfix_expression(pair: Pair<'_, Rule>) -> AffixExpression {
    let mut affix_expression = AffixExpression::default();

    affix_expression
}

pub fn handle_identifier(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

pub fn handle_literal(pair: Pair<'_, Rule>) -> String {
    let mut literal = String::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::number_literal => literal = handle_number_literal(inner_pair),
            Rule::string_literal => literal = handle_string_literal(inner_pair),
            Rule::boolean_literal => literal = handle_boolean_literal(inner_pair),
            _ => {}
        }
    }

    literal
}

pub fn handle_number_literal(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

pub fn handle_string_literal(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

pub fn handle_boolean_literal(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

pub fn handle_binary_operator(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

pub fn handle_function_call(pair: Pair<'_, Rule>) -> FunctionCall {
    let mut function_call = FunctionCall::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => function_call.identifier = handle_identifier(inner_pair),
            Rule::arg_list => function_call.args_list = handle_args_list(inner_pair),
            _ => {}
        }
    }

    function_call
}

pub fn handle_method_call(pair: Pair<'_, Rule>) -> FunctionCall {
    handle_function_call(pair.into_inner().next().unwrap())
}

pub fn handle_member_access(pair: Pair<'_, Rule>) -> MemberAccess {
    let mut member_access = MemberAccess::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::unary_expression => {
                member_access.parent = handle_unary_expression(inner_pair).string_value.unwrap()
            }
            Rule::identifier => member_access.identifier = handle_identifier(inner_pair),
            Rule::method_call => {
                // get the identifier and args_list
                let mut function_call = handle_method_call(inner_pair);
                member_access.identifier = function_call.identifier;
                member_access.args_list = function_call.args_list;
                member_access.kind = MemberAccessKind::METHOD
            }
            _ => {}
        }
    }

    member_access
}

pub fn handle_type(pair: Pair<'_, Rule>) -> Type {
    let mut type_def = Type::default();
    let type_pair = pair.into_inner().next().unwrap();

    match type_pair.as_rule() {
        Rule::simple_type => {
            type_def.type_identifier = type_pair.as_str().to_string();
            type_def.type_kind = TypeKind::SIMPLE
        }
        Rule::generic_type => {
            type_def.type_kind = TypeKind::GENERIC;
            type_def.type_identifier = type_pair
                .clone()
                .into_inner()
                .nth(0)
                .unwrap()
                .as_str()
                .to_string();
            for type_parameter_identifier in type_pair.into_inner().nth(1).unwrap().into_inner() {
                type_def
                    .type_parameters
                    .push(type_parameter_identifier.as_str().to_string());
            }
        }
        Rule::type_array => {
            type_def.type_kind = TypeKind::TYPEARRAY;
            type_def.is_type_array = true;

            type_def.type_identifier = type_pair
                .clone()
                .into_inner()
                .nth(0)
                .unwrap()
                .as_str()
                .to_string();

            for sub_type_pair in type_pair.into_inner() {
                match sub_type_pair.as_rule() {
                    Rule::simple_type => {
                        type_def.type_identifier = sub_type_pair.as_str().to_string().clone();
                    }
                    Rule::generic_type => {
                        for type_parameter_identifier in
                            sub_type_pair.into_inner().nth(1).unwrap().into_inner()
                        {
                            type_def
                                .type_parameters
                                .push(type_parameter_identifier.as_str().to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {
            type_def.type_identifier = type_pair.as_str().to_string();
            type_def.type_kind = TypeKind::TYPEARRAY;
        }
    }

    type_def
}

pub fn handle_var_declaration(pair: Pair<'_, Rule>) -> VarDeclaration {
    let mut var_declaration = VarDeclaration::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::r#type => {
                var_declaration.type_def = handle_type(inner_pair);
            }
            Rule::identifier => {
                var_declaration.identifier = handle_identifier(inner_pair);
            }
            Rule::definition => {
                var_declaration.definition =
                    Some(handle_expression(inner_pair.into_inner().next().unwrap()));
            }
            _ => {}
        }
    }

    var_declaration
}

pub fn handle_declaration(pair: Pair<'_, Rule>) -> Declaration {
    let mut declaration = Declaration::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::var_declaration => {
                declaration.var_declaration = Some(handle_var_declaration(inner_pair));
            }
            Rule::const_declaration => {
                declaration.declaration_kind = DeclarationKind::CONST;
            }
            Rule::class_declaration => {
                declaration.declaration_kind = DeclarationKind::CLASS;
            }
            Rule::function_declaration => {
                declaration.declaration_kind = DeclarationKind::FUNCTION;
            }
            Rule::interface_declaration => {
                declaration.declaration_kind = DeclarationKind::INTERFACE;
            }
            _ => {}
        }
    }

    declaration
}

fn solve_expression(expression: Expression) {}