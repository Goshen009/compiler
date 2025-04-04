use std::collections::{HashMap, HashSet};
use crate::ast_types::*;
use crate::error_codes::*;

// pub mod scope;
pub mod type_checker;
pub mod usage;

// pub use scope::*;
pub use type_checker::*;
pub use usage::*;

pub fn add_global_symbol_to_scope(&mut self, stmt: &StatementEnum) -> Result<(), String> { // if the scope already has a symbol with the same name
    match stmt {
        StatementEnum::FunctionDeclaration(func) => {
            if self.has_name_been_used_before(&func.function_name) {
                return Err(format!("There is already a function, struct or variable with the name '{}'", &func.function_name));
            }

            self.scope.insert(func.function_name.clone(), Symbol::new_function(&func));
            Ok(())
        }
        StatementEnum::StructDeclaration(strt) => {
            if self.has_name_been_used_before(&strt.struct_name) {
                return Err(format!("There is already a function, struct or variable with the name '{}'", &strt.struct_name));
            }

            self.scope.insert(strt.struct_name.clone(), Symbol::new_struct(&strt));
            Ok(())
        }
        StatementEnum::TupleDeclaration(tup) => {
            let mut index = 0;

            for (name, _) in tup.variable_name_and_type.iter() {
                if self.has_name_been_used_before(name) {
                    return Err(format!("There is already a function, struct or variable with the name '{}'", name));
                }
    
                self.scope.insert(name.clone(), Symbol::new_variable_from_tuple(&tup, index));
                index += 1;
            }

            Ok(())
        }
        StatementEnum::VariableDeclaration(var) => {
            if self.has_name_been_used_before(&var.variable_name) {
                return Err(format!("There is already a function, struct or variable with the name '{}'", &var.variable_name));
            }

            self.scope.insert(var.variable_name.clone(), Symbol::new_variable(&var));
            Ok(())
        }
    }
}

pub fn start_semantics(program: Program) {
    // let mut global_scope = get_global_scope(&ast);
    // print_scope(&global_scope.scope, "Global Scope");

    // println!("{}", check_block_scope(&ast, &mut global_scope))
    // let errors = check_through_block_scope(&ast, &mut global_scope);

    // let mut global_scope = Scope {
    //     scope: HashMap::new(),
    //     parent: None
    // };

    // let func_symbol = Symbol::Function {
    //     parameters: return_tuple_type(vec![return_primary_type(Types::Number)]),
    //     return_type: &return_tuple_type(vec![return_primary_type(Types::Number), return_primary_type(Types::String)])
    // };
    // let func_name = String::from("none");
    // global_scope.scope.insert(&func_name, func_symbol);




    let mut global_scope = Scope {
        scope: HashMap::new(),
        parent: None
    };

    let mut errors: Vec<ErrorCodes> = Vec::new();
    for statement in program.code.iter() {
        errors.append(&mut create_symbol(statement, &mut global_scope));
    }

    // Do something with the errors here! Maybe return them or something!

    let mut errors: Vec<ErrorCodes> = Vec::new();
    for statement in program.code.iter() {
        errors.append(&mut check_statement(statement, &mut global_scope));
    }

    if errors.len() > 0 { println!("All errors: {:?}", errors); }
    else { println!("No errors! Wheeeee"); }
}

pub fn return_hash(val: &String) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

// pub fn print_scope<'a>(sc: &HashMap<&String, Symbol<'a>>, sc_name: &str) {
//     for (_, sc_symbol) in sc.iter() {
//         println!("{sc_name} has: {:?}", sc_symbol);
//     }
// }

pub struct Scope<'a> {
    pub scope: HashMap<&'a String, Symbol<'a>>,
    pub parent: Option<&'a Scope<'a>>
}

impl <'a>Scope<'a> {
    pub fn has_variable_name_been_used_before(&self, var_name: &String) -> bool {
        if let None = self.check_if_symbol_is_defined_in_scope(var_name) {
            return false;
        }
        else {
            return true;
        }
    }

    pub fn check_if_symbol_is_defined_in_scope(&self, symbol_name: &String) -> Option<&Symbol> {
        // let symbol_hash = return_hash(symbol_name);

        if let Some(symbol) = self.scope.get(symbol_name) {
            return Some(symbol); // is symbol in current scope
        } 
        
        if let Some(parent) = self.parent {
            return parent.check_if_symbol_is_defined_in_scope(symbol_name); // is symbol in parent scope
        }        
        
        return None; // it doesn't exist in any scope
    }
}

#[derive(PartialEq, Debug)]
pub enum Symbol<'a> {
    Variable(&'a TypesEnum),

    Struct(&'a HashMap<String, TypesEnum>),

    Function{
        parameters: TypesEnum,
        return_type: &'a TypesEnum
    }
}

pub fn return_variable_symbol<'a>(type_: &'a TypesEnum) -> Symbol<'a> {
    Symbol::Variable(type_)
}

pub fn return_struct_symbol<'a>(fields: &'a HashMap<String, TypesEnum>) -> Symbol<'a> {
    Symbol::Struct(fields)
}

pub fn return_function_symbol<'a>(parameters: &'a Vec<(String, TypesEnum)>, return_type: &'a TypesEnum) -> Symbol<'a> {
    Symbol::Function { 
        parameters: (|| {
            let mut tuple: Vec<TypesEnum> = parameters.iter().map(|(_, param_type)| param_type.clone()).collect();
            if tuple.len() == 1 { return tuple.pop().unwrap() } else { return_tuple_type(tuple) }
        })(), 
        return_type 
    }
}