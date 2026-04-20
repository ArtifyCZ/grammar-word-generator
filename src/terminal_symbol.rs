use crate::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TerminalSymbol {
    pub value: String,
}

impl TerminalSymbol {
    pub fn as_symbol(&self) -> Symbol<'_> {
        Symbol::Terminal(self)
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}
