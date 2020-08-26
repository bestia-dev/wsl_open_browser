use std::env;
use wsl_open_browser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // only the 1st argument (the 0 argument is the binary path+name)
    open_browser(&args[1]);
}
