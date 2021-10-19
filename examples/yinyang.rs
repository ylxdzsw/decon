use decon::{reset, Cont};

fn main() {
    yinyang()
}

#[reset]
fn yinyang() {
    let yin = shift(|cont: Cont<usize, ()>| {
        // cont(cont)
    });
    let yang = shift(|cont: Cont<usize, ()>| {
        // cont(3) + 4
    });

}
