use core::str;
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(feature = "macros")]
pub use local_fmt_macros as macros;

#[derive(Debug, Clone)]
pub struct LocalFmt<Lang, Key> {
    locales: HashMap<Lang, HashMap<Key, &'static str>>,
    fallback: Lang,
    #[cfg(feature = "selected")]
    selected: Lang,
    #[cfg(feature = "global")]
    global: fn() -> Lang,
}

impl<Lang: std::fmt::Debug + Default + Hash + Eq + Copy, Key: Hash + Eq + Copy>
    LocalFmt<Lang, Key>
{
    #[cfg(not(any(feature = "selected", feature = "global")))]
    pub fn new(fallback: Lang) -> Self {
        Self {
            locales: Default::default(),
            fallback,
        }
    }

    #[cfg(all(feature = "selected", not(feature = "global")))]
    pub fn new(fallback: Lang, selected: Lang) -> Self {
        Self {
            locales: Default::default(),
            fallback,
            selected,
        }
    }

    #[cfg(all(not(feature = "selected"), feature = "global"))]
    pub fn new(fallback: Lang, global: fn() -> Lang) -> Self {
        Self {
            locales: Default::default(),
            fallback,
            global,
        }
    }

    #[cfg(all(feature = "selected", feature = "global"))]
    pub fn new(fallback: Lang, selected: Lang, global: fn() -> Lang) -> Self {
        Self {
            locales: Default::default(),
            fallback,
            selected,
            global,
        }
    }

    pub fn add_locale_fmt(&mut self, lang: Lang, key: Key, value: &'static str) {
        let locale = self.locales.entry(lang).or_default();
        locale.insert(key, value);
    }

    pub fn add_lang(&mut self, lang: Lang, locale: HashMap<Key, &'static str>) {
        self.locales.insert(lang, locale);
    }

    pub fn add_langs_of_key(&mut self, key: Key, locale: HashMap<Lang, &'static str>) {
        for (lang, value) in locale {
            self.add_locale_fmt(lang, key, value);
        }
    }

    #[cfg(feature = "global")]
    pub fn f(&self, key: Key, args: &[(&str, &str)]) -> String {
        self.format((self.global)(), key, args)
    }

    #[cfg(feature = "selected")]
    pub fn fmt(&self, key: Key, args: &[(&str, &str)]) -> String {
        self.format(self.selected, key, args)
    }

    pub fn format(&self, lang: Lang, key: Key, args: &[(&str, &str)]) -> String {
        match self.locales.get(&lang) {
            Some(locale) => match locale.get(&key) {
                Some(value) => replace_args(value, args),
                None => {
                    assert_ne!(self.fallback, lang, "fallback locale should be set");
                    self.format(self.fallback, key, args)
                }
            },
            None => {
                assert_ne!(self.fallback, lang, "fallback locale should be set");
                self.format(self.fallback, key, args)
            }
        }
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
                let key_s = unsafe { str::from_utf8_unchecked(&buffer) };
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
