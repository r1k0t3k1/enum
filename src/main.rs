
mod argparser;
mod scanner;

fn main() {
    let args = argparser::Argument::new();
    match args.parse_ipaddresses() {
        Ok(arg) => println!("{:#?}", arg),
        Err(e)  => eprintln!("{}", e),
    };
}
