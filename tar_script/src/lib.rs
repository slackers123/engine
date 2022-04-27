pub mod ast;
// pub mod astint;
pub mod bcvm;
pub mod bcasm;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

#[derive(Parser)]
#[grammar = "tar-script.pest"]
pub struct TarParser;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     bcvm::run_file_checked(args[1].as_str()).unwrap();

//     // let program = fs::read_to_string(args[1].as_str()).unwrap();

//     // let hash = format!("{:x}", md5::compute(program)).to_owned();

//     // let pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

//     // let (defs, funcs) = ast::parse_to_ast(pairs);

//     // let (funcs, stack, entry) = bcasm::assemble_bc(defs, funcs);

//     // bcvm::store_bc("tar.lock".to_owned(), funcs, stack, entry, hash);

//     // let program = fs::read_to_string("tar.lock").unwrap();

//     // bcvm::run_from_string(program).unwrap();

//     // bcvm::run_func(entry.unwrap(), &funcs, &mut stack, 0);
// }