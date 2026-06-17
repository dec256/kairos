use crate::elf::Elf;

mod elf;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("we require 2 args -> ./kairos <path>");
    }
    let path = &args[0] as &str;
    let elf = Elf::parse(&path).expect("failure in parse");
}
