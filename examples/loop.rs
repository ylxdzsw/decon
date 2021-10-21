use decon::{reset, ContBoxMutClonable};

fn main() {
    println!("{:?}", f());
}

#[reset]
fn f() {
    let mut a: Vec<usize> = vec![];
    shift(|cont: ContBoxMutClonable| {
        for i in 0..=2 {
            cont.clone()(())
        }
    }, ContBoxMutClonable);
    a.push(1);
    println!("{:?}", a);
}

#[reset]
fn g(outer_cont: ContBoxMutClonable) {
    shift(|mut cont: ContBoxMutClonable| {
        cont(())
    })
}
