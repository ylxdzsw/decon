use decon::*;

#[test]
fn test_no_option() {
    #[reset]
    fn f() -> usize {
        let a = shift(|cont: Cont<usize, usize>| {
            cont(1) + cont(2)
        });
        let b = shift(|cont: Cont<usize, usize>| {
            cont(3) + 4
        });
        a + b
    }

    assert_eq!(f(), 17)
}

#[test]
fn test_box() {
    #[reset]
    fn f() -> usize {
        let a = shift(|cont: ContBox<usize, usize>| {
            cont(1) + cont(2)
        }, ContBox);
        let b = shift(|cont: ContBox<usize, usize>| {
            cont(3) + 4
        }, ContBox);
        a + b
    }

    assert_eq!(f(), 17)
}

#[test]
fn test_ref() {
    #[reset]
    fn f() -> usize {
        let a = shift(|cont: ContRef<usize, usize>| {
            cont(1) + cont(2)
        }, ContRef);
        let b = shift(|cont: ContRef<usize, usize>| {
            cont(3) + 4
        }, ContRef);
        a + b
    }

    assert_eq!(f(), 17)
}

#[test]
fn test_mut() {
    #[reset]
    fn f() -> usize {
        let a = shift(|cont: ContMut<usize, usize>| {
            cont(1) + cont(2)
        }, ContMut);
        let b = shift(|cont: ContMut<usize, usize>| {
            cont(3) + 4
        }, ContMut);
        a + b
    }

    assert_eq!(f(), 17)
}
