use fnum::Fnum;

pub struct FnumBuf<T: Fnum> {
    vec: Vec<u8>,
    t: std::marker::PhantomData<T>,
}

impl<T: Fnum> FnumBuf<T> {
    pub fn new() -> Self {
        FnumBuf {
            vec: Vec::new(),
            t: Default::default(),
        }
    }

    pub fn push(&mut self, e: T) {
        let size = T::size_of_variant(e.variant_index());
        self.vec.extend_from_slice(unsafe {
            std::slice::from_raw_parts(std::mem::transmute::<&T, *const u8>(&e), size)
        });
        std::mem::forget(e);
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            slice: self.vec.as_slice(),
            t: Default::default(),
        }
    }
}

impl<T: Fnum> std::fmt::Debug for FnumBuf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnumBuf({:?} bytes)", self.vec.len())
    }
}

impl<T: Fnum> Drop for FnumBuf<T> {
    fn drop(&mut self) {
        self.for_each(drop);
    }
}


impl<T: Fnum> Iterator for FnumBuf<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vec.is_empty() {
            None
        } else {
            let e = unsafe { std::mem::transmute::<*const u8, &T>(self.vec.as_ptr()) };
            let size = T::size_of_variant(e.variant_index());
            let mut e_: T = unsafe {
                std::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe {
                std::ptr::copy(e, &mut e_ as *mut T, 1);
            }
            self.vec.drain(..size).for_each(drop);
            Some(e_)
        }
    }
}

#[derive(Clone)]
pub struct Iter<'a, T: 'a + Fnum> {
    slice: &'a [u8],
    t: std::marker::PhantomData<T>,
}

impl<'a, T: 'a + Fnum> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let e = unsafe { std::mem::transmute::<*const u8, &'a T>(self.slice.as_ptr()) };
            let size = T::size_of_variant(e.variant_index());
            self.slice = &self.slice[size..];
            Some(e)
        }
    }
}
