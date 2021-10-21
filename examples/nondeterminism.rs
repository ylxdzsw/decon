use decon::{reset_func, Cont};
use std::collections::BTreeSet;

fn main() {
    println!("{:?}", f());
}

#[reset_func]
fn f() -> BTreeSet<i32> {
    let a = shift(choose(0..=2));
    let b = shift(choose(0..=2));
    if shift(flip) {
        [a + b].into_iter().collect()
    } else {
        [a - b].into_iter().collect()
    }
}

fn choose<T, S: Ord>(iter: impl IntoIterator<Item=T>) -> impl FnOnce(Cont<T, BTreeSet<S>>) -> BTreeSet<S> {
    move |cont| {
        iter.into_iter().flat_map(cont).collect()
    }
}

fn flip<S: Ord>(cont: Cont<bool, BTreeSet<S>>) -> BTreeSet<S> {
    choose([true, false])(cont)
}