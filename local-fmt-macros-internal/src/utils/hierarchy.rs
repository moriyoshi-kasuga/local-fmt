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

    pub fn process<T>(&mut self, item: I, mut func: impl FnMut(&mut Self) -> T) -> T {
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
    pub fn join(&self, current: &str) -> String {
        let mut items = self.items.iter().map(|i| i.as_ref()).collect::<Vec<_>>();
        items.push(current);
        items.join(".")
    }
}

impl<I: ToString> Hierarchy<I> {
    pub fn joining(&self, current: String) -> String {
        let mut items = self.items.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        items.push(current);
        items.join(".")
    }
}
