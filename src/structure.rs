pub fn unbox<T>(value: Box<T>) -> T {
    *value
}


#[derive(Debug, Default)]
pub struct ImportedModules {
    pub modules: Vec<ImportedModule>,
}

#[derive(Debug, Default)]
pub struct ImportedModule {
    pub parent: String,
    pub name: String,
    pub full_path: String,
}

#[derive(Debug)]
pub enum MemberAccessKind {
    PROPERTY,
    METHOD
}

impl Default for MemberAccessKind {
    fn default() -> Self {
        MemberAccessKind::PROPERTY
    }
}

#[derive(Debug, Default)]
pub struct MemberAccess {
    pub parent: String,
    pub identifier: String,
    pub kind: MemberAccessKind,
    pub args_list: ArgsList
}

#[derive(Debug, Default)]
pub struct FunctionCall {
    pub identifier: String,
    pub args_list: ArgsList
}

#[derive(Debug, Default)]
pub struct BinaryExpression {
    pub first_term: UnaryExpression,
    pub operator: String,
    pub second_term: UnaryExpression,
}

#[derive(Debug, Default)]
pub struct Expression {
    pub binary_expression: Option<BinaryExpression>,
    pub unary_expression: Option<UnaryExpression>,
    pub prefix_expression: Option<AffixExpression>,
    pub postfix_expression: Option<AffixExpression>,
    pub member_access: Option<MemberAccess>
}

#[derive(Debug, Default)]
pub struct AffixExpression {
    pub operator: String,
    pub identifier: String
}

#[derive(Debug, Default)]
pub struct ExecutionContext<'a> {
    pub imported_modules: Option<&'a ImportedModules>,
    pub scope: &'a str,
}

#[derive(Debug)]
pub enum UnaryExpressionKind {
    IDENTIFIER,
    LITERAL,
    FUNCTIONCALL,
    EXPRESSION
}

impl Default for UnaryExpressionKind {
    fn default() -> Self {
        UnaryExpressionKind::IDENTIFIER
    }
}

#[derive(Debug, Default)]
pub struct UnaryExpression {
    pub kind: UnaryExpressionKind,
    pub string_value: Option<String>,
    pub function_call: Option<FunctionCall>,
    pub expression: Option<Box<Expression>>
}

#[derive(Debug, Default)]
pub struct ArgsList {
    pub args: Vec<Expression>
}

