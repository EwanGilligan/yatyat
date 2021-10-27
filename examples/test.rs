use std::time::{Duration, Instant};

use yatyat::{
    element::transformation::Transformation,
    semigroup::{
        algs::froidure_pin::FroidurePinBuilder,
        algs::froidure_pin::{froidure_pin_impl::FroidurePin, simple::FroidurePinSimple},
        impls::transformation::TransformationSemigroup,
    },
};

fn main() {
    let s = TransformationSemigroup::new(&[
        Transformation::from_vec(7, vec![1, 0, 2, 3, 4, 5, 6]).unwrap(),
        Transformation::from_vec(7, vec![1, 2, 3, 4, 5, 6, 0]).unwrap(),
        Transformation::from_vec(7, vec![1, 1, 2, 3, 4, 5, 6]).unwrap(),
    ])
    .unwrap();
    let fp = FroidurePinSimple::new(&s);
    println!("Start");
    let start = Instant::now();
    let res = fp.build();
    let end = start.elapsed();
    println!("End, time={}ms", end.as_millis());
}
