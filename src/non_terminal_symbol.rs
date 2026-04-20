use crate::symbol::Symbol;

#[derive(Debug, PartialEq, Eq)]
pub struct NonTerminalSymbol {
    pub value: String,
}

impl NonTerminalSymbol {
    pub fn as_symbol(&self) -> Symbol<'_> {
        Symbol::NonTerminal(self)
    }
}
