use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum TypesEnum {
    Primary(PrimaryType),
    Array(Box<ArrayType>),
    Tuple(Box<TupleType>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryType {
    pub typeq: Types,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayType {
    pub underlying: TypesEnum
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleType {
    pub tuple: Vec<TypesEnum>
}

// I'm thinking about something.
// Instead of the type_ to be a string, what if it was an Enum?
// So in-built types like Number and String can be easily known
// And for user-defined types, it'll be User_Defined(u64)
// u64 refers to the name of the user-defined type
// But because Strings in Enums have proven to be a hard thing (to compare)
// I'll hash the name of the user-defined type and store that as it's name.

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Types {
    User_Defined(String),
    Number,
    String,
}