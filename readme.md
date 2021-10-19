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

## Performance


## Limitations

1. Decon is implemented syntactically, so all `shift`s must lexically appear in the body of `#[reset]`. One may however
   use macros to split the body of the `#[reset]` function. Alternative implementations typically catch stack unwinding,
   which does not have this restriction.
2. Decon's AST traversing order is undefined, meaning if there are multiple `shift`s in a single statement, the order of
   continuation may differ from the actual execution. Spliting such statements by `let`s is highly recommened.
