Decon
=====

DElimited CONtinuation for Rust.

## Usage

Decon provides a `#[reset]` attribute for function definitions. Inside the function body, one gains access to a keyword
(in the form of a function) `shift`, which accepts a single argument `f` of type `fn(Box<dyn FnMut(T) -> S>) -> S`,
where `S` is the return type of the `#[reset]` function and `T` is the type of the `shift` expression. The argument
passed to `f` is the (delimited) continuation of the `shift` expression.

## Examples

### Yin-Yang Puzzle

> https://www.zhihu.com/question/27683900

```rust
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
```

### Non-deterministic

The following program will return a set `{-2, -1, 0, 1, 2, 3, 4}`. It corresponds to the `list` monad in effect
handlers. Unfortunately, as Decon is lexically scoped and statically typed, `f()` cannot simply return `i32` and the
implementations of `choose` and `flip` cannot be changed at call site.

```rust
#[reset]
fn f() -> BTreeSet<i32> {
    let a = shift(choose(0..=2));
    let b = shift(choose(0..=2));
    if shift(flip()) {
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

fn flip<S: Ord>() -> impl FnOnce(Cont<bool, BTreeSet<S>>) -> BTreeSet<S> {
    choose([true, false])
}
```

### NQueens


## Performance


## Limitations

1. Decon is implemented syntactically, so all `shift`s must lexically appear in the body of `#[reset]`. One may however
   use macros to split the body of the `#[reset]` function. Alternative implementations typically catch stack unwinding,
   which don't have this restriction.
2. Decon's AST traversing order is undefined, i.e. if there are multiple `shift`s in a single statement, the order of
   continuations may differ from the actual execution order. Spliting such statements by `let`s is highly recommened.
