extern crate forestfire;

pub fn main() {
    let mut f = forestfire::forest::Forest::new(30,30);


    f.light();
    println!("{}", f);

    let mut iters = 0;
    while f.burning() {
        f.burn(0.10, 0.30);
        println!("{}", f);

        // uncomment this to see this more as an animation
        // std::thread::sleep_ms(250);

        iters += 1;
    }

    println!("Iterations: {}", iters);
}
