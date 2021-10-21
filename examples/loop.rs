use std::rc::Rc;

use decon::*;

fn main() {
    println!("{:?}", f());
}

#[reset_func]
fn f() {
    let (loop_back, mut v) = shift(|cont| get_cc(cont, Default::default()));
    let next = shift(fork([0, 1]), ContBox);
    v.push(next);
    if v.len() <= 5 {
        loop_back.0((loop_back.clone(), v.clone()));
    } else {
        println!("{:?}", v);
    }
}

#[derive(Clone)]
struct Rec(Rc<ContBoxClonable<(Rec, Vec<usize>), ()>>);
fn get_cc(cont: ContBoxClonable<(Rec, Vec<usize>), ()>, v: Vec<usize>) {
    let cont = Rc::new(cont);
    cont((Rec(cont.clone()), v))
}

fn fork<T: Clone>(iter: impl IntoIterator<Item=T>) -> impl FnOnce(ContBoxMutClonable<T, ()>) {
    move |cont| {
        for i in iter {
            dyn_clone::clone_box(&cont)(i)
        }
    }
}