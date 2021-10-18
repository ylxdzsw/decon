fn main() {
    println!("hello fuck and {}", mannual_cps_inline());
    println!("hello fuck and {}", mannual_cps_no_inline());
}

/*
fn raw(n: usize) -> usize {
    let a = shift(|cont| {
        cont(1) + cont(2)
    });
    println!("{}", a);
    let b = shift(|cont| {
        cont(3) + 4
    });
    a + b
}
*/

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
    (|cont: fn(usize) -> usize| {
        cont(1) + cont(2)
    })(|v| {
        let a = v;
        println!("{}", a);

        (|cont: &mut dyn FnMut(usize) -> usize| {
            cont(3) + 4
        })(&mut |v| {
            let b = v;
            a + b
        })
    })
}