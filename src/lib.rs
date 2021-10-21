pub use decon_macro_impl::reset;

pub struct ContWrap<T, S>(pub ContBox<T, S>);
pub type Cont<T, S> = ContBox<T, S>;
pub type ContBox<T, S> = Box<dyn Fn(T) -> S>;
pub type ContRef<'a, T, S> = &'a dyn Fn(T) -> S;
pub type ContMut<'a, T, S> = &'a mut dyn Fn(T) -> S;

// TODO: inlining closures? it may also simplify the yin yang example
// TODO: the simple stmt-based non-recursive apporach cannot handle shifts that inside control flows (loop { shift(...) })
//       A better way is to reimplement using generators. It can also be multi-shot when https://github.com/rust-lang/rust/issues/57972 lands.
