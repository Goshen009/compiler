




#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ErrorCodes{
    Function_Parameter_Has_A_Name_That_Has_Been_Used_In_Scope{
        function_name: String,
        parameter_name: String
    },
    
    Function_Name_Has_Been_Used_Before(String),
    Struct_Name_Has_Been_Used_Before(String),
    Variable_Name_Has_Been_Used_Before(String),

    Variable_Is_Assigned_An_Incorrect_Type(String),

    Assigned_Symbol_Does_Not_Exist(),
    Struct_Not_Defined(),
    Struct_Fields_Do_Not_Match(),
    
    Empty_Tuple_Defined(),
}