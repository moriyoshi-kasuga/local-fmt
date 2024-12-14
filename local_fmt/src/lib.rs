pub mod enum_map;
pub use enum_map::*;

#[cfg(feature = "macros")]
pub use local_fmt_macros as macros;

#[cfg(feature = "macros")]
pub trait EnumIter: Sized {
    fn iter<'a>() -> core::slice::Iter<'a, Self>;
}

#[derive(Debug)]
pub struct LocalFmt<Lang: Enumable, Key: Enumable> {
    pub locales: EnumableMap<Lang, EnumableMap<Key, &'static str>>,
    #[cfg(feature = "selected")]
    pub selected: Lang,
    #[cfg(feature = "global")]
    pub global: fn() -> Lang,
}

impl<Lang: Enumable + Clone, Key: Enumable + Clone> Clone for LocalFmt<Lang, Key>
where
    Key::Array<&'static str>: Clone,
    Lang::Array<EnumableMap<Key, &'static str>>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            locales: self.locales.clone(),
            #[cfg(feature = "selected")]
            selected: self.selected.clone(),
            #[cfg(feature = "global")]
            global: self.global,
        }
    }
}

impl<Lang: Enumable + Copy, Key: Enumable> LocalFmt<Lang, Key> {
    #[cfg(not(any(feature = "selected", feature = "global")))]
    pub fn new(locales: EnumableMap<Lang, EnumableMap<Key, &'static str>>) -> Self {
        Self { locales }
    }

    #[cfg(all(feature = "selected", not(feature = "global")))]
    pub fn new(locales: EnumableMap<Lang, EnumableMap<Key, &'static str>>, selected: Lang) -> Self {
        Self { locales, selected }
    }

    #[cfg(all(not(feature = "selected"), feature = "global"))]
    pub fn new(
        locales: EnumableMap<Lang, EnumableMap<Key, &'static str>>,
        global: fn() -> Lang,
    ) -> Self {
        Self { locales, global }
    }

    #[cfg(all(feature = "selected", feature = "global"))]
    pub fn new(
        locales: EnumableMap<Lang, EnumableMap<Key, &'static str>>,
        selected: Lang,
        global: fn() -> Lang,
    ) -> Self {
        Self {
            locales,
            selected,
            global,
        }
    }

    #[cfg(feature = "global")]
    #[inline]
    pub fn f(&self, key: Key, args: &[(&str, &str)]) -> String {
        self.format((self.global)(), key, args)
    }

    #[cfg(feature = "selected")]
    #[inline]
    pub fn fmt(&self, key: Key, args: &[(&str, &str)]) -> String {
        self.format(self.selected, key, args)
    }

    #[allow(clippy::panic)]
    pub fn format(&self, lang: Lang, key: Key, args: &[(&str, &str)]) -> String {
        replace_args(self.locales[lang][key], args)
    }
}

pub fn replace_args(text: &'static str, args: &[(&str, &str)]) -> String {
    let args = args.iter();
    let input_bytes = text.as_bytes();
    let mut output = Vec::<u8>::with_capacity(input_bytes.len() + 64);
    let mut input_bytes = input_bytes.iter();

    let mut inner = false;

    let mut buffer = Vec::<u8>::new();

    while let Some(i) = input_bytes.next() {
        match *i {
            b'}' if inner => {
                let key_s = unsafe { core::str::from_utf8_unchecked(&buffer) };
                match args.clone().find(|(key, _)| *key == key_s) {
                    Some((_, value)) => {
                        output.extend(value.as_bytes());
                    }
                    None => output.extend(&[b"%{", buffer.as_slice(), b" is not binded}"].concat()),
                };
                inner = false;
            }
            _ if inner => {
                buffer.push(*i);
            }
            b'%' => match match input_bytes.next() {
                Some(i) => i,
                None => break,
            } {
                b'%' => {
                    output.push(b'%');
                }
                b'{' => inner = true,
                v => {
                    output.extend(&[b'%', *v]);
                }
            },
            _ => {
                output.push(*i);
            }
        };
    }

    unsafe { String::from_utf8_unchecked(output) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_replace_args() {
        assert_eq!(
            replace_args("Hello, %{world}", &[("world", "World!")]),
            "Hello, World!"
        );
    }

    #[test]
    fn escape_replace_args() {
        assert_eq!(
            replace_args("Hello, %%{world}", &[("world", "World!")]),
            "Hello, %{world}"
        );
    }

    #[test]
    fn no_bind_replace_args() {
        assert_eq!(
            replace_args("Hello, %{world}", &[]),
            "Hello, %{world is not binded}"
        );
    }
}
