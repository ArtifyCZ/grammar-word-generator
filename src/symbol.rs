use crate::non_terminal_symbol::NonTerminalSymbol;
use crate::terminal_symbol::TerminalSymbol;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
pub enum Symbol<'grammar> {
    NonTerminal(&'grammar NonTerminalSymbol),
    Terminal(&'grammar TerminalSymbol),
}

impl Symbol<'_> {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Symbol::Terminal(_))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Symbol::NonTerminal(s) => s.value.as_str(),
            Symbol::Terminal(s) => s.value.as_str(),
        }
    }   
}
