use std::hash::Hash;
use crate::non_terminal_symbol::NonTerminalSymbol;
use crate::symbol::Symbol;
use crate::symbol_string::SymbolString;

#[derive(Debug, Clone, Eq, Ord, PartialOrd)]
pub struct SequenceForm<'grammar> {
    /// left part `α` where `a` is a string of non-terminal and terminal symbols
    pub left: SymbolString<'grammar>,
    /// inner part `A` where `A` is a string of non-terminal symbols only
    pub inner: &'grammar NonTerminalSymbol,
    /// right part `b` where `b` is a string of non-terminal and terminal symbols
    pub right: SymbolString<'grammar>,
}

impl<'grammar> SequenceForm<'grammar> {
    pub fn symbols(&self) -> impl Iterator<Item = Symbol<'grammar>> {
        std::iter::chain(
            std::iter::chain(
                self.left.symbols(),
                std::iter::once(Symbol::NonTerminal(self.inner)),
            ),
            self.right.symbols(),
        )
    }
}

impl<'grammar> PartialEq for SequenceForm<'grammar> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_symbols = self.symbols();
        let mut right_symbols = other.symbols();
        loop {
            let self_symbol = self_symbols.next();
            let right_symbol = right_symbols.next();
            if self_symbol.is_none() && right_symbol.is_none() {
                return true;
            }

            if self_symbol != right_symbol {
                return false;
            }
        }
    }
}
