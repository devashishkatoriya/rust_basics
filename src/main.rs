
mod print;
mod vars;

fn main() {

    println!("Hello, world!");

    println!("calling print.rs");
    print::run();

    println!("calling vars.rs");
    vars::run();
}

