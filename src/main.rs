use clap::Parser;

mod argparser;

fn main() {
    let args = argparser::Argument::parse();
    println!("{:#?}", args);
}
