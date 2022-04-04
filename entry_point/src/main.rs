#[macro_use]
extern crate tarator;

fn main() {
    env_logger::init();    
    TR_ERROR!("TEST ERROR\n");
}