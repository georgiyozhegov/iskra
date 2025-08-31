use std::collections::HashMap;

#[cfg(test)]
mod test;

pub struct Interner {
    map: HashMap<&'static str, Symbol>,
    store: Vec<&'static str>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            store: Vec::new(),
        }
    }

    pub fn intern(&mut self, value: String) -> Symbol {
        if let Some(symbol) = self.map.get(value.as_str()) {
            return *symbol;
        }
        let leaked = Box::leak(value.into_boxed_str());
        let symbol = Symbol(self.store.len());
        self.store.push(leaked);
        self.map.insert(leaked, symbol);
        symbol
    }

    pub fn resolve(&self, symbol: Symbol) -> &str {
        &self.store[symbol.0]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol(usize);
