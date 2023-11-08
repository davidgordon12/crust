use std::env;

mod crust;

fn main() {
    if env::args().len() != 2 {
        println!("Invalid arguments!");
        println!("USAGE: crust <url>");
        return;
    }

    let args: Vec<_> = env::args().collect();

    let _res = crust::execute_request(args[1].to_string());
}
