#![allow(dead_code)]
use handlers::{
    handle_args_list, handle_binary_expression, handle_binary_operator, handle_boolean_literal,
    handle_declaration, handle_expression, handle_function_call, handle_identifier,
    handle_import_stmt, handle_literal, handle_member_access, handle_number_literal,
    handle_string_literal, handle_unary_expression, handle_var_declaration,
};
use pest::Parser;
use pest_derive::Parser;
use std::fs;
use structure::{DeclarationKind, ExecutionContext, ExpressionKind, ImportedModules};

mod errors;
mod handlers;
mod standard_library;
mod structure;

use crate::{
    errors::{throw_undefined, throw_undefined_class_error},
    structure::ArgumentKind,
    standard_library::console::ConsoleModule
};

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
struct WSParser;

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).ok().unwrap();
}

fn main() {
    let input_code = read_file("tests/1.ws");

    match WSParser::parse(Rule::program, &input_code) {
        Ok(program_pairs) => {
            for program_pair in program_pairs {
                let mut imported_modules = ImportedModules::default();
                let mut global_execution_context = ExecutionContext::default();
                let mut line: usize = 1;

                // println!("{program_pair:#?}");

                for inner_pair in program_pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::program => {}
                        Rule::import_stmt => {
                            let module = handle_import_stmt(inner_pair);
                            imported_modules.modules.push(module);
                        }
                        Rule::module_name => {}
                        Rule::declaration => {
                            let declaration = handle_declaration(inner_pair);

                            match declaration.declaration_kind {
                                DeclarationKind::VAR => {
                                    global_execution_context
                                        .scoped_variables
                                        .push(declaration.var_declaration.unwrap());
                                }
                                DeclarationKind::CONST => todo!(),
                                DeclarationKind::CLASS => todo!(),
                                DeclarationKind::INTERFACE => todo!(),
                                DeclarationKind::FUNCTION => todo!(),
                            }
                        }
                        Rule::definition => {}
                        Rule::var_declaration => global_execution_context
                            .scoped_variables
                            .push(handle_var_declaration(inner_pair)),
                        Rule::const_declaration => {}
                        Rule::class_declaration => {}
                        Rule::interface_declaration => {}
                        Rule::function_declaration => {}
                        Rule::parameter_list => {}
                        Rule::parameter => {}
                        Rule::property_declaration => {}
                        Rule::method_declaration => {}
                        Rule::interface_method_declaration => {}
                        Rule::access_modifier => {}
                        Rule::member => {}
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
                            let span_str = inner_pair.as_str();
                            let member = handle_member_access(inner_pair);

                            match member.parent.as_str() {
                                "Console" => {
                                    let found_module = imported_modules
                                        .modules
                                        .iter()
                                        .find(|module| module.name == "Console");

                                    match found_module {
                                        Some(t) => {}
                                        None => {
                                            println!("{:}", throw_undefined_class_error(
                                                "Console",
                                                span_str,
                                                0,
                                                line,
                                                "Consider importing this module with `import std.Console;`",
                                            )
                                            .as_str());
                                            std::process::exit(1);
                                        }
                                    }

                                    match member.identifier.as_str() {
                                        "print" => ConsoleModule::print(member.args_list, &global_execution_context),
                                        "println" => ConsoleModule::println(member.args_list, &global_execution_context),
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
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
                        Rule::new_line => {
                            line += 1;
                        }
                        _ => {}
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
