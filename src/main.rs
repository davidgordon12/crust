use std::env;

fn main() {
    if env::args().len() < 3 {
        print!("Too few arguments!");
    }
}
