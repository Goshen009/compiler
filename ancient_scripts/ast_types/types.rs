use std::fmt::Debug;
use super::{TypeTrait, TypeObj};

impl TypeTrait for PrimaryType { }
impl TypeTrait for ArrayType { }

#[derive(Debug)]
pub struct PrimaryType {
    pub type_: String, // T (i.e int or string...)
}

#[derive(Debug)]
pub struct ArrayType {
    pub underlying: TypeObj // []T (i.e an array of type T (T here can also be another array))
}
