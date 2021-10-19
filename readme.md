Decon
=====

DElimited CONtinuation for Rust.

## Usage

Decon provides a `#[reset]` attribute for function definitions. Inside the function body, one gains access to a keyword
(in the form of a function) `shift`, which accepts a single argument `f` of type `fn(Box<dyn FnMut(T) -> S>) -> S`,
where `S` is the return type of the `#[reset]` function and `T` is the type of the `shift` expression. The argument
passed to `f` is the (delimited) continuation of the `shift` expression.

## Examples

### NQueens

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

## Performance


## Limitations

1. Decon is implemented syntactically, so all `shift`s must lexically appear in the body of `#[reset]`. One may however
   use macros to split the body of the `#[reset]` function. Alternative implementations typically catch stack unwinding,
   which don't have this restriction.
2. Decon's AST traversing order is undefined, i.e. if there are multiple `shift`s in a single statement, the order of
   continuations may differ from the actual execution order. Spliting such statements by `let`s is highly recommened.
