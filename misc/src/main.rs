use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::collections::BTreeSet;
use std::ops::Bound::Included;

fn main() {
}

fn single_sample() {
    let mut rng = rand::thread_rng();
    println!("{}", rng.gen_range(0..10));
}

fn multiple_sample() {
    let between = Uniform::from(10..10000);
    let mut rng = rand::thread_rng();
    let mut sum = 0;
    for _ in 0..1000 {
        sum += between.sample(&mut rng);
    }
    println!("{}", between.sample(&mut rng));
}

fn btree_range() {
    let mut set = BTreeSet::new();
    set.insert(3);
    set.insert(5);
    set.insert(8);
    for &elem in set.range((Included(&4), Included(&8))) {
        println!("{elem}");
    }
}
