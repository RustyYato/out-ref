#[cfg(feature = "nightly")]
mod nightly {
    #[test]
    fn maybe_uninit() {
        use std::mem::MaybeUninit;
        use crate::Out;

        let mut x = MaybeUninit::uninitialized();
        let mut out_x = Out::from_maybe_uninit(&mut x);

        out_x.set(10);

        let x = unsafe { x.assume_init() };
        assert_eq!(x, 10);
    }
}

#[test]
fn normal() {
    use crate::*;

    let mut x = 0;

    let mut out_x = x.out();
    out_x.set(10);

    assert_eq!(x, 10);
}

#[test]
fn leak() {
    use crate::*;

    let mut x = vec![0, 1, 2];

    let mut out_x = x.out();
    out_x.set(vec![]);

    assert_eq!(x, vec![]);
}