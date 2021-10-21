use decon::{reset_func, Cont};
use std::{io::{Write, stdout}, rc::Rc};

struct Rec(Rc<Cont<Rec, ()>>);

#[reset_func]
fn main() {
    let yin = shift(|cont: Cont<Rec, ()>| {
        let cont = Rc::new(cont);
        cont(Rec(cont.clone()))
    });
    print!("@"); stdout().flush().unwrap();

    let yang = shift(|cont: Cont<Rec, ()>| {
        let cont = Rc::new(cont);
        cont(Rec(cont.clone()))
    });
    print!("."); stdout().flush().unwrap();

    yin.0(yang)
}
