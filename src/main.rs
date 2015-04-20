extern crate getopts;
use getopts::Options;
use std::env;
use std::str::FromStr;

extern crate forestfire;

struct AppConfig {
  forest_width: usize,
  forest_height: usize,
  frame_delay: u32,
  spread_probability: f64,
  burn_out_probability: f64,
}

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
    opts.optopt("", "width", "width of forest (tiles)", "WIDTH");
    opts.optopt("", "height", "height of forest (tiles)", "HEIGHT");
    opts.optopt("d", "frame-delay", "specify delay between frames (milliseconds)", "DELAY");
    opts.optopt("s", "spread-rate", "normalized rate of spread", "RATE");
    opts.optopt("b", "burn-out-rate", "normalized rate of burning out", "RATE");
    opts.optflag("h", "help", "print this help menu");

    let option_matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Look for the help flag and return if present
    if option_matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // If nothing is overridden, use these defaults
    let config: AppConfig = AppConfig {
        // RD 20Apr2015 16:12:25 - Give me one-liner power and I'll use it!
        forest_width: option_matches.opt_str("width").and_then(|v| usize::from_str_radix(&v, 10).ok()).unwrap_or(30),
        forest_height: option_matches.opt_str("height").and_then(|v| usize::from_str_radix(&v, 10).ok()).unwrap_or(30),
        frame_delay: option_matches.opt_str("frame-delay").and_then(|v| u32::from_str(&v).ok()).unwrap_or(0),
        spread_probability: option_matches.opt_str("spread-rate").and_then(|v| f64::from_str(&v).ok()).unwrap_or(0.1f64),
        burn_out_probability: option_matches.opt_str("burn-out-rate").and_then(|v| f64::from_str(&v).ok()).unwrap_or(0.3f64)
    };

    let mut f = forestfire::forest::Forest::new(config.forest_width, config.forest_height);


    f.light();
    println!("{}", f);

    let mut iters = 0;
    while f.burning() {
        f.burn(config.spread_probability, config.burn_out_probability);
        println!("{}", f);

        std::thread::sleep_ms(config.frame_delay);

        iters += 1;
    }

    println!("Iterations: {}", iters);
}
