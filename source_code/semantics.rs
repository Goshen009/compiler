use crate::ast_types::*;
use std::collections::HashMap;

pub mod type_checker;
pub mod scope;

pub use type_checker::*;
pub use scope::*;

pub fn start_semantics(ast: BlockStatement) {
    let mut global_scope = get_global_scope(&ast);
    print_scope(&global_scope.scope, "Global Scope");

    println!("{}", check_block_scope(&ast, &mut global_scope))
}

pub fn return_hash(val: &String) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

pub fn print_scope<'a>(sc: &HashMap<u64, Symbol<'a>>, sc_name: &str) {
    for (_, sc_symbol) in sc.iter() {
        println!("{sc_name} has: {:?}", sc_symbol);
    }
}

pub fn has_variable_name_been_used_before(scope: &Scope, name: &String) -> bool {
    if let None = scope.check_if_symbol_is_defined_in_scope(name) {
        return false;
    } else {
        return true;
    }
}


/* IMPORTANT: I used u64 as the key for most of the hashmaps here. It's the hash of the variable name.
  A hash is used because it's faster to compute than cloning strings and easier to use than keeping references to it.
  P.S. Try changing it to use a &String and see where I ran into difficulties, oh smarty pants. */

  pub struct Scope<'a> {
    pub scope: HashMap<u64, Symbol<'a>>,
    pub parent: Option<&'a Scope<'a>>
}

impl <'a>Scope<'a> {
    pub fn check_if_symbol_is_defined_in_scope(&self, symbol_name: &String) -> Option<&Symbol> {
        let symbol_hash = return_hash(symbol_name);

        if let Some(symbol) = self.scope.get(&symbol_hash) {
            return Some(symbol); // is symbol in current scope
        } 
        
        if let Some(parent) = self.parent {
            return parent.check_if_symbol_is_defined_in_scope(symbol_name); // is symbol in parent scope
        }        
        
        return None; // it doesn't exist in any scope
    }
}

#[derive(Debug, PartialEq)]
pub enum Symbol<'a> { // A Symbol is an entry into the symbol table. I use enums because the fields for the types are different from one another.
    Variable(VariableSymbol<'a>),
    Struct(StructSymbol<'a>),
    Function(FunctionSymbol<'a>)
}

#[derive(Debug, PartialEq)]
pub struct VariableSymbol<'a> {
    pub type_ : &'a TypesEnum
}

impl <'a>VariableSymbol<'a> {
    pub fn check_type_variable(&self, type_to_check: &TypesEnum) -> bool {
        if self.type_ == type_to_check {
            return true;
        }
        return false;
    }
}

#[derive(Debug, PartialEq)]
pub struct StructSymbol<'a> {
    pub fields : HashMap<u64, &'a TypesEnum>
}

impl <'a>StructSymbol<'a> {
    pub fn check_type_struct<'b>(&self, str_expr: &Box<StructAssignmentExpression>, curr_scope: &Scope<'b>) -> bool {
        // for (field_name_hash, field_type) in self.fields.iter() {   
        //     if let Some((_expr_field_name, expr_field_expr)) = str_expr.struct_fields.get(field_name_hash) {
        //         if type_check(field_type, expr_field_expr, curr_scope) == false {
        //             return false; // if any one of it's fields fails to match on it's type then everything fails
        //         }
        //     } else { panic!("Instantiate all the fields in your struct!"); }
        // }

        return true;
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionSymbol<'a> {
    pub parameters: Vec<&'a TypesEnum>,
    pub return_types: &'a TypesEnum
}

impl <'a>FunctionSymbol<'a> {
    pub fn check_type<'b>(&self, type_to_check: &TypesEnum, curr_scope: &Scope<'b>) -> bool {
        // this one is for the the
        // let t = func_call();
        todo!()
    }
}











    // pub fn expect_relational(&mut self) -> TokenStruct {
    //     let err_msg = format!("Expected a relational operator but found {:?}", self.get_current_token());

    //     let token = self.get_current_token();
    //     if token.binding_power() != BindingPower::Relational {
    //         panic!("{}", err_msg);
    //     }

    //     self.advance()
    // }