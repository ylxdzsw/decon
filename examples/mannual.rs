use decon::reset;

fn main() {
    println!("hello fuck and {}", raw(1));
    println!("hello fuck and {}", mannual_cps_inline());
    println!("hello fuck and {}", mannual_cps_no_inline());
}

fn shift(f: impl Fn(Box<dyn Fn(usize) -> usize>) -> usize) -> usize {
    unimplemented!()
}

#[reset]
fn raw(n: usize) -> usize {
    let a = shift(|cont: Box<dyn Fn(usize) -> usize>| {
        cont(1) + cont(2)
    });
    println!("{}", a);
    let b = shift(|cont: Box<dyn Fn(usize) -> usize>| {
        cont(3) + 4
    });
    a + b
}

fn mannual_cps_inline() -> usize {
    let cont = |v: usize| {
        let a = v;
        println!("{}", a);

        let cont = |v: usize| {
            let b = v;
            a + b
        };

        cont(3) + 4
    };

    cont(1) + cont(2)
}

#[allow(clippy::redundant_closure_call)]
fn mannual_cps_no_inline() -> usize {
    (|mut cont: Box<dyn FnMut(usize) -> usize>| {
        cont(1) + cont(2)
    })(Box::new(move |v| {
        let a = v;
        println!("{}", a);

        (|mut cont: Box<dyn FnMut(usize) -> usize>| {
            cont(3) + 4
        })(Box::new(move |v| {
            let b = v;
            a + b
        }))
    }))
}