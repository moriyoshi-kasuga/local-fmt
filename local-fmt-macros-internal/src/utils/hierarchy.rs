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

    pub fn process(&mut self, item: I, mut func: impl FnMut(&mut Self)) {
        self.items.push(item);
        func(self);
        self.items.pop();
    }
}

impl<I: AsRef<str>> Hierarchy<I> {
    pub fn join(&self, current: &str) -> String {
        let mut items = self.items.iter().map(|i| i.as_ref()).collect::<Vec<_>>();
        items.push(current);
        items.join(".")
    }
}
