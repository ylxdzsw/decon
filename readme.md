Decon
=====

DElimited CONtinuation for Rust.

## Usage

Decon provides a `#[reset]` attribute for function definitions. Inside the function body, one gains access to a keyword
(in the form of a function) `shift`, which accepts a single argument `f` of type `fn(&mut dyn FnMut(T) -> S) -> S`,
where `S` is the return type of the `#[reset]` function and `T` is the type of the `shift` expression. The argument
passed to `f` is the (delimited) continuation of the `shift` expression.

## Examples

### NQueens

### Yin-Yang Puzzle

## What the hell is it?

### Relation to `call/cc`

1. the continuation is delimited;
2. as an implication of 1, the continuation returns and does not need to be a tail call;
3. as an implication of 2, the continuation can be called multiple times.

### Relation to Effect Handlers

> while relates to goto, as effect handlers to shift/reset

Effect handlers are more structured / deciplined constructions that are usually implemented using delimited continuations.

### Relation to `try/catch`

It can be seen as _resumable_ exceptions, where you can resume the execution of a `try` block by setting a value for
the failed expression in the `cache` handler.

### Relation to `yield`, `async/await`

## Limitations

Too much.
