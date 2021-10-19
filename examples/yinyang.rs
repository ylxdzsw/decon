use decon::{reset, Cont};
use std::{io::{Write, stdout}, rc::Rc};

fn main() {
    yinyang();
}

struct Rec(Rc<Cont<Rec, ()>>);

#[reset]
fn yinyang() {
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
