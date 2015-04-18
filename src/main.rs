extern crate forestfire;

pub fn main() {
    let mut f = forestfire::forest::Forest::new(30,30);


    f.light();
    println!("{}", f);

    while f.burning() {
        f.burn(0.25, 0.25);
        println!("{}", f);
    }
}
