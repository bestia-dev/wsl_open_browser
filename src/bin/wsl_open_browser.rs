use std::env;
use wsl_open_browser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // only the 1st argument (the 0 argument is the binary path+name)
    // if we omit the argument it will default to "."
    let arg_1 = if args.len() >= 2 { &args[1] } else { "." };
    open_browser(arg_1);
}
