

# Welcome to my React Markdown Previewer!

## This is a sub-heading...
### And here's some other cool stuff:

Heres some code, `<div></div>`, between 2 backticks.

```
// this is multi-line code:

function anotherExample(firstLine, lastLine) {
  if (firstLine == '```' && lastLine == '```') {
    return multiLineCode;
  }
}
```

You can also make text **bold**... whoa!
Or _italic_.
Or... wait for it... **_both!_**
And feel free to go crazy ~~crossing stuff out~~.

There's also [links](https://github.com/MrNaturi), and
> Block Quotes!

And if you want to get really crazy, even tables:

Wild Header | Crazy Header | Another Header?
------------ | ------------- | -------------
Your content can | be here, and it | can be here....
And here. | Okay. | I think we get it.

- And of course there are lists.
  - Some are bulleted.
     - With different indentation levels.
        - That look like this.


1. And there are numbered lists too.
1. Use just 1s if you want!
1. And last but not least, let's not forget embedded images:

![practiceImage](https://imgs.search.brave.com/3uQtr9jUhG1Mp_ex57zejlu1afEEmry9agUnFZyVong/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly90My5m/dGNkbi5uZXQvanBn/LzA5LzgwLzA0LzI0/LzM2MF9GXzk4MDA0/MjQ2N19TbEJZcjJG/aVplZXNXcnJwa1d4/VmxpM3U2MUNuMFE3/NS5qcGc)








NOTES FOR SEMANTICS
------------------------------------------------------------

A scope is anything written within { }. The exception to this is the global scope which contains all global variables, global struct declarations and function declarations.

struct Scope<'a> {
    I'll leave the key as a u64. Under the types, User-Defined types are stored as u64 (i.e the hash of the variable names). Changing this would mean adding a lifetime annotation to the TypesEnum which in turn means I'd have to change it everywhere else in the code... Definately not doing that! 

    scope: HashMap<u64, Symbol<'a>>

    I choose to continue with this idea of a scope having a reference to the parent it was declared in. It makes it easier to search if a symbol exists within scopes higher than it.
    
    parent: Option<&'a Scope>
}


Symbol is the representation of a variable, struct or function. It states what type it is (variables), what type it's fields are (structs) or what type it's parameters and arguments are (functions).

Because there are many types of symbols that will be on the symbol table, each with different fields it keeps track of, I'll use an enum to keep track of them.

enum Symbol<'a> {
    Variable(&'a TypesEnum),

    Struct(HashMap<u64, &'a TypesEnum>),

    Function{
        parameters: &'a Vec<(String, TypesEnum)>,
        return_types: &'a TypesEnum
    }
}

I switched from having each variant represented as a struct to having the enum hold the value for each of it's variants. I think it makes it much easier to read and follow through... at least for me.

Next, to make type checking much cleaner, I'll have the symbols implement the checks themselves. ASsuming,

    let a: Number = 90;
    let b: Number = a;

    let a: MyStruct = MyStruct {
        field1: 90,
        field2: 0
    }

    let b: MyStruct = a;

Variable 'a' will be parsed first and it's symbol stored in the symbol table.
When b is getting checked, if it's value is a Symbol Expression (which is in this case), 'b' will first check if that symbol is within it's scope heirachy. If it is, then the symbol ('a') will be called to check if the type of b is of the same type.

Same goes for structs. If it is a Struct Expression, then it first checks if the struct is in scope. If it is, the Symbol that was matched in the HashMap will then be called to verify that 1.) The name of the struct matches the name of the type AND 2.) The fields of the struct matches the fields of the Symbol






// I'M NOT EVEN CHECKING IF THE TYPE OF THE STRUCT IS THE SAME AS THE TYPE 
// OF THE VARIABLE IT IS ASSIGNED TO!

// THAT'S WHAT THE CHECK TYPE VARIABLE DOES. IT CHECKS IF 

//     let a: Number = cdf;

// CHECKS IF CDF = Number.

// CDF HERE IS A SYMBOL AND IT IS A VARIABLE SYMBOL.

// FOR STRUCTS -----
// I GUESS IF I CHECK ON THE TYPE OF THE VARIABLE (: NUMBER) 
// AND SEE IF IT IS EQUAL TO THE HASH OF THE NAME OF THE STRUCT EXPRESSION,
// THEN I'LL KNOW IF THEY ARE THE SAME TYPE!

// CAN I TEST ALL THESE THINGS OUT FIRST?



monk main(a: Number, b: String) -> WayoUD {
    ...
    ...
    ...
}