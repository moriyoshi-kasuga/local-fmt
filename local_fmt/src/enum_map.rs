// inspired by https://codeberg.org/xfix/enum-map

use std::fmt::Debug;
use std::{mem::MaybeUninit, ops::Index};

pub trait Enumable {
    type Array<V>: EnumArray;

    fn _from_usize(value: usize) -> Self;
    fn _into_usize(self) -> usize;
}

pub trait EnumArray {
    const LENGTH: usize;
}

impl<V, const N: usize> EnumArray for [V; N] {
    const LENGTH: usize = N;
}

pub struct EnumableMap<K: Enumable, V> {
    array: K::Array<V>,
}

impl<K: Enumable, V> EnumableMap<K, V> {
    #[inline]
    pub fn from_from_value(array: K::Array<V>) -> Self {
        Self { array }
    }

    pub fn new(f: impl Fn(K) -> V) -> Self {
        let mut uninit = MaybeUninit::<K::Array<V>>::uninit();
        (0..K::Array::<V>::LENGTH).for_each(|i| {
            let v = f(K::_from_usize(i));
            unsafe { uninit.as_mut_ptr().cast::<V>().add(i).write(v) };
        });
        unsafe { EnumableMap::from_from_value(uninit.assume_init()) }
    }

    #[inline]
    pub fn as_slice(&self) -> &[V] {
        unsafe {
            core::slice::from_raw_parts(
                core::ptr::addr_of!(self.array).cast(),
                K::Array::<V>::LENGTH,
            )
        }
    }
}

impl<K: Enumable, V> Index<K> for EnumableMap<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, key: K) -> &Self::Output {
        &self.as_slice()[key._into_usize()]
    }
}

impl<K: Enumable + Debug, V: Debug> Debug for EnumableMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_map();
        for (k, v) in self.as_slice().iter().enumerate() {
            f.entry(&K::_from_usize(k), v);
        }
        f.finish()
    }
}

impl<K: Enumable, V: Clone> Clone for EnumableMap<K, V>
where
    K::Array<V>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
        }
    }
}

impl<K: Enumable, V: Copy> Copy for EnumableMap<K, V> where K::Array<V>: Copy {}
