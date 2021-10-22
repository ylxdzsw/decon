use decon::*;

fn main() {
    let a = Box::into_raw(Box::new([1,2,3])); // a variable that needs clearing
    reset! {
        shift(defer(|| clear_resource(a)), ContRef); // defer clearing
        unsafe { (*a)[0] += 1 } // do something with a
    };
}

fn defer(clear: impl FnOnce()) -> impl FnOnce(ContRef) {
    move |cont| { cont(()); clear() }
}

fn clear_resource(a: *mut [i32; 3]) {
    unsafe {
        println!("clear {:?}", (*a));
        Box::from_raw(a);
    }
}