use super::*;

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Variable(TypesEnum),
    Struct(HashMap<String, TypesEnum>),
    Function{ parameters: TypesEnum, return_type: TypesEnum }
}

impl Symbol {
    pub fn new_variable(var: &VariableDeclarationStatement) -> Symbol {
        Symbol::Variable( var.explicit_type.clone() )
    }

    pub fn new_variable_from_tuple(tup: &TupleDeclarationStatement, index: usize) -> Symbol {
        Symbol::Variable( tup.variable_name_and_type.get(index).unwrap().1.clone() )
    }

    pub fn new_struct(strt: &StructDeclarationStatement) -> Symbol {
        Symbol::Struct( strt.struct_fields.clone() )
    }

    pub fn new_function(func: &FunctionDeclarationStatement) -> Symbol {
        let mut tuple: Vec<TypesEnum> = func.function_parameters.iter().map(|(_, param_type)| param_type.clone()).collect();
        let parameters = if tuple.len() == 1 { tuple.pop().unwrap() } else { return_tuple_type(tuple) };

        Symbol::Function { parameters, return_type: func.function_return_type.clone() }
    }
}

#[derive(Debug, PartialEq)]
pub struct Scope<'scope> {
    pub parent: Option<&'scope Scope<'scope>>,
    pub scope: HashMap<String, Symbol>,
}

impl <'scope>Scope<'scope> {
    pub fn new(parent: Option<&'scope Scope<'_>>) -> Self {
        Self {
            scope: HashMap::new(),
            parent
        }
    }

    pub fn is_global_scope(&self) -> bool {
        if self.parent == None {
            return true;
        } else {
            return false;
        }
    }

    pub fn has_name_been_used_before(&self, name: &String) -> bool {
        if self.is_symbol_defined_in_scope(name) == None { 
            return false; 
        } else {
            return true;
        }
    }  

    pub fn is_symbol_defined_in_scope(&self, symbol_name: &String) -> Option<&Symbol> {
        if let Some(symbol) = self.scope.get(symbol_name) {
            return Some(symbol); // is symbol in current scope
        }

        if let Some(parent) = self.parent {
            return parent.is_symbol_defined_in_scope(symbol_name); // is symbol in parent scope
        }

        return None; // it doesn't exist in any scope
    }
}