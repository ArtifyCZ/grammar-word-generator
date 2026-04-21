use crate::grammar::Grammar;
use crate::sequence_form::SequenceForm;
use crate::symbol::Symbol;
use crate::symbol_string::SymbolString;
use crate::valid_word::ValidWord;
use std::collections::VecDeque;

pub(super) struct BreadthFirstValidWordIterator<'grammar> {
    grammar: &'grammar Grammar,
    start_rule_idx: usize,
    rules: Vec<(SequenceForm<'grammar>, Vec<SymbolString<'grammar>>)>,
    // item = (rule_idx, replacement_idx) where rule_idx is the index of the currently processed rule
    // on the current level, and replacement_idx is the index of the currently processed replacement
    // of the currently processed rule.
    current_rules: VecDeque<(usize, usize)>,
    target_depth: usize,
}

impl<'grammar> BreadthFirstValidWordIterator<'grammar> {
    pub(super) fn new(grammar: &'grammar Grammar) -> Self {
        let rules = grammar.rules().collect::<Vec<_>>();
        let start_rule_idx = rules
            .iter()
            .enumerate()
            .find(|(_idx, (seq_form, _replacements))| {
                let mut symbols = seq_form.symbols();
                let first_symbol = symbols.next().unwrap();
                if symbols.next().is_some() {
                    false
                } else {
                    match first_symbol {
                        Symbol::NonTerminal(symbol) => &grammar.start_symbol == symbol,
                        Symbol::Terminal(_) => false,
                    }
                }
            })
            .map(|(idx, (_seq_form, _replacements))| idx)
            .unwrap();
        let mut current_rules = VecDeque::new();
        current_rules.push_back((start_rule_idx, 0));
        Self {
            grammar,
            start_rule_idx,
            rules,
            current_rules,
            target_depth: 0,
        }
    }

    fn next_indices(&self, rule_idx: usize, replacement_idx: usize) -> Option<(usize, usize)> {
        let (_rule_pattern, rule_replacements) = &self.rules[rule_idx];
        if replacement_idx + 1 < rule_replacements.len() {
            // Continue with the next replacement of the current rule on the current level of recursion
            return Some((rule_idx, replacement_idx + 1));
        };
        if rule_idx + 1 < self.rules.len() {
            // Continue with the next rule on the current level of recursion
            return Some((rule_idx + 1, 0));
        }

        // There are no remaining rules on the current level of recursion to try,
        // so we return to the parent level of recursion.
        None
    }

    fn move_to_next(&mut self, rule_idx: usize, replacement_idx: usize) {
        self.current_rules.push_back((rule_idx, replacement_idx));
        while let Some((rule_idx, replacement_idx)) = self.current_rules.pop_back() {
            if let Some((rule_idx, replacement_idx)) = self.next_indices(rule_idx, replacement_idx)
            {
                self.current_rules.push_back((rule_idx, replacement_idx));
                return;
            }
        }

        self.current_rules.push_back((self.start_rule_idx, 0));
        self.target_depth += 1;
    }
}

impl<'grammar> Iterator for BreadthFirstValidWordIterator<'grammar> {
    type Item = ValidWord<'grammar>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (rule_idx, replacement_idx) = self.current_rules.pop_back().unwrap();
            assert!(rule_idx < self.rules.len());
            let replacements_len = self.rules[rule_idx].1.len();
            assert!(replacement_idx < replacements_len);

            let Some(current_string) = self.current_rules.iter().try_fold(
                SymbolString::new_single(Symbol::NonTerminal(&self.grammar.start_symbol)),
                |current_string, (rule_idx, replacement_idx)| {
                    let (rule_pattern, rule_replacements) = &self.rules[*rule_idx];
                    let rule_pattern_symbols = rule_pattern.symbols().collect::<Vec<_>>();
                    let replacement = &rule_replacements[*replacement_idx];
                    current_string
                        .replace_once(rule_pattern_symbols.as_slice(), replacement)
                },
            ) else {
                self.move_to_next(rule_idx, replacements_len - 1);
                continue;
            };
            let (rule_pattern, rule_replacements) = &self.rules[rule_idx];
            let rule_pattern_symbols = rule_pattern.symbols().collect::<Vec<_>>();
            let replacement = &rule_replacements[replacement_idx];
            let Some(current_string) =
                current_string.replace_once(rule_pattern_symbols.as_slice(), replacement)
            else {
                self.move_to_next(rule_idx, replacements_len - 1);
                continue;
            };

            if self.current_rules.len() < self.target_depth {
                self.current_rules.push_back((rule_idx, replacement_idx));
                self.current_rules.push_back((0, 0));
                continue;
            }

            if self.current_rules.len() == self.target_depth
                && current_string.symbols().all(|s| s.is_terminal())
            {
                let symbols = current_string
                    .symbols()
                    .map(|s| match s {
                        Symbol::NonTerminal(_) => panic!("Expected terminal symbol"),
                        Symbol::Terminal(s) => s,
                    })
                    .collect();
                let path = self
                    .current_rules
                    .iter()
                    .chain(std::iter::once(&(rule_idx, replacement_idx)))
                    .map(|(rule_idx, _replacement_idx)| self.rules[*rule_idx].0.clone())
                    .collect();
                self.move_to_next(rule_idx, replacement_idx);
                return Some(ValidWord { symbols, path });
            }

            self.move_to_next(rule_idx, replacement_idx);
        }
    }
}
