pub use decon_macro_impl::reset;
use dyn_clone::DynClone;

pub type Cont<T=(), S=()> = ContBox<T, S>;
pub type ContBox<T=(), S=()> = Box<dyn Fn(T) -> S>;
pub type ContRef<'a, T=(), S=()> = &'a dyn Fn(T) -> S;
pub type ContMut<'a, T=(), S=()> = &'a mut dyn FnMut(T) -> S;
pub type ContBoxMut<T=(), S=()> = Box<dyn FnMut(T) -> S>;
pub type ContBoxOnce<'a, T=(), S=()> = Box<dyn FnOnce(T) -> S>;
pub type ContBoxClonable<T=(), S=()> = Box<dyn ClonableFn<T, S>>;
pub type ContBoxMutClonable<T=(), S=()> = Box<dyn ClonableFnMut<T, S>>;
pub type ContBoxOnceClonable<'a, T=(), S=()> = Box<dyn ClonableFnOnce<T, S>>;

// maybe still useful for internal mutability. Otherwise Rc<ContBox> should be enough.
pub trait ClonableFn<T=(), S=()>: DynClone + Fn(T) -> S {}
impl<T, S, F: Clone + Fn(T) -> S> ClonableFn<T, S> for F {}
dyn_clone::clone_trait_object!(ClonableFn);

pub trait ClonableFnMut<T=(), S=()>: DynClone + FnMut(T) -> S {}
impl<T, S, F: Clone + FnMut(T) -> S> ClonableFnMut<T, S> for F {}
dyn_clone::clone_trait_object!(ClonableFnMut);

pub trait ClonableFnOnce<T=(), S=()>: DynClone + FnOnce(T) -> S {}
impl<T, S, F: Clone + FnOnce(T) -> S> ClonableFnOnce<T, S> for F {}
dyn_clone::clone_trait_object!(ClonableFnOnce);

// TODO: inlining closures? it may also simplify the yin yang example
// TODO: the simple stmt-based non-recursive apporach cannot handle shifts that inside control flows (loop { shift(...) })
//       A better way is to reimplement using generators. It can also be multi-shot when https://github.com/rust-lang/rust/issues/57972 lands.
