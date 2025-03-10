/// A simple stack-like structure that can be used to keep track of a hierarchy.
/// This is useful for keeping track of a hierarchy of items, such as a hierarchy of keys.
/// # Example
/// ```rust
/// use local_fmt_macros_internal::utils::hierarchy::Hierarchy;
/// let mut hierarchy = Hierarchy::new();
/// hierarchy.process("a", |h| {
///     h.process("b", |h| {
///         assert_eq!(h.join("key"), "a.b.key");
///    });
///    assert_eq!(h.join("key"), "a.key");
/// });
///
/// assert_eq!(hierarchy.join("key"), "key");
pub struct Hierarchy<I> {
    items: Vec<I>,
}

impl<I> Default for Hierarchy<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I> Hierarchy<I> {
    pub fn new() -> Self {
        Hierarchy { items: Vec::new() }
    }

    /// Process an item and a function that takes a mutable reference to the hierarchy.
    /// The item is pushed onto the hierarchy before the function is called and popped off after the function is called.
    /// Example usage of [Hierarchy]
    pub fn process<T>(&mut self, item: I, func: impl FnOnce(&mut Self) -> T) -> T {
        self.items.push(item);
        let t = func(self);
        self.items.pop();
        t
    }

    pub fn as_vec(&self) -> &Vec<I> {
        &self.items
    }
}

impl<I: AsRef<str>> Hierarchy<I> {
    /// Join the items in the hierarchy with the current item.
    /// The items are joined with a period.
    /// The generic type must implement [AsRef<str>].
    ///
    /// Example usage of [Hierarchy]
    pub fn join(&self, current: &str) -> String {
        let mut items = self.items.iter().map(|i| i.as_ref()).collect::<Vec<_>>();
        items.push(current);
        items.join(".")
    }
}

impl<I: ToString> Hierarchy<I> {
    /// Join the items in the hierarchy with the current item.
    /// The items are joined with a period.
    /// The generic type must implement [ToString].
    ///
    /// Similarity Example usage of [Hierarchy]
    pub fn joining(&self, current: String) -> String {
        let mut items = self.items.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        items.push(current);
        items.join(".")
    }
}
