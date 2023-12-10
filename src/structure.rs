pub fn unbox<T>(value: Box<T>) -> T {
    *value
}

#[derive(Debug, Default, Clone)]
pub struct ImportedModules {
    pub modules: Vec<ImportedModule>,
}

#[derive(Debug, Default, Clone)]
pub struct ImportedModule {
    pub parent: String,
    pub name: String,
    pub full_path: String,
}

#[derive(Debug, Clone)]
pub enum MemberAccessKind {
    PROPERTY,
    METHOD
}

impl Default for MemberAccessKind {
    fn default() -> Self {
        MemberAccessKind::PROPERTY
    }
}

#[derive(Debug, Default, Clone)]
pub struct MemberAccess {
    pub parent: String,
    pub identifier: String,
    pub kind: MemberAccessKind,
    pub args_list: Vec<Argument>,
    pub type_arguments: Option<Vec<Type>>,
}

#[derive(Debug, Default, Clone)]
pub struct Argument {
    pub argument_kind: ArgumentKind,
    pub expression: Option<Expression>,
    pub identifier: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ArgumentKind {
    EXPRESSION,
    IDENTIFIER
}

impl Default for ArgumentKind {
    fn default() -> Self {
        ArgumentKind::EXPRESSION
    }
}

#[derive(Debug, Default, Clone)]
pub struct FunctionCall {
    pub identifier: String,
    pub type_arguments: Vec<Type>,
    pub args_list: Vec<Argument>
}

#[derive(Debug, Default, Clone)]
pub struct BinaryExpression {
    pub first_term: UnaryExpression,
    pub operator: String,
    pub second_term: UnaryExpression,
}

#[derive(Debug, Default, Clone)]
pub struct Expression {
    pub expression_kind: ExpressionKind,
    pub binary_expression: Option<BinaryExpression>,
    pub unary_expression: Option<UnaryExpression>,
    pub prefix_expression: Option<AffixExpression>,
    pub postfix_expression: Option<AffixExpression>,
    pub member_access: Option<MemberAccess>
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    UNARY,
    BINARY,
    PREFIX,
    POSTFIX,
    MEMBERACCESS
}

impl Default for ExpressionKind {
    fn default() -> Self {
        ExpressionKind::UNARY
    }
}

#[derive(Debug, Default, Clone)]
pub struct AffixExpression {
    pub operator: String,
    pub identifier: String
}

#[derive(Debug, Default, Clone)]
pub struct UnaryExpression {
    pub kind: UnaryExpressionKind,
    pub string_value: Option<String>,
    pub function_call: Option<FunctionCall>,
    pub expression: Option<Box<Expression>>
}

#[derive(Debug, Clone)]
pub enum AccessModifier {
    PUBLIC,
    PRIVATE,
    PROTECTED
}

impl Default for AccessModifier {
    fn default() -> Self {
        AccessModifier::PRIVATE
    }
}

#[derive(Debug, Default, Clone)]
pub struct VarDeclaration {
    pub type_def: Type,
    pub identifier: String,
    pub definition: Option<Expression>,
}

#[derive(Debug, Default, Clone)]
pub struct ConstDeclaration {
    pub access_modifier: AccessModifier,
    pub type_name: String,
    pub identifier: String,
    pub definition: Expression,
}

#[derive(Debug, Default, Clone)]
pub struct FunctionDeclaration {
    pub access_modifier: AccessModifier,
    pub is_async: bool,
    pub identifier: String,
    pub type_arguments: Vec<Type>,
    pub parameter_list: Vec<Parameter>,
    pub block: Block,
    pub return_type: Type,
}

#[derive(Debug, Default, Clone)]
pub struct Parameter {
    pub identifier: String,
    pub type_identifier: String,
    pub type_class: ClassDeclaration,
}

#[derive(Debug, Default, Clone)]
pub struct Type {
    pub type_kind: TypeKind,
    pub type_identifier: String,
    pub type_parameters: Vec<String>,
    pub type_class: Option<ClassDeclaration>,
    pub is_type_array: bool
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    BUILTIN,
    SIMPLE,
    GENERIC,
    TYPEARRAY
}

impl Default for TypeKind {
    fn default() -> Self {
        TypeKind::SIMPLE
    }
}

#[derive(Debug, Default, Clone)]
pub struct Block {
    pub statements: Vec<Statement>
}

#[derive(Debug, Default, Clone)]
pub struct Statement {
    pub statement_kind: StatementKind,
    pub expression: Option<Expression>,
    pub async_expression: Option<Expression>,
    pub const_declaration: Option<ConstDeclaration>,
    pub var_declaration: Option<VarDeclaration>,
    pub block: Option<Block>,
    pub if_statement: Option<IfStatement>,
    pub while_statement: Option<WhileStatement>,
    pub for_statement: Option<ForStatement>,
}

#[derive(Debug, Default, Clone)]
pub struct WhileStatement {
    pub expression: Expression,
    pub block: Block
}

#[derive(Debug, Default, Clone)]
pub struct ForStatement {
    pub for_statement_kind: ForStatementKind,
    pub variable: VarDeclaration,
    pub binary_expression: BinaryExpression,
    pub postfix_expression: AffixExpression,
    pub block: Block
}

#[derive(Debug, Clone)]
pub enum ForStatementKind {
    CONDITIONAL,
    ITERATOR
}

impl Default for ForStatementKind {
    fn default() -> Self {
        ForStatementKind::CONDITIONAL
    }
}


#[derive(Debug, Clone)]
pub enum StatementKind {
    EXPRESSION,
    ASYNCEXPRESSION,
    CONSTDECLARATION,
    VARDECLARATION,
    BLOCK,
    IFSTMT,
    WHILESTMT,
    FORSTMT
}

impl Default for StatementKind {
    fn default() -> Self {
        StatementKind::EXPRESSION
    }
}

#[derive(Debug, Default, Clone)]
pub struct IfStatement {
    pub if_statement_kind: IfStatementKind,
    pub expression: Expression,
    pub block: Block,
    pub else_statement: Option<Block>,
    pub else_if_statement: Option<Box<IfStatement>>
}

#[derive(Debug, Clone)]
pub enum IfStatementKind {
    OPEN,
    CLOSED,
    DOUBLE,
}

impl Default for IfStatementKind {
    fn default() -> Self {
        IfStatementKind::DOUBLE
    }
}

#[derive(Debug, Default, Clone)]
pub struct ClassDeclaration {
    pub access_modifier: AccessModifier,
    pub is_static: bool,
    pub identifier: String,
    pub properties: Vec<ClassProperty>,
    pub methods: Vec<ClassMethod>,
    pub constructor: Option<Constructor>
}

#[derive(Debug, Default, Clone)]
pub struct Constructor {
    pub access_modifier: AccessModifier,
    pub identifier: String,
    pub type_arguments: Vec<Type>,
    pub constructor_parameter_list: ConstructorParameterList,
    pub block: Block
}

#[derive(Debug, Default, Clone)]
pub struct ConstructorParameterList {
    pub properties: Vec<ClassProperty>,
    pub parameters: Vec<Parameter>
}

#[derive(Debug, Default, Clone)]
pub struct ClassProperty {
    pub access_modifier: AccessModifier,
    pub is_static: bool,
    pub is_readonly: bool,
    pub identifier: String,
    pub definition: Option<Expression>,
}

#[derive(Debug, Default, Clone)]
pub struct ClassMethod {
    pub access_modifier: AccessModifier,
    pub is_static: bool,
    pub identifier: String,
    pub type_arguments: Vec<Type>,
    pub parameter_list: Vec<Parameter>,
    pub return_type: Type,
    pub block: Block
}

#[derive(Debug, Default, Clone)]
pub struct InterfaceDeclaration {
    pub access_modifier: AccessModifier,
    pub identifier: String,
    pub methods: Vec<InterfaceMethodDeclaration>
}

#[derive(Debug, Default, Clone)]
pub struct InterfaceMethodDeclaration {
    pub access_modifier: AccessModifier,
    pub is_async: bool,
    pub identifier: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionContext {
    pub scope_name: String,
    pub scope_type: ScopeType,
    pub scoped_constants: Vec<ConstDeclaration>,
    pub scoped_variables: Vec<VarDeclaration>,
    pub scoped_classes: Vec<ClassDeclaration>,
    pub scoped_functions: Vec<FunctionDeclaration>,
    pub scoped_interfaces: Vec<InterfaceDeclaration>,
}

#[derive(Debug, Clone)]
pub enum ScopeType {
    GLOBAL,
    CLASSDECLARATION,
    FUNCTIONDECLARATION,
}

impl Default for ScopeType {
    fn default() -> Self {
        ScopeType::GLOBAL
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Default, Clone)]
pub struct Declaration {
    pub declaration_kind: DeclarationKind,
    pub const_declaration: Option<ConstDeclaration>,
    pub var_declaration: Option<VarDeclaration>,
    pub class_declaration: Option<ClassDeclaration>,
    pub function_declaration: Option<FunctionDeclaration>,
    pub interface_declaration: Option<InterfaceDeclaration>,
}

#[derive(Debug, Clone)]
pub enum DeclarationKind {
    VAR,
    CONST,
    CLASS,
    INTERFACE,
    FUNCTION,
}

impl Default for DeclarationKind {
    fn default() -> Self {
        DeclarationKind::VAR
    }
}



















#[derive(Debug, Default, Clone)]
struct test1<'a> {
    test_type: testEnum<'a>
}

#[derive(Debug, Clone)]
enum testEnum<'a> {
    test2(test2<'a>),
    test3(test3)
}

impl <'a>Default for testEnum<'a> {
    fn default() -> Self {
        testEnum::test2(test2::default())
    }
}

#[derive(Debug, Default, Clone)]
struct test2<'a> {
    name: &'a str
}

#[derive(Debug, Default, Clone)]
struct test3 {

}

fn wtt() {
    let mut test = test1::default();
    test.test_type = testEnum::test2(test2 {  name: "test"});

    match test.test_type {
        testEnum::test2(t2) => {
            println!("{}", t2.name);
        }
        testEnum::test3(t3) => {}
    }
}