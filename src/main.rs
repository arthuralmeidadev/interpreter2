#![allow(dead_code)]
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::fs;
use structure::{
    unbox, AffixExpression, ArgsList, BinaryExpression, ExecutionContext, Expression, FunctionCall,
    ImportedModule, ImportedModules, MemberAccess, MemberAccessKind, UnaryExpression,
    UnaryExpressionKind,
};

mod standard_library;
mod structure;
use standard_library::{ConsoleModule, StandardLibrary};

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
struct WSParser;

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).ok().unwrap();
}

fn handle_import_stmt(pair: Pair<'_, Rule>) -> ImportedModule {
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

fn handle_args_list(pair: Pair<'_, Rule>) -> ArgsList {
    let mut args_list = ArgsList::default();
    for inner_pair in pair.into_inner() {
        args_list.args.push(handle_expression(inner_pair));
    }

    args_list
}

fn handle_expression(pair: Pair<'_, Rule>) -> Expression {
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

fn handle_unary_expression(pair: Pair<'_, Rule>) -> UnaryExpression {
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

fn handle_binary_expression(pair: Pair<'_, Rule>) -> BinaryExpression {
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

fn handle_prefix_expression(pair: Pair<'_, Rule>) -> AffixExpression {
    let mut affix_expression = AffixExpression::default();

    affix_expression
}

fn handle_postfix_expression(pair: Pair<'_, Rule>) -> AffixExpression {
    let mut affix_expression = AffixExpression::default();

    affix_expression
}

fn handle_identifier(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn handle_literal(pair: Pair<'_, Rule>) -> String {
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

fn handle_number_literal(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn handle_string_literal(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn handle_boolean_literal(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn handle_binary_operator(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn handle_function_call(pair: Pair<'_, Rule>) -> FunctionCall {
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

fn handle_method_call(pair: Pair<'_, Rule>) -> FunctionCall {
    handle_function_call(pair.into_inner().next().unwrap())
}

fn handle_member_access(pair: Pair<'_, Rule>) -> MemberAccess {
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

fn solve_expression(expression: Expression) {

}

fn main() {
    let input_code = read_file("tests/1.ws");

    match WSParser::parse(Rule::program, &input_code) {
        Ok(program_pairs) => {
            for program_pair in program_pairs {
                let mut imported_modules = ImportedModules::default();

                for inner_pair in program_pair.into_inner() {
                    // println!("{:#?}", inner_pair);

                    match inner_pair.as_rule() {
                        Rule::program => {}
                        Rule::import_stmt => imported_modules
                            .modules
                            .push(handle_import_stmt(inner_pair)),
                        Rule::module_name => {}
                        Rule::declaration => {}
                        Rule::definition => {}
                        Rule::var_declaration => {}
                        Rule::const_declaration => {}
                        Rule::class_declaration => {}
                        Rule::interface_declaration => {}
                        Rule::function_declaration => {}
                        Rule::parameter_list => {}
                        Rule::parameter => {}
                        Rule::property_declaration => {}
                        Rule::method_declaration => {}
                        Rule::inerface_property_declaration => {}
                        Rule::access_modifier => {}
                        Rule::member => {}
                        Rule::interface_member => {}
                        Rule::stmt => {}
                        Rule::expression => {
                            let expression = handle_expression(inner_pair);
                        }
                        Rule::unary_expression => {
                            let unary_expression = handle_unary_expression(inner_pair);
                        }
                        Rule::binary_expression => {
                            let binary_expression = handle_binary_expression(inner_pair);
                        }
                        Rule::prefix_expression => {}
                        Rule::postfix_expression => {}
                        Rule::literal => {
                            let literal = handle_literal(inner_pair);
                        }
                        Rule::number_literal => {
                            let literal = handle_number_literal(inner_pair);
                        }
                        Rule::string_literal => {
                            let literal = handle_string_literal(inner_pair);
                        }
                        Rule::boolean_literal => {
                            let literal = handle_boolean_literal(inner_pair);
                        }
                        Rule::if_stmt => {}
                        Rule::while_stmt => {}
                        Rule::for_stmt => {}
                        Rule::function_call => {
                            let function = handle_function_call(inner_pair);
                        }
                        Rule::method_call => {}
                        Rule::arg_list => {
                            let args_list = handle_args_list(inner_pair);
                        }
                        Rule::member_access_infix => {}
                        Rule::member_access => {
                            let member = handle_member_access(inner_pair);

                            match member.parent.as_str() {
                                "Console" => {
                                    match member.identifier.as_str() {
                                        "println" => {
                                            for arg in member.args_list.args {
                                                ConsoleModule::println(arg.unary_expression.unwrap().string_value.unwrap());
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            // println!("{:#?}", member);
                        }
                        Rule::r#type => {}
                        Rule::type_array => {}
                        Rule::simple_type => {}
                        Rule::void => {}
                        Rule::generic_type => {}
                        Rule::type_argument => {}
                        Rule::identifier => {
                            let identifier = handle_identifier(inner_pair);
                        }
                        Rule::binary_operator => {
                            let binary_operator = handle_binary_operator(inner_pair);
                        }
                        Rule::prefix_operator => {}
                        Rule::postfix_operator => {}
                        Rule::WHITESPACE => {}
                        Rule::ASCII_ALPHANUMERIC => {}
                        Rule::block => {}
                    }
                }
            }
        }
        Err(error) => {
            // Handle parsing error
            println!("Error: {:#?}", error);
        }
    }
}
