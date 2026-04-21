use crate::non_terminal_symbol::NonTerminalSymbol;
use crate::sequence_form::SequenceForm;
use crate::symbol::Symbol;
use crate::symbol_string::SymbolString;
use crate::terminal_symbol::TerminalSymbol;
use std::collections::BTreeMap;
use crate::iterators::{ValidWordIterator, ValidWordIteratorStrategy};

type RulesVecMap = Vec<(
    // key: (left, inner, right)
    (Vec<String>, NonTerminalSymbol, Vec<String>),
    // value: production replacements
    Vec<Vec<String>>,
)>;

/// A formal grammar representation.
///
/// This grammar should be understood in the context of the formal language theory,
/// and an instance of this `Grammar` struct represents a formal grammar
/// of a given formal language.
///
/// ## Formal definition
///
/// A grammar `G` is a four-tuple `G = (N, ∑, P, S)` where:
/// - `N` is a finite set of non-terminal symbols (also known as the variable symbols)
/// - `∑` is a finite set of terminal symbols
///   (technically, the alphabet of the language, or its subset)
/// - `P` is a finite set of production rules
/// - `S` is the start symbol, belonging to the set of non-terminal symbols `N`
///   (also known as the initial symbol or the sentence symbol)
///
/// Note that a symbol, both terminal and non-terminal, is a string of characters,
/// and strings of symbols are represented as vectors of symbols, e.g. `Vec<Symbol>`, `Vec<String>`.
///
/// @TODO: use generic type parameters for the symbols instead of just `String`
pub struct Grammar {
    /// Also known as the variable symbols
    pub(crate) non_terminals: Vec<NonTerminalSymbol>,
    /// Also known as the alphabet of the language
    pub(crate) terminals: Vec<TerminalSymbol>,
    pub(crate) rules: RulesVecMap,
    pub(crate) start_symbol: NonTerminalSymbol,
}

fn parse_sequence_form(
    raw_symbols: Vec<String>,
    non_terminals: &[NonTerminalSymbol],
) -> Option<(Vec<String>, NonTerminalSymbol, Vec<String>)> {
    let mut left = Vec::new();
    let mut inner = None;
    let mut right = Vec::new();
    for raw_symbol in raw_symbols {
        if inner.is_some() {
            right.push(raw_symbol);
            continue;
        }
        if non_terminals.iter().any(|nt| nt.value == raw_symbol) {
            inner = Some(NonTerminalSymbol { value: raw_symbol });
        } else {
            left.push(raw_symbol);
        }
    }

    inner.map(|inner| (left, inner, right))
}

impl Grammar {
    pub fn new(
        non_terminals: Vec<String>,
        terminals: Vec<String>,
        rules: BTreeMap<Vec<String>, Vec<Vec<String>>>,
        start_symbol: String,
    ) -> Self {
        // @TODO: add validation
        let non_terminals: Vec<NonTerminalSymbol> = non_terminals
            .into_iter()
            .map(|s| NonTerminalSymbol { value: s })
            .collect();
        let terminals: Vec<TerminalSymbol> = terminals
            .into_iter()
            .map(|s| TerminalSymbol { value: s })
            .collect();
        let rules = rules
            .into_iter()
            .map(|(k, v)| (parse_sequence_form(k, non_terminals.as_slice()).unwrap(), v))
            .collect();
        Self {
            non_terminals,
            terminals,
            rules,
            start_symbol: NonTerminalSymbol {
                value: start_symbol,
            },
        }
    }

    pub fn find_symbol<'grammar, 'value>(
        &'grammar self,
        value: &'value str,
    ) -> Option<Symbol<'grammar>>
    where
        'value: 'grammar,
    {
        if let Some(non_terminal_symbol) = self.non_terminals.iter().find(|s| value == s.value) {
            return Some(Symbol::NonTerminal(non_terminal_symbol));
        }

        if let Some(terminal_symbol) = self.terminals.iter().find(|s| value == s.value) {
            return Some(Symbol::Terminal(terminal_symbol));
        }

        None
    }

    pub fn rules(&self) -> impl Iterator<Item = (SequenceForm<'_>, Vec<SymbolString<'_>>)> {
        self.rules.iter().map(|(pattern, replacements)| {
            let (left_raw, inner, right_raw) = pattern;
            let left = SymbolString {
                symbols: left_raw
                    .iter()
                    .map(|s| self.find_symbol(s).unwrap())
                    .collect(),
            };
            let right = SymbolString {
                symbols: right_raw
                    .iter()
                    .map(|s| self.find_symbol(s).unwrap())
                    .collect(),
            };
            let pattern = SequenceForm { left, inner, right };
            let replacements = replacements
                .iter()
                .map(|replacement| SymbolString {
                    symbols: replacement
                        .iter()
                        .map(|s| self.find_symbol(s).unwrap())
                        .collect(),
                })
                .collect();
            (pattern, replacements)
        })
    }

    pub fn valid_words_iter(&self, strategy: ValidWordIteratorStrategy) -> ValidWordIterator<'_> {
        ValidWordIterator::new(self, strategy)
    }
}
