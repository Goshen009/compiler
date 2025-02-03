#[derive(Debug)]
pub struct ParserError {
    pub errors: Vec<String>
}

impl ParserError {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn print_errors(&self) {
        println!("\nERRORS: ");
        for error in self.errors.iter() {
            println!("{error}");
        }
    }
}


#[derive(Debug)]
pub struct SemanticsError {
    pub errors: Vec<String>
}

impl SemanticsError {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn print_errors(&self) {
        println!("\nERRORS: ");
        for error in self.errors.iter() {
            println!("{error}");
        }
    }

    pub fn completed_without_errors(&self) -> bool {
        if self.has_errors() {
            self.print_errors();
        }
        return !self.has_errors();
    }
}



// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// pub enum ParserErrorTypes<'a>{
//     Expects_Name,

//     Expects_Struct_Field,

//     Expects_Token(&'a str),

//     Expects_Stmt_Fn,
// }

// impl <'a>ParserErrorTypes<'a> {
//     fn as_string(&self, position: Position) -> String {
//         match *self {
//             ParserErrorTypes::Expects_Name => format!("Expected a name at {position}"),
//             ParserErrorTypes::Expects_Struct_Field => format!("Expected a field for struct as {position}"),
//             ParserErrorTypes::Expects_Token(token) => format!("Expected '{token}' at {position}"),
//             ParserErrorTypes::Expects_Stmt_Fn => format!("Expected a stmt fn at {position}"),
//         }
//     }
// }