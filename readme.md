Decon
=====

DElimited CONtinuation for Rust.

## Usage

Decon provides a `reset! {}` block inside which one gains access to a new keyword (in the form of a function) `shift`,
which accepts a single argument `f` of type `fn(Box<dyn FnMut(T) -> S>) -> S`, where `S` is the type of the `reset`
expression and `T` is the type of the `shift` expression. The argument passed to `f` is the (delimited) continuation of
the `shift` expression. A function attribute `#[reset_func]` is equivalent to `reset!` a whole function definition.

`shift` optionally accepts the second argument, which specifies the type of the continuation (`Box`ed or (mut) borrowed).
See tests/basic.rs for examples.

Decon is still under development and anything other than the examples may not work. Specifically, `shift` inside control
flows (`if` branches, `loop`, etc.) do not work for now. _You know how to implement loops now you have `call/cc`, right?_

## Examples

### Yin-Yang Puzzle

> https://stackoverflow.com/questions/2694679/how-does-the-yin-yang-puzzle-work

```rust
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
```

### Non-deterministic

The following program will return a set `{-2, -1, 0, 1, 2, 3, 4}`. It corresponds to the `list` monad in effect
handlers. Unfortunately, as Decon is lexically scoped and statically typed, `f()` cannot simply return `i32` and the
implementations of `choose` and `flip` cannot be changed at call site.

```rust
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
```

### NQueens

This example will print all solutions to nqueens. It also shows how to clone a continuation as well as its captured
states.

```rust
type Board = Vec<i32>;

#[derive(Clone)]
struct Rec<T>(Rc<ContBoxClonable<(Rec<T>, T)>>);

#[reset_func]
fn nqueens(n: i32) {
    // board[i] = j means a queen in (i,j)
    let (loop_back, mut board): (Rec<Board>, Board) = shift(get_cc);
    let next_row = board.len();
    let next = shift(fork(0..n));

    if board.iter().enumerate().any(|(i, j)| j == &next || (next - j).abs() as usize == next_row - i) {
        return
    }

    board.push(next);
    if board.len() < n as _ {
        loop_back.0((loop_back.clone(), board));
    } else {
        println!("{:?}", board);
    }
}

fn get_cc<T: Default>(cont: ContBoxClonable<(Rec<T>, T)>) {
    let cont = Rc::new(cont);
    cont((Rec(cont.clone()), Default::default()))
}

fn fork<T: Clone>(iter: impl IntoIterator<Item=T>) -> impl FnOnce(ContBoxOnceClonable<T>) {
    move |cont| {
        for i in iter {
            cont.clone()(i);
        }
    }
}
```

## Limitations

1. Decon is implemented syntactically, so all `shift`s must lexically appear in the body of `reset`. Alternative
   implementations typically catch stack unwinding, which don't have this restriction.
2. Decon's AST traversing order is undefined, i.e., if there are multiple `shift`s in a single statement, the order of
   continuations may differ from the actual execution order. Spliting such statements by `let`s is highly recommened.