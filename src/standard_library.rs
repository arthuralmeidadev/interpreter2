#[derive(Debug, Default)]
pub struct StandardLibrary {
    pub console_module: ConsoleModule
}

#[derive(Debug, Default)]
pub struct ConsoleModule {}
impl ConsoleModule {
    pub fn print(value: String) {
        print!("{}", value);
    }

    pub fn println(value: String) {
        println!("{}", value);
    }
}

struct FunctionDeclaration {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Option<Type>,
    // ... other function declaration details
}

struct Parameter {
    name: String,
    data_type: Type,
    // ... other parameter details
}

struct Type {
    // ... type details
}