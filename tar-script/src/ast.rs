pub enum BinOp {// types of binary operation
    Plus,
    Minus,
    Mul,
    Div,
}

pub enum AstNode {
    Definitions(Vec<AstNode>), // The value is the array of type Definition or Import
    Definition{
        target: String,
        value: String,
    },
    Import(Vec<String>), // The string vector is the "path" to import e.g. std::log -> ["std", "log"]
    FuncDefs(Vec<AstNode>), // array of function definitions (just functions in .pest)
    FuncDef {
        ident: String, // name of the function
        args: Vec<AstNode>, // arguments the function requires (optional)
        ret_ty: String, // type to be returned (optional)
        block: Vec<AstNode>, // The function block (code inside the function)
    },
    Arg{
        ident: String, // name of the argument
        ty: String, // type of the argument
    },
    Integer(i32),
    Float(f32),
    String(String),
    BinOp { // binary operation (operation with two arguments)
        op: BinOp, // type of operation
        lhs: Box<AstNode>, // left side of operation
        rhs: Box<AstNode>, // right side of operation
    },
    ReturnStmt(Box<AstNode>), // Value is the expression to be evaluated for returning
    Declaration { // A variable declaration e.g. int a = 10;
        ty: String,
        ident: String,
        val: Box<AstNode>,
    },
    ValAssign { // a value assignment to a variable e.g. a = 20;
        ident: String,
        val: Box<AstNode>
    },
    FuncCall { // a call to a function e.g. log("Hello, World!");
        ident: String, // name of the function to be called
        args: Vec<AstNode>, // array of arguments given to the function (optional)
    },
    Expr(Box<AstNode>), // Can either be a calculation or a string
}