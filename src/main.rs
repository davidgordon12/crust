use std::env;

mod crust;
use crust::CrustRequest;

fn main() {
    if env::args().len() != 3 {
        println!("Invalid arguments!");
        println!("USAGE: crust <method> <url>");
        return;
    }

    let args: Vec<_> = env::args().collect();

    let req: CrustRequest = CrustRequest::new(
        args[1].to_string(), 
        args[2].to_string()
    );

    let res = req.execute_request();

    println!("{:#?}", res.status());
    println!("{:#?}", res.text().unwrap());
}
