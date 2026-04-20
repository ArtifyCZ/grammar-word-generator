use crate::non_terminal_symbol::NonTerminalSymbol;
use crate::sequence_form::SequenceForm;
use crate::symbol::Symbol;
use crate::symbol_string::SymbolString;
use crate::terminal_symbol::TerminalSymbol;
use std::collections::BTreeMap;

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

#[derive(Debug)]
pub struct ValidWord {
    pub word: Vec<TerminalSymbol>,
    pub path: Vec<Vec<String>>,
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

    pub fn rules<'grammar>(
        &'grammar self,
    ) -> impl Iterator<Item = (SequenceForm<'grammar>, Vec<SymbolString<'grammar>>)> {
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

    pub fn generate_valid_words(&self, limit: usize) -> Vec<ValidWord> {
        let start_string = SymbolString::new_single(Symbol::NonTerminal(&self.start_symbol));
        let mut remaining_till_limit = limit;
        find_all_valid_words(self, start_string, vec![], &mut remaining_till_limit, 0)
    }
}

fn find_all_valid_words<'grammar>(
    grammar: &'grammar Grammar,
    current_string: SymbolString<'grammar>,
    current_path: Vec<SequenceForm<'grammar>>,
    remaining_till_limit: &mut usize,
    recursion_depth: usize,
) -> Vec<ValidWord> {
    if *remaining_till_limit == 0 {
        return vec![];
    }
    if recursion_depth > 10 {
        return vec![];
    }
    if current_string.symbols().all(|s| s.is_terminal()) {
        *remaining_till_limit -= 1;
        return vec![ValidWord {
            word: current_string
                .symbols()
                .map(|s| match s {
                    Symbol::NonTerminal(_) => panic!("Expected terminal symbol"),
                    Symbol::Terminal(s) => TerminalSymbol {
                        value: s.value.clone(),
                    },
                })
                .collect(),
            path: current_path
                .into_iter()
                .map(|seq_form| seq_form.symbols().map(|s| s.as_str().to_string()).collect())
                .collect(),
        }];
    }
    let mut valid_words = Vec::new();
    for (rule_pattern, rule_replacements) in grammar.rules() {
        let pattern_symbols = rule_pattern.symbols().collect::<Vec<_>>();
        let mut new_path = current_path.clone();
        new_path.push(rule_pattern);
        for rule_replacement in rule_replacements {
            let Some(new_string) =
                current_string.replace_once(pattern_symbols.as_slice(), &rule_replacement)
            else {
                break;
            };
            let new_valid_words = find_all_valid_words(grammar, new_string, new_path.clone(), remaining_till_limit, recursion_depth + 1);
            valid_words.extend(new_valid_words);
            if *remaining_till_limit == 0 {
                return valid_words;
            }
        }
    }
    valid_words
}
