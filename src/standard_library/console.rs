use crate::structure::{Argument, ExpressionKind, ArgumentKind, ExecutionContext};

pub struct ConsoleModule {}
impl ConsoleModule {
    pub fn print(arguments: Vec<Argument>, execution_context: &ExecutionContext) {
        for arg in arguments {
            let mut value = String::new();
            match arg.argument_kind {
                ArgumentKind::EXPRESSION => {
                    let expression = arg.expression.unwrap();
                    match expression.expression_kind {
                        ExpressionKind::UNARY => {
                            // let variable_name = expression
                            //     .unary_expression
                            //     .unwrap()
                            //     .string_value
                            //     .unwrap()
                            //     .replace('"', "");
                        }
                        ExpressionKind::BINARY => {}
                        ExpressionKind::MEMBERACCESS => {}
                        ExpressionKind::PREFIX => {}
                        ExpressionKind::POSTFIX => {}
                    }
                }
                ArgumentKind::IDENTIFIER => {
                    let identifier_node = execution_context
                        .scoped_variables
                        .iter()
                        .find(|variable| variable.identifier == arg.identifier.clone().unwrap());

                    value = identifier_node
                        .clone()
                        .unwrap()
                        .clone()
                        .definition
                        .unwrap()
                        .unary_expression
                        .unwrap()
                        .string_value
                        .unwrap()
                        .replace('"', "");
                }
            }

            print!("{}", value);
        }
        
    }

    pub fn println(arguments: Vec<Argument>, execution_context: &ExecutionContext) {
        for arg in arguments {
            let mut value = String::new();
            match arg.argument_kind {
                ArgumentKind::EXPRESSION => {
                    let expression = arg.expression.unwrap();
                    match expression.expression_kind {
                        ExpressionKind::UNARY => {
                            // let variable_name = expression
                            //     .unary_expression
                            //     .unwrap()
                            //     .string_value
                            //     .unwrap()
                            //     .replace('"', "");
                        }
                        ExpressionKind::BINARY => {}
                        ExpressionKind::MEMBERACCESS => {}
                        ExpressionKind::PREFIX => {}
                        ExpressionKind::POSTFIX => {}
                    }
                }
                ArgumentKind::IDENTIFIER => {
                    let identifier_node = execution_context
                        .scoped_variables
                        .iter()
                        .find(|variable| variable.identifier == arg.identifier.clone().unwrap());

                    value = identifier_node
                        .clone()
                        .unwrap()
                        .clone()
                        .definition
                        .unwrap()
                        .unary_expression
                        .unwrap()
                        .string_value
                        .unwrap()
                        .replace('"', "");
                }
            }

            println!("{}", value);
        }
        
    }
}
