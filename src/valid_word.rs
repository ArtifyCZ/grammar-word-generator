use crate::sequence_form::SequenceForm;
use crate::terminal_symbol::TerminalSymbol;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ValidWord<'grammar> {
    pub(crate) symbols: Vec<&'grammar TerminalSymbol>,
    pub(crate) path: Vec<SequenceForm<'grammar>>,
}

impl<'grammar> ValidWord<'grammar> {
    pub fn symbols(&self) -> &Vec<&'grammar TerminalSymbol> {
        &self.symbols
    }

    pub fn path(&self) -> &Vec<SequenceForm<'grammar>> {
        &self.path
    }
}
