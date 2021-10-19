pub use decon_macro_impl::reset;

pub type Cont<T, S> = Box<dyn Fn(T) -> S>;

// roadmap
// 1. a submodule for non-deterministic
// 2. examples and benchmark
// 3. inline
// 4. borrowed continuation