extern crate getopts;
use getopts::Options;
use std::env;
use std::str::FromStr;

extern crate forestfire;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Specify your options here
    // See the usage docs http://doc.rust-lang.org/getopts/getopts/index.html
    let mut opts = Options::new();
    opts.optopt("d", "frame-delay", "specify delay between frames (milliseconds)", "DELAY");
    opts.optflag("h", "help", "print this help menu");

    let option_matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Look for the help flag
    if option_matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Rather gnarly conversion from String -> u32
    let delay = match option_matches.opt_str("frame-delay") {
        Some(val) => { 
            match u32::from_str(&val) {
                Ok(d) => { d }
                Err(_) => { 0u32 }
            }
        }
        None => { 0u32 }
    };

    let mut f = forestfire::forest::Forest::new(30,30);


    f.light();
    println!("{}", f);

    let mut iters = 0;
    while f.burning() {
        f.burn(0.10, 0.30);
        println!("{}", f);

        std::thread::sleep_ms(delay);

        iters += 1;
    }

    println!("Iterations: {}", iters);
}
