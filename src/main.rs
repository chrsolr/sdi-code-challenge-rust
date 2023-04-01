use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get args value
    // -file [VALUE]
    // -sort [VALUE]
    dbg!(args);
}
