use crate::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolString<'grammar> {
    pub symbols: Vec<Symbol<'grammar>>,
}

impl<'grammar> SymbolString<'grammar>
{
    pub fn new_empty() -> Self {
        Self { symbols: vec![] }
    }

    pub fn new_single(symbol: Symbol<'grammar>) -> Self {
        Self {
            symbols: vec![symbol],
        }
    }

    pub fn append(&self, symbol: Symbol<'grammar>) -> Self {
        let mut symbols: Vec<Symbol<'grammar>> = Vec::with_capacity(self.symbols.len() + 1);
        for symbol in &self.symbols {
            symbols.push(*symbol);
        }
        symbols.push(symbol);

        Self { symbols }
    }

    pub fn symbols(&self) -> impl Iterator<Item = Symbol<'grammar>> {
        self.symbols.iter().map(|s| *s)
    }

    pub fn replace_once(&self, pattern: &[Symbol<'grammar>], replacement: &SymbolString<'grammar>) -> Option<Self> {
        assert_ne!(pattern.len(), 0, "Cannot replace an empty pattern");
        let pattern_at_idx = self.symbols.windows(pattern.len())
            .position(|window| window == pattern)?;
        let symbols = self.symbols[0..pattern_at_idx].iter().chain(replacement.symbols.iter()).chain(self.symbols[(pattern_at_idx + pattern.len())..].iter()).copied();

        Some(Self { symbols: symbols.collect() })
    }
}
