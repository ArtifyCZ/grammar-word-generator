use std::collections::BTreeMap;

#[test]
pub fn basic_grammar_word_generation_test() {
    use crate::grammar::Grammar;
    let non_terminals: Vec<String> = vec!["S", "A", "B", "X", "Y"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let terminals: Vec<String> = vec!["a", "b", "c", "slovo1", "slovo2"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut rules = BTreeMap::new();
    rules.insert(vec!["S".to_string()], vec![vec!["a".to_string(), "A".to_string(), "b".to_string()]]);
    rules.insert(vec!["a".to_string(), "A".to_string()], vec![vec!["X".to_string()]]);
    rules.insert(vec!["A".to_string(), "b".to_string()], vec![vec!["Y".to_string()]]);
    rules.insert(vec!["A".to_string()], vec![
        vec!["Y".to_string(), "A".to_string(), "b".to_string()],
        vec!["X".to_string(), "A".to_string(), "a".to_string()],
    ]);
    rules.insert(vec!["X".to_string()], vec![vec!["slovo1".to_string()]]);
    rules.insert(vec!["Y".to_string()], vec![vec!["slovo2".to_string()]]);

    let grammar = Grammar::new(
        non_terminals,
        terminals,
        rules,
        "S".to_string(),
    );
    let words = grammar.generate_valid_words(13);
    assert!(words.len() == 13);
    assert!(words.iter().any(|w| w.word.iter().map(|s| s.as_str().to_string()).collect::<Vec<String>>() == vec![
        "a".to_string(),
        "slovo2".to_string(),
        "slovo2".to_string(),
        "slovo2".to_string(),
        "slovo2".to_string(),
        "b".to_string(),
        "b".to_string(),
        "b".to_string(),
    ]));
}
