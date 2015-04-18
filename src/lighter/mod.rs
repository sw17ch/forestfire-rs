extern crate rand;

use self::rand::distributions::{IndependentSample, Range};

pub fn spark(burning_neighbors: usize, prob_spread: f64) -> bool {
    let between = Range::new(0f64, 1f64);
    let mut rng = rand::thread_rng();

    let bns = burning_neighbors as f64;
    let prob = (1f64 - prob_spread).powf(bns);

    prob < between.ind_sample(&mut rng)
}

pub fn burn_out(prob_burn_out: f64) -> bool {
    let between = Range::new(0f64, 1f64);
    let mut rng = rand::thread_rng();

    between.ind_sample(&mut rng) < prob_burn_out
}

#[test]
fn always() {
    assert!(spark(1, 1f64));
}

#[test]
fn never() {
    assert!(false == spark(1, 0f64));
}

#[test]
fn occasionally() {
    let times = 100; // I dare you to be this unlucky.
    let mut had_true = false;
    let mut had_false = false;

    for _ in 0..times {
        if spark(1, 0.5) {
            had_true = true;
        } else {
            had_false = true;
        }

        if had_true && had_false {
            break;
        }
    }

    assert!(had_true);
    assert!(had_false);
}
