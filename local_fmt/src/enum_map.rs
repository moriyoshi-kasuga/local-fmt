// inspired by https://codeberg.org/xfix/enum-map

use std::{mem::MaybeUninit, ops::Index};

pub trait Enumable {
    const LENGTH: usize;
    type SLICE<V>: Index<usize, Output = V>;

    fn _from_usize(value: usize) -> Self;
    fn _into_usize(self) -> usize;
}

pub struct EnumableMap<K: Enumable, V> {
    pub(crate) slice: K::SLICE<V>,
}

impl<K: Enumable, V> EnumableMap<K, V> {
    #[inline]
    pub fn from_from_value(slice: K::SLICE<V>) -> Self {
        Self { slice }
    }

    pub fn new(f: fn(K) -> V) -> Self {
        let mut uninit = MaybeUninit::<K::SLICE<V>>::uninit();
        (0..K::LENGTH).for_each(|i| {
            let v = f(K::_from_usize(i));
            unsafe { uninit.as_mut_ptr().cast::<V>().add(i).write(v) };
        });
        unsafe { EnumableMap::from_from_value(uninit.assume_init()) }
    }
}

impl<K: Enumable, V> Index<K> for EnumableMap<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, key: K) -> &Self::Output {
        &self.slice[key._into_usize()]
    }
}
